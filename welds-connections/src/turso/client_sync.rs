use super::{build_params, execute_on_conn, fetch_rows_on_conn};
use crate::ExecuteResult;
use crate::Param;
use crate::Row;
use crate::TransactStart;
use crate::errors::Result;
use crate::trace;
use crate::transaction::{TransT, Transaction};
use crate::{Client, Fetch};
use std::future::Future;
use std::sync::mpsc;
use std::sync::{Arc, Mutex, MutexGuard};
use tokio::runtime::{Handle, Runtime};
use turso::{Builder, Connection, Database};

#[derive(Clone)]
pub struct TursoSyncClient {
    conn: Arc<Mutex<Connection>>,
    handle: Handle,
    _runtime: Arc<RuntimeHolder>,
}

// shutdown_background() lets the worker threads wind down on their own so
// dropping from inside another tokio runtime doesn't panic.
struct RuntimeHolder {
    runtime: Option<Runtime>,
}

impl Drop for RuntimeHolder {
    fn drop(&mut self) {
        if let Some(rt) = self.runtime.take() {
            rt.shutdown_background();
        }
    }
}

fn build_runtime() -> Result<Runtime> {
    tokio::runtime::Builder::new_multi_thread()
        .worker_threads(1)
        .enable_all()
        .thread_name("welds-turso-sync")
        .build()
        .map_err(|e| {
            crate::Error::Turso(turso::Error::ConversionFailure(format!(
                "could not start tokio runtime for turso-sync: {e}"
            )))
        })
}

pub fn connect(url: &str) -> Result<TursoSyncClient> {
    let mut path = url.trim_start_matches("turso://").to_string();
    if path.is_empty() || path == ":memory:" {
        path = ":memory:".to_string();
    }
    let runtime = build_runtime()?;
    let db = block_on(runtime.handle(), async move {
        Builder::new_local(&path).build().await
    })?;
    TursoSyncClient::wrap(db.connect()?, runtime)
}

impl TursoSyncClient {
    pub fn from_database(db: Database) -> Result<TursoSyncClient> {
        Self::from_connection(db.connect()?)
    }

    pub fn from_connection(conn: Connection) -> Result<TursoSyncClient> {
        Self::wrap(conn, build_runtime()?)
    }

    fn wrap(conn: Connection, runtime: Runtime) -> Result<TursoSyncClient> {
        let handle = runtime.handle().clone();
        Ok(TursoSyncClient {
            conn: Arc::new(Mutex::new(conn)),
            handle,
            _runtime: Arc::new(RuntimeHolder {
                runtime: Some(runtime),
            }),
        })
    }

    pub fn execute_batch(&self, sql: &str) -> Result<()> {
        let guard = self.conn.lock().unwrap();
        let conn = unsafe { extend_lifetime(&guard) };
        let sql = sql.to_string();
        trace::db_error(block_on(&self.handle, async move {
            conn.execute_batch(&sql).await
        }))?;
        Ok(())
    }
}

impl Client for TursoSyncClient {
    fn execute(&self, sql: &str, params: &[&(dyn Param + Sync)]) -> Result<ExecuteResult> {
        log::trace!("TURSO-SYNC EXECUTE: {}", sql);
        let guard = self.conn.lock().unwrap();
        run_execute(&self.handle, &guard, sql, params)
    }

    fn fetch_rows(&self, sql: &str, params: &[&(dyn Param + Sync)]) -> Result<Vec<Row>> {
        log::trace!("TURSO-SYNC FETCH_ROWS: {}", sql);
        let guard = self.conn.lock().unwrap();
        run_fetch_rows(&self.handle, &guard, sql, params)
    }

    fn fetch_many<'s, 'args, 't>(&self, fetches: &[Fetch<'s, 'args, 't>]) -> Result<Vec<Vec<Row>>> {
        let guard = self.conn.lock().unwrap();
        let mut datasets = Vec::with_capacity(fetches.len());
        for fetch in fetches {
            log::trace!("TURSO-SYNC FETCH_MANY: {}", fetch.sql);
            datasets.push(run_fetch_rows(
                &self.handle,
                &guard,
                fetch.sql,
                fetch.params,
            )?);
        }
        Ok(datasets)
    }

    fn syntax(&self) -> crate::Syntax {
        crate::Syntax::Sqlite
    }
}

pub struct TursoSyncTransaction<'a> {
    // Field order: `transaction` drops first, while `guard` still holds the
    // lock, so its Drop can flag a dangling rollback on the connection.
    transaction: Option<turso::transaction::Transaction<'static>>,
    pub(crate) guard: MutexGuard<'a, Connection>,
    pub(crate) handle: Handle,
}

impl TursoSyncTransaction<'_> {
    pub fn commit(mut self) -> Result<()> {
        if let Some(tx) = self.transaction.take() {
            trace::db_error(block_on(&self.handle, async move { tx.commit().await }))?;
        }
        Ok(())
    }

    pub fn rollback(mut self) -> Result<()> {
        if let Some(tx) = self.transaction.take() {
            trace::db_error(block_on(&self.handle, async move { tx.rollback().await }))?;
        }
        Ok(())
    }
}

impl TransactStart for TursoSyncClient {
    fn begin<'t>(&'t self) -> Result<Transaction<'t>> {
        let guard = self.conn.lock().unwrap();
        let conn = unsafe { extend_lifetime(&guard) };
        let tx = trace::db_error(block_on(&self.handle, async move {
            conn.unchecked_transaction().await
        }))?;
        let t = TursoSyncTransaction {
            transaction: Some(tx),
            guard,
            handle: self.handle.clone(),
        };
        Ok(Transaction::new(TransT::TursoSync(t)))
    }
}

pub(crate) fn run_execute(
    handle: &Handle,
    guard: &MutexGuard<Connection>,
    sql: &str,
    params: &[&(dyn Param + Sync)],
) -> Result<ExecuteResult> {
    let conn = unsafe { extend_lifetime(guard) };
    let sql = sql.to_string();
    let p = build_params(params);
    block_on(handle, async move { execute_on_conn(conn, &sql, p).await })
}

pub(crate) fn run_fetch_rows(
    handle: &Handle,
    guard: &MutexGuard<Connection>,
    sql: &str,
    params: &[&(dyn Param + Sync)],
) -> Result<Vec<Row>> {
    let conn = unsafe { extend_lifetime(guard) };
    let sql = sql.to_string();
    let p = build_params(params);
    block_on(
        handle,
        async move { fetch_rows_on_conn(conn, &sql, p).await },
    )
}

/// Spawns `fut` onto the worker runtime and blocks the calling thread until it
/// resolves. Lets us drive async turso calls from sync code even when the
/// caller is already inside another tokio runtime.
pub(crate) fn block_on<F, T>(handle: &Handle, fut: F) -> T
where
    F: Future<Output = T> + Send + 'static,
    T: Send + 'static,
{
    let (tx, rx) = mpsc::sync_channel(1);
    handle.spawn(async move {
        let _ = tx.send(fut.await);
    });
    rx.recv()
        .expect("turso-sync worker runtime terminated unexpectedly")
}

/// SAFETY: the returned `&'static Connection` must not outlive `guard`. Every
/// caller holds `guard` for the duration of [`block_on`], which completes
/// synchronously before the guard is released.
pub(crate) unsafe fn extend_lifetime<'a>(
    guard: &'a MutexGuard<'a, Connection>,
) -> &'static Connection {
    let conn: &Connection = guard;
    unsafe { std::mem::transmute::<&Connection, &'static Connection>(conn) }
}
