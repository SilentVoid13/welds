use crate::Param;
use crate::Row;
use crate::errors::{Error, Result};
use std::sync::Arc;
use turso::params::Params as TursoParams;
use turso::{Connection, Value};

#[cfg(feature = "turso")]
mod client;
#[cfg(feature = "turso")]
pub use client::{TursoClient, TursoTransaction, connect};

#[cfg(feature = "turso-sync")]
mod client_sync;
#[cfg(feature = "turso-sync")]
pub use client_sync::{TursoSyncClient, TursoSyncTransaction, connect};
#[cfg(feature = "turso-sync")]
pub(crate) use client_sync::{run_execute, run_fetch_rows};

pub struct TursoOwnedRow {
    pub data: Vec<Value>,
    pub columns: Arc<Vec<String>>,
}

impl TursoOwnedRow {
    pub fn try_get<T>(&self, idx: usize) -> Result<T>
    where
        T: TryFromTursoValue,
    {
        let value = self
            .data
            .get(idx)
            .ok_or_else(|| Error::ColumnNotFound(idx.to_string()))?;
        T::try_from_turso(value)
    }
}

pub trait TursoParam {
    fn to_turso_value(&self) -> Value;
}

macro_rules! impl_param_int {
    ($($t:ty),*) => {
        $(
            impl TursoParam for $t {
                fn to_turso_value(&self) -> Value {
                    Value::Integer(*self as i64)
                }
            }
        )*
    };
}

impl_param_int!(i8, i16, i32, i64, isize, u8, u16, u32);

impl TursoParam for u64 {
    fn to_turso_value(&self) -> Value {
        Value::Integer((*self).min(i64::MAX as u64) as i64)
    }
}

impl TursoParam for usize {
    fn to_turso_value(&self) -> Value {
        Value::Integer((*self).min(i64::MAX as usize) as i64)
    }
}

impl TursoParam for f32 {
    fn to_turso_value(&self) -> Value {
        Value::Real(*self as f64)
    }
}

impl TursoParam for f64 {
    fn to_turso_value(&self) -> Value {
        Value::Real(*self)
    }
}

impl TursoParam for bool {
    fn to_turso_value(&self) -> Value {
        Value::Integer(if *self { 1 } else { 0 })
    }
}

impl TursoParam for String {
    fn to_turso_value(&self) -> Value {
        Value::Text(self.clone())
    }
}

impl TursoParam for &str {
    fn to_turso_value(&self) -> Value {
        Value::Text((*self).to_string())
    }
}

impl TursoParam for Vec<u8> {
    fn to_turso_value(&self) -> Value {
        Value::Blob(self.clone())
    }
}

impl<T> TursoParam for Option<T>
where
    T: TursoParam,
{
    fn to_turso_value(&self) -> Value {
        match self {
            Some(v) => v.to_turso_value(),
            None => Value::Null,
        }
    }
}

impl TursoParam for chrono::NaiveDateTime {
    fn to_turso_value(&self) -> Value {
        Value::Text(self.format("%F %T%.f").to_string())
    }
}

impl TursoParam for chrono::NaiveDate {
    fn to_turso_value(&self) -> Value {
        Value::Text(self.to_string())
    }
}

impl TursoParam for uuid::Uuid {
    fn to_turso_value(&self) -> Value {
        Value::Text(self.to_string())
    }
}

pub trait TryFromTursoValue: Sized {
    fn try_from_turso(v: &Value) -> Result<Self>;
}

fn type_mismatch<T>(expected: &str, got: &Value) -> Result<T> {
    let actual = match got {
        Value::Null => "NULL",
        Value::Integer(_) => "INTEGER",
        Value::Real(_) => "REAL",
        Value::Text(_) => "TEXT",
        Value::Blob(_) => "BLOB",
    };
    Err(Error::Turso(turso::Error::ConversionFailure(format!(
        "cannot convert {actual} to {expected}"
    ))))
}

macro_rules! impl_int_from_turso {
    ($($t:ty),*) => {
        $(
            impl TryFromTursoValue for $t {
                fn try_from_turso(v: &Value) -> Result<Self> {
                    match v {
                        Value::Integer(i) => (*i).try_into().map_err(|_| {
                            Error::Turso(turso::Error::ConversionFailure(format!(
                                "integer {} out of range for {}",
                                i, stringify!($t)
                            )))
                        }),
                        Value::Null => Err(Error::UnexpectedNoneInColumn(stringify!($t).to_string())),
                        other => type_mismatch(stringify!($t), other),
                    }
                }
            }
        )*
    };
}

impl_int_from_turso!(i8, i16, i32, i64, isize, u8, u16, u32, u64, usize);

impl TryFromTursoValue for f32 {
    fn try_from_turso(v: &Value) -> Result<Self> {
        match v {
            Value::Real(r) => Ok(*r as f32),
            Value::Integer(i) => Ok(*i as f32),
            Value::Null => Err(Error::UnexpectedNoneInColumn("f32".to_string())),
            other => type_mismatch("f32", other),
        }
    }
}

impl TryFromTursoValue for f64 {
    fn try_from_turso(v: &Value) -> Result<Self> {
        match v {
            Value::Real(r) => Ok(*r),
            Value::Integer(i) => Ok(*i as f64),
            Value::Null => Err(Error::UnexpectedNoneInColumn("f64".to_string())),
            other => type_mismatch("f64", other),
        }
    }
}

