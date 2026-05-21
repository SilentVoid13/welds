use super::{build_params, execute_on_conn, fetch_rows_on_conn};
use crate::ExecuteResult;
use crate::Param;
use crate::Row;
use crate::TransactStart;
use crate::errors::Result;
use crate::trace;
use crate::transaction::{TransT, Transaction};
use crate::{Client, Fetch};
use async_trait::async_trait;
use std::sync::Arc;
use tokio::sync::{Mutex, OwnedMutexGuard};
use turso::{Builder, Connection, Database};

#[derive(Clone)]
pub struct TursoClient {
    conn: Arc<Mutex<Connection>>,
}

pub async fn connect(url: &str) -> Result<TursoClient> {
    let mut path = url.trim_start_matches("turso://");
    if path.is_empty() || path == ":memory:" {
        path = ":memory:";
    }
    let db = Builder::new_local(path).build().await?;
    TursoClient::from_database(db)
}

impl TursoClient {
    pub fn from_database(db: Database) -> Result<TursoClient> {
        Ok(TursoClient::from_connection(db.connect()?))
    }

    pub fn from_connection(conn: Connection) -> TursoClient {
        TursoClient {
            conn: Arc::new(Mutex::new(conn)),
        }
    }

    pub async fn execute_batch(&self, sql: &str) -> Result<()> {
        let conn = self.conn.lock().await;
        trace::db_error(conn.execute_batch(sql).await)?;
        Ok(())
    }
}

#[async_trait]
impl Client for TursoClient {
    async fn execute(&self, sql: &str, params: &[&(dyn Param + Sync)]) -> Result<ExecuteResult> {
        log::trace!("TURSO EXECUTE: {}", sql);
        let conn = self.conn.lock().await;
        execute_on_conn(&conn, sql, build_params(params)).await
    }

    async fn fetch_rows(&self, sql: &str, params: &[&(dyn Param + Sync)]) -> Result<Vec<Row>> {
        log::trace!("TURSO FETCH_ROWS: {}", sql);
        let conn = self.conn.lock().await;
        fetch_rows_on_conn(&conn, sql, build_params(params)).await
    }

    async fn fetch_many<'s, 'args, 't>(
        &self,
        fetches: &[Fetch<'s, 'args, 't>],
    ) -> Result<Vec<Vec<Row>>> {
        let conn = self.conn.lock().await;
        let mut datasets = Vec::with_capacity(fetches.len());
        for fetch in fetches {
            log::trace!("TURSO FETCH_MANY: {}", fetch.sql);
            let rows = fetch_rows_on_conn(&conn, fetch.sql, build_params(fetch.params)).await?;
            datasets.push(rows);
        }
        Ok(datasets)
    }

    fn syntax(&self) -> crate::Syntax {
        crate::Syntax::Sqlite
    }
}

pub struct TursoTransaction<'a> {
    // Field order: `transaction` drops first, while `guard` still holds the
    // lock, so its Drop can flag a dangling rollback on the connection.
    transaction: Option<turso::transaction::Transaction<'static>>,
    pub(crate) guard: OwnedMutexGuard<Connection>,
    _phantom: std::marker::PhantomData<&'a ()>,
}

impl TursoTransaction<'_> {
    pub async fn commit(mut self) -> Result<()> {
        if let Some(tx) = self.transaction.take() {
            trace::db_error(tx.commit().await)?;
        }
        Ok(())
    }

    pub async fn rollback(mut self) -> Result<()> {
        if let Some(tx) = self.transaction.take() {
            trace::db_error(tx.rollback().await)?;
        }
        Ok(())
    }
}

#[async_trait]
impl TransactStart for TursoClient {
    async fn begin<'t>(&'t self) -> Result<Transaction<'t>> {
        let guard = self.conn.clone().lock_owned().await;
        let tx = trace::db_error(guard.unchecked_transaction().await)?;
        // SAFETY: `tx` borrows the Connection inside `guard`. We store both in
        // the same struct and field order ensures `tx` drops before the lock
        // releases. The Connection's heap address is stable across moves.
        let tx: turso::transaction::Transaction<'static> = unsafe { std::mem::transmute(tx) };
        let t = TursoTransaction {
            transaction: Some(tx),
            guard,
            _phantom: std::marker::PhantomData,
        };
        Ok(Transaction::new(TransT::Turso(t)))
    }
}