impl TryFromTursoValue for bool {
    fn try_from_turso(v: &Value) -> Result<Self> {
        match v {
            Value::Integer(i) => Ok(*i != 0),
            Value::Null => Err(Error::UnexpectedNoneInColumn("bool".to_string())),
            other => type_mismatch("bool", other),
        }
    }
}

impl TryFromTursoValue for String {
    fn try_from_turso(v: &Value) -> Result<Self> {
        match v {
            Value::Text(s) => Ok(s.clone()),
            Value::Null => Err(Error::UnexpectedNoneInColumn("String".to_string())),
            other => type_mismatch("String", other),
        }
    }
}

impl TryFromTursoValue for Vec<u8> {
    fn try_from_turso(v: &Value) -> Result<Self> {
        match v {
            Value::Blob(b) => Ok(b.clone()),
            Value::Text(s) => Ok(s.as_bytes().to_vec()),
            Value::Null => Err(Error::UnexpectedNoneInColumn("Vec<u8>".to_string())),
            other => type_mismatch("Vec<u8>", other),
        }
    }
}

impl<T> TryFromTursoValue for Option<T>
where
    T: TryFromTursoValue,
{
    fn try_from_turso(v: &Value) -> Result<Self> {
        match v {
            Value::Null => Ok(None),
            other => T::try_from_turso(other).map(Some),
        }
    }
}

impl TryFromTursoValue for chrono::NaiveDateTime {
    fn try_from_turso(v: &Value) -> Result<Self> {
        match v {
            Value::Text(s) => parse_naive_datetime(s),
            Value::Integer(secs) => chrono::DateTime::from_timestamp(*secs, 0)
                .map(|dt| dt.naive_utc())
                .ok_or_else(|| {
                    Error::Turso(turso::Error::ConversionFailure(format!(
                        "invalid unix timestamp {secs} for NaiveDateTime"
                    )))
                }),
            Value::Null => Err(Error::UnexpectedNoneInColumn("NaiveDateTime".to_string())),
            other => type_mismatch("NaiveDateTime", other),
        }
    }
}

impl TryFromTursoValue for chrono::NaiveDate {
    fn try_from_turso(v: &Value) -> Result<Self> {
        match v {
            Value::Text(s) => chrono::NaiveDate::parse_from_str(s, "%F").map_err(|e| {
                Error::Turso(turso::Error::ConversionFailure(format!(
                    "could not parse NaiveDate from {s:?}: {e}"
                )))
            }),
            Value::Null => Err(Error::UnexpectedNoneInColumn("NaiveDate".to_string())),
            other => type_mismatch("NaiveDate", other),
        }
    }
}

impl TryFromTursoValue for uuid::Uuid {
    fn try_from_turso(v: &Value) -> Result<Self> {
        match v {
            Value::Text(s) => uuid::Uuid::parse_str(s).map_err(|e| {
                Error::Turso(turso::Error::ConversionFailure(format!(
                    "could not parse Uuid from {s:?}: {e}"
                )))
            }),
            Value::Blob(b) if b.len() == 16 => Ok(uuid::Uuid::from_slice(b).unwrap()),
            Value::Null => Err(Error::UnexpectedNoneInColumn("Uuid".to_string())),
            other => type_mismatch("Uuid", other),
        }
    }
}

fn parse_naive_datetime(s: &str) -> Result<chrono::NaiveDateTime> {
    const FORMATS: &[&str] = &[
        "%F %T%.f", "%FT%T%.f", "%F %T", "%FT%T", "%F %H:%M", "%FT%H:%M",
    ];
    for fmt in FORMATS {
        if let Ok(dt) = chrono::NaiveDateTime::parse_from_str(s, fmt) {
            return Ok(dt);
        }
    }
    Err(Error::Turso(turso::Error::ConversionFailure(format!(
        "could not parse NaiveDateTime from {s:?}"
    ))))
}

pub(crate) fn build_params(params: &[&(dyn Param + Sync)]) -> TursoParams {
    let vals: Vec<Value> = params
        .iter()
        .map(|p| TursoParam::to_turso_value(*p))
        .collect();
    if vals.is_empty() {
        TursoParams::None
    } else {
        TursoParams::Positional(vals)
    }
}

pub(crate) async fn execute_on_conn(
    conn: &Connection,
    sql: &str,
    params: TursoParams,
) -> Result<crate::ExecuteResult> {
    let n = crate::trace::db_error(conn.execute(sql, params).await)?;
    Ok(crate::ExecuteResult::new(n))
}

pub(crate) async fn fetch_rows_on_conn(
    conn: &Connection,
    sql: &str,
    params: TursoParams,
) -> Result<Vec<Row>> {
    let mut rows = crate::trace::db_error(conn.query(sql, params).await)?;
    let columns = Arc::new(rows.column_names());
    let col_count = rows.column_count();
    let mut res = Vec::new();
    while let Some(row) = crate::trace::db_error(rows.next().await)? {
        let mut data = Vec::with_capacity(col_count);
        for i in 0..col_count {
            data.push(row.get_value(i)?);
        }
        res.push(Row::from(TursoOwnedRow {
            data,
            columns: Arc::clone(&columns),
        }));
    }
    Ok(res)
}
