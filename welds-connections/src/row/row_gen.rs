
use super::*;

#[cfg(any(feature = "sqlite", feature = "postgres", feature = "mysql"))]
use sqlx::Row as SqlxRow;


#[cfg(all(feature = "sqlite-sync", not(feature = "turso-sync")))]
impl Row {

    /// gets the value for a column in the row by its name.
    /// Errors:
    ///  * if column missing
    ///  * if column could not be deserialized into requested type <T>
    pub fn get<T>(&self, name: &str) -> Result<T>
      where T: rusqlite::types::FromSql
    {
        match &self.inner {
            #[cfg(feature = "sqlite-sync")]
            RowInner::SqliteSync(r) => { let index = r.columns.iter().position(|c| c == name).ok_or(crate::Error::ColumnNotFound(name.to_string()))?; Ok(r.try_get(index)?) },
        }
    }

    /// gets the value for a column in the row by its index (position, zero based index).
    /// Errors:
    ///  * if column missing, out of bounds
    ///  * if column could not be deserialized into requested type <T>
    pub fn get_by_position<T>(&self, index: usize) -> Result<T>
      where T: rusqlite::types::FromSql
    {
        match &self.inner {
            #[cfg(feature = "sqlite-sync")]
            RowInner::SqliteSync(r) => Ok(r.try_get(index)?),
        }
    }


}


#[cfg(all(feature = "turso-sync", not(feature = "sqlite-sync")))]
impl Row {

    /// gets the value for a column in the row by its name.
    /// Errors:
    ///  * if column missing
    ///  * if column could not be deserialized into requested type <T>
    pub fn get<T>(&self, name: &str) -> Result<T>
      where T: crate::turso::TryFromTursoValue
    {
        match &self.inner {
            #[cfg(any(feature = "turso", feature = "turso-sync"))]
            RowInner::Turso(r) => { let index = r.columns.iter().position(|c| c == name).ok_or_else(|| crate::Error::ColumnNotFound(name.to_string()))?; r.try_get(index) },
        }
    }

    /// gets the value for a column in the row by its index (position, zero based index).
    /// Errors:
    ///  * if column missing, out of bounds
    ///  * if column could not be deserialized into requested type <T>
    pub fn get_by_position<T>(&self, index: usize) -> Result<T>
      where T: crate::turso::TryFromTursoValue
    {
        match &self.inner {
            #[cfg(any(feature = "turso", feature = "turso-sync"))]
            RowInner::Turso(r) => r.try_get(index),
        }
    }


}


#[cfg(all(feature = "sqlite-sync", feature = "turso-sync"))]
impl Row {

    /// gets the value for a column in the row by its name.
    /// Errors:
    ///  * if column missing
    ///  * if column could not be deserialized into requested type <T>
    pub fn get<T>(&self, name: &str) -> Result<T>
      where T: rusqlite::types::FromSql + crate::turso::TryFromTursoValue
    {
        match &self.inner {
            #[cfg(feature = "sqlite-sync")]
            RowInner::SqliteSync(r) => { let index = r.columns.iter().position(|c| c == name).ok_or(crate::Error::ColumnNotFound(name.to_string()))?; Ok(r.try_get(index)?) },
            #[cfg(any(feature = "turso", feature = "turso-sync"))]
            RowInner::Turso(r) => { let index = r.columns.iter().position(|c| c == name).ok_or_else(|| crate::Error::ColumnNotFound(name.to_string()))?; r.try_get(index) },
        }
    }

    /// gets the value for a column in the row by its index (position, zero based index).
    /// Errors:
    ///  * if column missing, out of bounds
    ///  * if column could not be deserialized into requested type <T>
    pub fn get_by_position<T>(&self, index: usize) -> Result<T>
      where T: rusqlite::types::FromSql + crate::turso::TryFromTursoValue
    {
        match &self.inner {
            #[cfg(feature = "sqlite-sync")]
            RowInner::SqliteSync(r) => Ok(r.try_get(index)?),
            #[cfg(any(feature = "turso", feature = "turso-sync"))]
            RowInner::Turso(r) => r.try_get(index),
        }
    }


}




#[cfg(all(feature = "sqlite", not(feature = "postgres"), not(feature = "mysql"), not(feature = "mssql"), not(feature = "turso")))]
impl Row {

    /// gets the value for a column in the row by its name. 
    /// Errors: 
    ///  * if column missing
    ///  * if column could not be deserialized into requested type <T>
    pub fn get<T>(&self, name: &str) -> Result<T>
      where T: for<'r> Decode<'r, sqlx::Sqlite> + Type<sqlx::Sqlite>
    {
        match &self.inner {
            #[cfg(feature = "sqlite")]
            RowInner::Sqlite(r) => Ok(r.try_get(name)?),
            #[cfg(feature = "mssql")]
            RowInner::Mssql(r) => r.try_get(name),
            #[cfg(feature = "postgres")]
            RowInner::Postgres(r) => Ok(r.try_get(name)?),
            #[cfg(feature = "mysql")]
            RowInner::Mysql(r) => Ok(r.try_get(name)?),
            #[cfg(any(feature = "turso", feature = "turso-sync"))]
            RowInner::Turso(r) => { let index = r.columns.iter().position(|c| c == name).ok_or_else(|| crate::Error::ColumnNotFound(name.to_string()))?; r.try_get(index) },
        }
    }

    /// gets the value for a column in the row by its index (position, zero based index). 
    /// Errors: 
    ///  * if column missing, out of bounds
    ///  * if column could not be deserialized into requested type <T>
    pub fn get_by_position<T>(&self, index: usize) -> Result<T>
      where T: for<'r> Decode<'r, sqlx::Sqlite> + Type<sqlx::Sqlite>
    {
        match &self.inner {
            #[cfg(feature = "sqlite")]
            RowInner::Sqlite(r) => Ok(r.try_get(index)?),
            #[cfg(feature = "mssql")]
            RowInner::Mssql(r) => r.try_get_by_posision(index),
            #[cfg(feature = "postgres")]
            RowInner::Postgres(r) => Ok(r.try_get(index)?),
            #[cfg(feature = "mysql")]
            RowInner::Mysql(r) => Ok(r.try_get(index)?),
            #[cfg(any(feature = "turso", feature = "turso-sync"))]
            RowInner::Turso(r) => r.try_get(index),
        }
    }


}





#[cfg(all(feature = "postgres", not(feature = "sqlite"), not(feature = "mysql"), not(feature = "mssql"), not(feature = "turso")))]
impl Row {

    /// gets the value for a column in the row by its name. 
    /// Errors: 
    ///  * if column missing
    ///  * if column could not be deserialized into requested type <T>
    pub fn get<T>(&self, name: &str) -> Result<T>
      where T: for<'r> Decode<'r, sqlx::Postgres> + Type<sqlx::Postgres>
    {
        match &self.inner {
            #[cfg(feature = "sqlite")]
            RowInner::Sqlite(r) => Ok(r.try_get(name)?),
            #[cfg(feature = "mssql")]
            RowInner::Mssql(r) => r.try_get(name),
            #[cfg(feature = "postgres")]
            RowInner::Postgres(r) => Ok(r.try_get(name)?),
            #[cfg(feature = "mysql")]
            RowInner::Mysql(r) => Ok(r.try_get(name)?),
            #[cfg(any(feature = "turso", feature = "turso-sync"))]
            RowInner::Turso(r) => { let index = r.columns.iter().position(|c| c == name).ok_or_else(|| crate::Error::ColumnNotFound(name.to_string()))?; r.try_get(index) },
        }
    }

    /// gets the value for a column in the row by its index (position, zero based index). 
    /// Errors: 
    ///  * if column missing, out of bounds
    ///  * if column could not be deserialized into requested type <T>
    pub fn get_by_position<T>(&self, index: usize) -> Result<T>
      where T: for<'r> Decode<'r, sqlx::Postgres> + Type<sqlx::Postgres>
    {
        match &self.inner {
            #[cfg(feature = "sqlite")]
            RowInner::Sqlite(r) => Ok(r.try_get(index)?),
            #[cfg(feature = "mssql")]
            RowInner::Mssql(r) => r.try_get_by_posision(index),
            #[cfg(feature = "postgres")]
            RowInner::Postgres(r) => Ok(r.try_get(index)?),
            #[cfg(feature = "mysql")]
            RowInner::Mysql(r) => Ok(r.try_get(index)?),
            #[cfg(any(feature = "turso", feature = "turso-sync"))]
            RowInner::Turso(r) => r.try_get(index),
        }
    }


}





#[cfg(all(feature = "mysql", not(feature = "sqlite"), not(feature = "postgres"), not(feature = "mssql"), not(feature = "turso")))]
impl Row {

    /// gets the value for a column in the row by its name. 
    /// Errors: 
    ///  * if column missing
    ///  * if column could not be deserialized into requested type <T>
    pub fn get<T>(&self, name: &str) -> Result<T>
      where T: for<'r> Decode<'r, sqlx::MySql> + Type<sqlx::MySql>
    {
        match &self.inner {
            #[cfg(feature = "sqlite")]
            RowInner::Sqlite(r) => Ok(r.try_get(name)?),
            #[cfg(feature = "mssql")]
            RowInner::Mssql(r) => r.try_get(name),
            #[cfg(feature = "postgres")]
            RowInner::Postgres(r) => Ok(r.try_get(name)?),
            #[cfg(feature = "mysql")]
            RowInner::Mysql(r) => Ok(r.try_get(name)?),
            #[cfg(any(feature = "turso", feature = "turso-sync"))]
            RowInner::Turso(r) => { let index = r.columns.iter().position(|c| c == name).ok_or_else(|| crate::Error::ColumnNotFound(name.to_string()))?; r.try_get(index) },
        }
    }

    /// gets the value for a column in the row by its index (position, zero based index). 
    /// Errors: 
    ///  * if column missing, out of bounds
    ///  * if column could not be deserialized into requested type <T>
    pub fn get_by_position<T>(&self, index: usize) -> Result<T>
      where T: for<'r> Decode<'r, sqlx::MySql> + Type<sqlx::MySql>
    {
        match &self.inner {
            #[cfg(feature = "sqlite")]
            RowInner::Sqlite(r) => Ok(r.try_get(index)?),
            #[cfg(feature = "mssql")]
            RowInner::Mssql(r) => r.try_get_by_posision(index),
            #[cfg(feature = "postgres")]
            RowInner::Postgres(r) => Ok(r.try_get(index)?),
            #[cfg(feature = "mysql")]
            RowInner::Mysql(r) => Ok(r.try_get(index)?),
            #[cfg(any(feature = "turso", feature = "turso-sync"))]
            RowInner::Turso(r) => r.try_get(index),
        }
    }


}





#[cfg(all(feature = "mssql", not(feature = "sqlite"), not(feature = "postgres"), not(feature = "mysql"), not(feature = "turso")))]
impl Row {

    /// gets the value for a column in the row by its name. 
    /// Errors: 
    ///  * if column missing
    ///  * if column could not be deserialized into requested type <T>
    pub fn get<T>(&self, name: &str) -> Result<T>
      where T: TiberiusDecode
    {
        match &self.inner {
            #[cfg(feature = "sqlite")]
            RowInner::Sqlite(r) => Ok(r.try_get(name)?),
            #[cfg(feature = "mssql")]
            RowInner::Mssql(r) => r.try_get(name),
            #[cfg(feature = "postgres")]
            RowInner::Postgres(r) => Ok(r.try_get(name)?),
            #[cfg(feature = "mysql")]
            RowInner::Mysql(r) => Ok(r.try_get(name)?),
            #[cfg(any(feature = "turso", feature = "turso-sync"))]
            RowInner::Turso(r) => { let index = r.columns.iter().position(|c| c == name).ok_or_else(|| crate::Error::ColumnNotFound(name.to_string()))?; r.try_get(index) },
        }
    }

    /// gets the value for a column in the row by its index (position, zero based index). 
    /// Errors: 
    ///  * if column missing, out of bounds
    ///  * if column could not be deserialized into requested type <T>
    pub fn get_by_position<T>(&self, index: usize) -> Result<T>
      where T: TiberiusDecode
    {
        match &self.inner {
            #[cfg(feature = "sqlite")]
            RowInner::Sqlite(r) => Ok(r.try_get(index)?),
            #[cfg(feature = "mssql")]
            RowInner::Mssql(r) => r.try_get_by_posision(index),
            #[cfg(feature = "postgres")]
            RowInner::Postgres(r) => Ok(r.try_get(index)?),
            #[cfg(feature = "mysql")]
            RowInner::Mysql(r) => Ok(r.try_get(index)?),
            #[cfg(any(feature = "turso", feature = "turso-sync"))]
            RowInner::Turso(r) => r.try_get(index),
        }
    }


}





#[cfg(all(feature = "turso", not(feature = "sqlite"), not(feature = "postgres"), not(feature = "mysql"), not(feature = "mssql")))]
impl Row {

    /// gets the value for a column in the row by its name. 
    /// Errors: 
    ///  * if column missing
    ///  * if column could not be deserialized into requested type <T>
    pub fn get<T>(&self, name: &str) -> Result<T>
      where T: crate::turso::TryFromTursoValue
    {
        match &self.inner {
            #[cfg(feature = "sqlite")]
            RowInner::Sqlite(r) => Ok(r.try_get(name)?),
            #[cfg(feature = "mssql")]
            RowInner::Mssql(r) => r.try_get(name),
            #[cfg(feature = "postgres")]
            RowInner::Postgres(r) => Ok(r.try_get(name)?),
            #[cfg(feature = "mysql")]
            RowInner::Mysql(r) => Ok(r.try_get(name)?),
            #[cfg(any(feature = "turso", feature = "turso-sync"))]
            RowInner::Turso(r) => { let index = r.columns.iter().position(|c| c == name).ok_or_else(|| crate::Error::ColumnNotFound(name.to_string()))?; r.try_get(index) },
        }
    }

    /// gets the value for a column in the row by its index (position, zero based index). 
    /// Errors: 
    ///  * if column missing, out of bounds
    ///  * if column could not be deserialized into requested type <T>
    pub fn get_by_position<T>(&self, index: usize) -> Result<T>
      where T: crate::turso::TryFromTursoValue
    {
        match &self.inner {
            #[cfg(feature = "sqlite")]
            RowInner::Sqlite(r) => Ok(r.try_get(index)?),
            #[cfg(feature = "mssql")]
            RowInner::Mssql(r) => r.try_get_by_posision(index),
            #[cfg(feature = "postgres")]
            RowInner::Postgres(r) => Ok(r.try_get(index)?),
            #[cfg(feature = "mysql")]
            RowInner::Mysql(r) => Ok(r.try_get(index)?),
            #[cfg(any(feature = "turso", feature = "turso-sync"))]
            RowInner::Turso(r) => r.try_get(index),
        }
    }


}





#[cfg(all(feature = "sqlite", feature = "postgres", not(feature = "mysql"), not(feature = "mssql"), not(feature = "turso")))]
impl Row {

    /// gets the value for a column in the row by its name. 
    /// Errors: 
    ///  * if column missing
    ///  * if column could not be deserialized into requested type <T>
    pub fn get<T>(&self, name: &str) -> Result<T>
      where T: for<'r> Decode<'r, sqlx::Sqlite> + Type<sqlx::Sqlite> + for<'r> Decode<'r, sqlx::Postgres> + Type<sqlx::Postgres>
    {
        match &self.inner {
            #[cfg(feature = "sqlite")]
            RowInner::Sqlite(r) => Ok(r.try_get(name)?),
            #[cfg(feature = "mssql")]
            RowInner::Mssql(r) => r.try_get(name),
            #[cfg(feature = "postgres")]
            RowInner::Postgres(r) => Ok(r.try_get(name)?),
            #[cfg(feature = "mysql")]
            RowInner::Mysql(r) => Ok(r.try_get(name)?),
            #[cfg(any(feature = "turso", feature = "turso-sync"))]
            RowInner::Turso(r) => { let index = r.columns.iter().position(|c| c == name).ok_or_else(|| crate::Error::ColumnNotFound(name.to_string()))?; r.try_get(index) },
        }
    }

    /// gets the value for a column in the row by its index (position, zero based index). 
    /// Errors: 
    ///  * if column missing, out of bounds
    ///  * if column could not be deserialized into requested type <T>
    pub fn get_by_position<T>(&self, index: usize) -> Result<T>
      where T: for<'r> Decode<'r, sqlx::Sqlite> + Type<sqlx::Sqlite> + for<'r> Decode<'r, sqlx::Postgres> + Type<sqlx::Postgres>
    {
        match &self.inner {
            #[cfg(feature = "sqlite")]
            RowInner::Sqlite(r) => Ok(r.try_get(index)?),
            #[cfg(feature = "mssql")]
            RowInner::Mssql(r) => r.try_get_by_posision(index),
            #[cfg(feature = "postgres")]
            RowInner::Postgres(r) => Ok(r.try_get(index)?),
            #[cfg(feature = "mysql")]
            RowInner::Mysql(r) => Ok(r.try_get(index)?),
            #[cfg(any(feature = "turso", feature = "turso-sync"))]
            RowInner::Turso(r) => r.try_get(index),
        }
    }


}





#[cfg(all(feature = "sqlite", feature = "mysql", not(feature = "postgres"), not(feature = "mssql"), not(feature = "turso")))]
impl Row {

    /// gets the value for a column in the row by its name. 
    /// Errors: 
    ///  * if column missing
    ///  * if column could not be deserialized into requested type <T>
    pub fn get<T>(&self, name: &str) -> Result<T>
      where T: for<'r> Decode<'r, sqlx::Sqlite> + Type<sqlx::Sqlite> + for<'r> Decode<'r, sqlx::MySql> + Type<sqlx::MySql>
    {
        match &self.inner {
            #[cfg(feature = "sqlite")]
            RowInner::Sqlite(r) => Ok(r.try_get(name)?),
            #[cfg(feature = "mssql")]
            RowInner::Mssql(r) => r.try_get(name),
            #[cfg(feature = "postgres")]
            RowInner::Postgres(r) => Ok(r.try_get(name)?),
            #[cfg(feature = "mysql")]
            RowInner::Mysql(r) => Ok(r.try_get(name)?),
            #[cfg(any(feature = "turso", feature = "turso-sync"))]
            RowInner::Turso(r) => { let index = r.columns.iter().position(|c| c == name).ok_or_else(|| crate::Error::ColumnNotFound(name.to_string()))?; r.try_get(index) },
        }
    }

    /// gets the value for a column in the row by its index (position, zero based index). 
    /// Errors: 
    ///  * if column missing, out of bounds
    ///  * if column could not be deserialized into requested type <T>
    pub fn get_by_position<T>(&self, index: usize) -> Result<T>
      where T: for<'r> Decode<'r, sqlx::Sqlite> + Type<sqlx::Sqlite> + for<'r> Decode<'r, sqlx::MySql> + Type<sqlx::MySql>
    {
        match &self.inner {
            #[cfg(feature = "sqlite")]
            RowInner::Sqlite(r) => Ok(r.try_get(index)?),
            #[cfg(feature = "mssql")]
            RowInner::Mssql(r) => r.try_get_by_posision(index),
            #[cfg(feature = "postgres")]
            RowInner::Postgres(r) => Ok(r.try_get(index)?),
            #[cfg(feature = "mysql")]
            RowInner::Mysql(r) => Ok(r.try_get(index)?),
            #[cfg(any(feature = "turso", feature = "turso-sync"))]
            RowInner::Turso(r) => r.try_get(index),
        }
    }


}





#[cfg(all(feature = "sqlite", feature = "mssql", not(feature = "postgres"), not(feature = "mysql"), not(feature = "turso")))]
impl Row {

    /// gets the value for a column in the row by its name. 
    /// Errors: 
    ///  * if column missing
    ///  * if column could not be deserialized into requested type <T>
    pub fn get<T>(&self, name: &str) -> Result<T>
      where T: for<'r> Decode<'r, sqlx::Sqlite> + Type<sqlx::Sqlite> + TiberiusDecode
    {
        match &self.inner {
            #[cfg(feature = "sqlite")]
            RowInner::Sqlite(r) => Ok(r.try_get(name)?),
            #[cfg(feature = "mssql")]
            RowInner::Mssql(r) => r.try_get(name),
            #[cfg(feature = "postgres")]
            RowInner::Postgres(r) => Ok(r.try_get(name)?),
            #[cfg(feature = "mysql")]
            RowInner::Mysql(r) => Ok(r.try_get(name)?),
            #[cfg(any(feature = "turso", feature = "turso-sync"))]
            RowInner::Turso(r) => { let index = r.columns.iter().position(|c| c == name).ok_or_else(|| crate::Error::ColumnNotFound(name.to_string()))?; r.try_get(index) },
        }
    }

    /// gets the value for a column in the row by its index (position, zero based index). 
    /// Errors: 
    ///  * if column missing, out of bounds
    ///  * if column could not be deserialized into requested type <T>
    pub fn get_by_position<T>(&self, index: usize) -> Result<T>
      where T: for<'r> Decode<'r, sqlx::Sqlite> + Type<sqlx::Sqlite> + TiberiusDecode
    {
        match &self.inner {
            #[cfg(feature = "sqlite")]
            RowInner::Sqlite(r) => Ok(r.try_get(index)?),
            #[cfg(feature = "mssql")]
            RowInner::Mssql(r) => r.try_get_by_posision(index),
            #[cfg(feature = "postgres")]
            RowInner::Postgres(r) => Ok(r.try_get(index)?),
            #[cfg(feature = "mysql")]
            RowInner::Mysql(r) => Ok(r.try_get(index)?),
            #[cfg(any(feature = "turso", feature = "turso-sync"))]
            RowInner::Turso(r) => r.try_get(index),
        }
    }


}





#[cfg(all(feature = "sqlite", feature = "turso", not(feature = "postgres"), not(feature = "mysql"), not(feature = "mssql")))]
impl Row {

    /// gets the value for a column in the row by its name. 
    /// Errors: 
    ///  * if column missing
    ///  * if column could not be deserialized into requested type <T>
    pub fn get<T>(&self, name: &str) -> Result<T>
      where T: for<'r> Decode<'r, sqlx::Sqlite> + Type<sqlx::Sqlite> + crate::turso::TryFromTursoValue
    {
        match &self.inner {
            #[cfg(feature = "sqlite")]
            RowInner::Sqlite(r) => Ok(r.try_get(name)?),
            #[cfg(feature = "mssql")]
            RowInner::Mssql(r) => r.try_get(name),
            #[cfg(feature = "postgres")]
            RowInner::Postgres(r) => Ok(r.try_get(name)?),
            #[cfg(feature = "mysql")]
            RowInner::Mysql(r) => Ok(r.try_get(name)?),
            #[cfg(any(feature = "turso", feature = "turso-sync"))]
            RowInner::Turso(r) => { let index = r.columns.iter().position(|c| c == name).ok_or_else(|| crate::Error::ColumnNotFound(name.to_string()))?; r.try_get(index) },
        }
    }

    /// gets the value for a column in the row by its index (position, zero based index). 
    /// Errors: 
    ///  * if column missing, out of bounds
    ///  * if column could not be deserialized into requested type <T>
    pub fn get_by_position<T>(&self, index: usize) -> Result<T>
      where T: for<'r> Decode<'r, sqlx::Sqlite> + Type<sqlx::Sqlite> + crate::turso::TryFromTursoValue
    {
        match &self.inner {
            #[cfg(feature = "sqlite")]
            RowInner::Sqlite(r) => Ok(r.try_get(index)?),
            #[cfg(feature = "mssql")]
            RowInner::Mssql(r) => r.try_get_by_posision(index),
            #[cfg(feature = "postgres")]
            RowInner::Postgres(r) => Ok(r.try_get(index)?),
            #[cfg(feature = "mysql")]
            RowInner::Mysql(r) => Ok(r.try_get(index)?),
            #[cfg(any(feature = "turso", feature = "turso-sync"))]
            RowInner::Turso(r) => r.try_get(index),
        }
    }


}





#[cfg(all(feature = "postgres", feature = "mysql", not(feature = "sqlite"), not(feature = "mssql"), not(feature = "turso")))]
impl Row {

    /// gets the value for a column in the row by its name. 
    /// Errors: 
    ///  * if column missing
    ///  * if column could not be deserialized into requested type <T>
    pub fn get<T>(&self, name: &str) -> Result<T>
      where T: for<'r> Decode<'r, sqlx::Postgres> + Type<sqlx::Postgres> + for<'r> Decode<'r, sqlx::MySql> + Type<sqlx::MySql>
    {
        match &self.inner {
            #[cfg(feature = "sqlite")]
            RowInner::Sqlite(r) => Ok(r.try_get(name)?),
            #[cfg(feature = "mssql")]
            RowInner::Mssql(r) => r.try_get(name),
            #[cfg(feature = "postgres")]
            RowInner::Postgres(r) => Ok(r.try_get(name)?),
            #[cfg(feature = "mysql")]
            RowInner::Mysql(r) => Ok(r.try_get(name)?),
            #[cfg(any(feature = "turso", feature = "turso-sync"))]
            RowInner::Turso(r) => { let index = r.columns.iter().position(|c| c == name).ok_or_else(|| crate::Error::ColumnNotFound(name.to_string()))?; r.try_get(index) },
        }
    }

    /// gets the value for a column in the row by its index (position, zero based index). 
    /// Errors: 
    ///  * if column missing, out of bounds
    ///  * if column could not be deserialized into requested type <T>
    pub fn get_by_position<T>(&self, index: usize) -> Result<T>
      where T: for<'r> Decode<'r, sqlx::Postgres> + Type<sqlx::Postgres> + for<'r> Decode<'r, sqlx::MySql> + Type<sqlx::MySql>
    {
        match &self.inner {
            #[cfg(feature = "sqlite")]
            RowInner::Sqlite(r) => Ok(r.try_get(index)?),
            #[cfg(feature = "mssql")]
            RowInner::Mssql(r) => r.try_get_by_posision(index),
            #[cfg(feature = "postgres")]
            RowInner::Postgres(r) => Ok(r.try_get(index)?),
            #[cfg(feature = "mysql")]
            RowInner::Mysql(r) => Ok(r.try_get(index)?),
            #[cfg(any(feature = "turso", feature = "turso-sync"))]
            RowInner::Turso(r) => r.try_get(index),
        }
    }


}





#[cfg(all(feature = "postgres", feature = "mssql", not(feature = "sqlite"), not(feature = "mysql"), not(feature = "turso")))]
impl Row {

    /// gets the value for a column in the row by its name. 
    /// Errors: 
    ///  * if column missing
    ///  * if column could not be deserialized into requested type <T>
    pub fn get<T>(&self, name: &str) -> Result<T>
      where T: for<'r> Decode<'r, sqlx::Postgres> + Type<sqlx::Postgres> + TiberiusDecode
    {
        match &self.inner {
            #[cfg(feature = "sqlite")]
            RowInner::Sqlite(r) => Ok(r.try_get(name)?),
            #[cfg(feature = "mssql")]
            RowInner::Mssql(r) => r.try_get(name),
            #[cfg(feature = "postgres")]
            RowInner::Postgres(r) => Ok(r.try_get(name)?),
            #[cfg(feature = "mysql")]
            RowInner::Mysql(r) => Ok(r.try_get(name)?),
            #[cfg(any(feature = "turso", feature = "turso-sync"))]
            RowInner::Turso(r) => { let index = r.columns.iter().position(|c| c == name).ok_or_else(|| crate::Error::ColumnNotFound(name.to_string()))?; r.try_get(index) },
        }
    }

    /// gets the value for a column in the row by its index (position, zero based index). 
    /// Errors: 
    ///  * if column missing, out of bounds
    ///  * if column could not be deserialized into requested type <T>
    pub fn get_by_position<T>(&self, index: usize) -> Result<T>
      where T: for<'r> Decode<'r, sqlx::Postgres> + Type<sqlx::Postgres> + TiberiusDecode
    {
        match &self.inner {
            #[cfg(feature = "sqlite")]
            RowInner::Sqlite(r) => Ok(r.try_get(index)?),
            #[cfg(feature = "mssql")]
            RowInner::Mssql(r) => r.try_get_by_posision(index),
            #[cfg(feature = "postgres")]
            RowInner::Postgres(r) => Ok(r.try_get(index)?),
            #[cfg(feature = "mysql")]
            RowInner::Mysql(r) => Ok(r.try_get(index)?),
            #[cfg(any(feature = "turso", feature = "turso-sync"))]
            RowInner::Turso(r) => r.try_get(index),
        }
    }


}





#[cfg(all(feature = "postgres", feature = "turso", not(feature = "sqlite"), not(feature = "mysql"), not(feature = "mssql")))]
impl Row {

    /// gets the value for a column in the row by its name. 
    /// Errors: 
    ///  * if column missing
    ///  * if column could not be deserialized into requested type <T>
    pub fn get<T>(&self, name: &str) -> Result<T>
      where T: for<'r> Decode<'r, sqlx::Postgres> + Type<sqlx::Postgres> + crate::turso::TryFromTursoValue
    {
        match &self.inner {
            #[cfg(feature = "sqlite")]
            RowInner::Sqlite(r) => Ok(r.try_get(name)?),
            #[cfg(feature = "mssql")]
            RowInner::Mssql(r) => r.try_get(name),
            #[cfg(feature = "postgres")]
            RowInner::Postgres(r) => Ok(r.try_get(name)?),
            #[cfg(feature = "mysql")]
            RowInner::Mysql(r) => Ok(r.try_get(name)?),
            #[cfg(any(feature = "turso", feature = "turso-sync"))]
            RowInner::Turso(r) => { let index = r.columns.iter().position(|c| c == name).ok_or_else(|| crate::Error::ColumnNotFound(name.to_string()))?; r.try_get(index) },
        }
    }

    /// gets the value for a column in the row by its index (position, zero based index). 
    /// Errors: 
    ///  * if column missing, out of bounds
    ///  * if column could not be deserialized into requested type <T>
    pub fn get_by_position<T>(&self, index: usize) -> Result<T>
      where T: for<'r> Decode<'r, sqlx::Postgres> + Type<sqlx::Postgres> + crate::turso::TryFromTursoValue
    {
        match &self.inner {
            #[cfg(feature = "sqlite")]
            RowInner::Sqlite(r) => Ok(r.try_get(index)?),
            #[cfg(feature = "mssql")]
            RowInner::Mssql(r) => r.try_get_by_posision(index),
            #[cfg(feature = "postgres")]
            RowInner::Postgres(r) => Ok(r.try_get(index)?),
            #[cfg(feature = "mysql")]
            RowInner::Mysql(r) => Ok(r.try_get(index)?),
            #[cfg(any(feature = "turso", feature = "turso-sync"))]
            RowInner::Turso(r) => r.try_get(index),
        }
    }


}





#[cfg(all(feature = "mysql", feature = "mssql", not(feature = "sqlite"), not(feature = "postgres"), not(feature = "turso")))]
impl Row {

    /// gets the value for a column in the row by its name. 
    /// Errors: 
    ///  * if column missing
    ///  * if column could not be deserialized into requested type <T>
    pub fn get<T>(&self, name: &str) -> Result<T>
      where T: for<'r> Decode<'r, sqlx::MySql> + Type<sqlx::MySql> + TiberiusDecode
    {
        match &self.inner {
            #[cfg(feature = "sqlite")]
            RowInner::Sqlite(r) => Ok(r.try_get(name)?),
            #[cfg(feature = "mssql")]
            RowInner::Mssql(r) => r.try_get(name),
            #[cfg(feature = "postgres")]
            RowInner::Postgres(r) => Ok(r.try_get(name)?),
            #[cfg(feature = "mysql")]
            RowInner::Mysql(r) => Ok(r.try_get(name)?),
            #[cfg(any(feature = "turso", feature = "turso-sync"))]
            RowInner::Turso(r) => { let index = r.columns.iter().position(|c| c == name).ok_or_else(|| crate::Error::ColumnNotFound(name.to_string()))?; r.try_get(index) },
        }
    }

    /// gets the value for a column in the row by its index (position, zero based index). 
    /// Errors: 
    ///  * if column missing, out of bounds
    ///  * if column could not be deserialized into requested type <T>
    pub fn get_by_position<T>(&self, index: usize) -> Result<T>
      where T: for<'r> Decode<'r, sqlx::MySql> + Type<sqlx::MySql> + TiberiusDecode
    {
        match &self.inner {
            #[cfg(feature = "sqlite")]
            RowInner::Sqlite(r) => Ok(r.try_get(index)?),
            #[cfg(feature = "mssql")]
            RowInner::Mssql(r) => r.try_get_by_posision(index),
            #[cfg(feature = "postgres")]
            RowInner::Postgres(r) => Ok(r.try_get(index)?),
            #[cfg(feature = "mysql")]
            RowInner::Mysql(r) => Ok(r.try_get(index)?),
            #[cfg(any(feature = "turso", feature = "turso-sync"))]
            RowInner::Turso(r) => r.try_get(index),
        }
    }


}





#[cfg(all(feature = "mysql", feature = "turso", not(feature = "sqlite"), not(feature = "postgres"), not(feature = "mssql")))]
impl Row {

    /// gets the value for a column in the row by its name. 
    /// Errors: 
    ///  * if column missing
    ///  * if column could not be deserialized into requested type <T>
    pub fn get<T>(&self, name: &str) -> Result<T>
      where T: for<'r> Decode<'r, sqlx::MySql> + Type<sqlx::MySql> + crate::turso::TryFromTursoValue
    {
        match &self.inner {
            #[cfg(feature = "sqlite")]
            RowInner::Sqlite(r) => Ok(r.try_get(name)?),
            #[cfg(feature = "mssql")]
            RowInner::Mssql(r) => r.try_get(name),
            #[cfg(feature = "postgres")]
            RowInner::Postgres(r) => Ok(r.try_get(name)?),
            #[cfg(feature = "mysql")]
            RowInner::Mysql(r) => Ok(r.try_get(name)?),
            #[cfg(any(feature = "turso", feature = "turso-sync"))]
            RowInner::Turso(r) => { let index = r.columns.iter().position(|c| c == name).ok_or_else(|| crate::Error::ColumnNotFound(name.to_string()))?; r.try_get(index) },
        }
    }

    /// gets the value for a column in the row by its index (position, zero based index). 
    /// Errors: 
    ///  * if column missing, out of bounds
    ///  * if column could not be deserialized into requested type <T>
    pub fn get_by_position<T>(&self, index: usize) -> Result<T>
      where T: for<'r> Decode<'r, sqlx::MySql> + Type<sqlx::MySql> + crate::turso::TryFromTursoValue
    {
        match &self.inner {
            #[cfg(feature = "sqlite")]
            RowInner::Sqlite(r) => Ok(r.try_get(index)?),
            #[cfg(feature = "mssql")]
            RowInner::Mssql(r) => r.try_get_by_posision(index),
            #[cfg(feature = "postgres")]
            RowInner::Postgres(r) => Ok(r.try_get(index)?),
            #[cfg(feature = "mysql")]
            RowInner::Mysql(r) => Ok(r.try_get(index)?),
            #[cfg(any(feature = "turso", feature = "turso-sync"))]
            RowInner::Turso(r) => r.try_get(index),
        }
    }


}





#[cfg(all(feature = "mssql", feature = "turso", not(feature = "sqlite"), not(feature = "postgres"), not(feature = "mysql")))]
impl Row {

    /// gets the value for a column in the row by its name. 
    /// Errors: 
    ///  * if column missing
    ///  * if column could not be deserialized into requested type <T>
    pub fn get<T>(&self, name: &str) -> Result<T>
      where T: TiberiusDecode + crate::turso::TryFromTursoValue
    {
        match &self.inner {
            #[cfg(feature = "sqlite")]
            RowInner::Sqlite(r) => Ok(r.try_get(name)?),
            #[cfg(feature = "mssql")]
            RowInner::Mssql(r) => r.try_get(name),
            #[cfg(feature = "postgres")]
            RowInner::Postgres(r) => Ok(r.try_get(name)?),
            #[cfg(feature = "mysql")]
            RowInner::Mysql(r) => Ok(r.try_get(name)?),
            #[cfg(any(feature = "turso", feature = "turso-sync"))]
            RowInner::Turso(r) => { let index = r.columns.iter().position(|c| c == name).ok_or_else(|| crate::Error::ColumnNotFound(name.to_string()))?; r.try_get(index) },
        }
    }

    /// gets the value for a column in the row by its index (position, zero based index). 
    /// Errors: 
    ///  * if column missing, out of bounds
    ///  * if column could not be deserialized into requested type <T>
    pub fn get_by_position<T>(&self, index: usize) -> Result<T>
      where T: TiberiusDecode + crate::turso::TryFromTursoValue
    {
        match &self.inner {
            #[cfg(feature = "sqlite")]
            RowInner::Sqlite(r) => Ok(r.try_get(index)?),
            #[cfg(feature = "mssql")]
            RowInner::Mssql(r) => r.try_get_by_posision(index),
            #[cfg(feature = "postgres")]
            RowInner::Postgres(r) => Ok(r.try_get(index)?),
            #[cfg(feature = "mysql")]
            RowInner::Mysql(r) => Ok(r.try_get(index)?),
            #[cfg(any(feature = "turso", feature = "turso-sync"))]
            RowInner::Turso(r) => r.try_get(index),
        }
    }


}





#[cfg(all(feature = "sqlite", feature = "postgres", feature = "mysql", not(feature = "mssql"), not(feature = "turso")))]
impl Row {

    /// gets the value for a column in the row by its name. 
    /// Errors: 
    ///  * if column missing
    ///  * if column could not be deserialized into requested type <T>
    pub fn get<T>(&self, name: &str) -> Result<T>
      where T: for<'r> Decode<'r, sqlx::Sqlite> + Type<sqlx::Sqlite> + for<'r> Decode<'r, sqlx::Postgres> + Type<sqlx::Postgres> + for<'r> Decode<'r, sqlx::MySql> + Type<sqlx::MySql>
    {
        match &self.inner {
            #[cfg(feature = "sqlite")]
            RowInner::Sqlite(r) => Ok(r.try_get(name)?),
            #[cfg(feature = "mssql")]
            RowInner::Mssql(r) => r.try_get(name),
            #[cfg(feature = "postgres")]
            RowInner::Postgres(r) => Ok(r.try_get(name)?),
            #[cfg(feature = "mysql")]
            RowInner::Mysql(r) => Ok(r.try_get(name)?),
            #[cfg(any(feature = "turso", feature = "turso-sync"))]
            RowInner::Turso(r) => { let index = r.columns.iter().position(|c| c == name).ok_or_else(|| crate::Error::ColumnNotFound(name.to_string()))?; r.try_get(index) },
        }
    }

    /// gets the value for a column in the row by its index (position, zero based index). 
    /// Errors: 
    ///  * if column missing, out of bounds
    ///  * if column could not be deserialized into requested type <T>
    pub fn get_by_position<T>(&self, index: usize) -> Result<T>
      where T: for<'r> Decode<'r, sqlx::Sqlite> + Type<sqlx::Sqlite> + for<'r> Decode<'r, sqlx::Postgres> + Type<sqlx::Postgres> + for<'r> Decode<'r, sqlx::MySql> + Type<sqlx::MySql>
    {
        match &self.inner {
            #[cfg(feature = "sqlite")]
            RowInner::Sqlite(r) => Ok(r.try_get(index)?),
            #[cfg(feature = "mssql")]
            RowInner::Mssql(r) => r.try_get_by_posision(index),
            #[cfg(feature = "postgres")]
            RowInner::Postgres(r) => Ok(r.try_get(index)?),
            #[cfg(feature = "mysql")]
            RowInner::Mysql(r) => Ok(r.try_get(index)?),
            #[cfg(any(feature = "turso", feature = "turso-sync"))]
            RowInner::Turso(r) => r.try_get(index),
        }
    }


}





#[cfg(all(feature = "sqlite", feature = "postgres", feature = "mssql", not(feature = "mysql"), not(feature = "turso")))]
impl Row {

    /// gets the value for a column in the row by its name. 
    /// Errors: 
    ///  * if column missing
    ///  * if column could not be deserialized into requested type <T>
    pub fn get<T>(&self, name: &str) -> Result<T>
      where T: for<'r> Decode<'r, sqlx::Sqlite> + Type<sqlx::Sqlite> + for<'r> Decode<'r, sqlx::Postgres> + Type<sqlx::Postgres> + TiberiusDecode
    {
        match &self.inner {
            #[cfg(feature = "sqlite")]
            RowInner::Sqlite(r) => Ok(r.try_get(name)?),
            #[cfg(feature = "mssql")]
            RowInner::Mssql(r) => r.try_get(name),
            #[cfg(feature = "postgres")]
            RowInner::Postgres(r) => Ok(r.try_get(name)?),
            #[cfg(feature = "mysql")]
            RowInner::Mysql(r) => Ok(r.try_get(name)?),
            #[cfg(any(feature = "turso", feature = "turso-sync"))]
            RowInner::Turso(r) => { let index = r.columns.iter().position(|c| c == name).ok_or_else(|| crate::Error::ColumnNotFound(name.to_string()))?; r.try_get(index) },
        }
    }

    /// gets the value for a column in the row by its index (position, zero based index). 
    /// Errors: 
    ///  * if column missing, out of bounds
    ///  * if column could not be deserialized into requested type <T>
    pub fn get_by_position<T>(&self, index: usize) -> Result<T>
      where T: for<'r> Decode<'r, sqlx::Sqlite> + Type<sqlx::Sqlite> + for<'r> Decode<'r, sqlx::Postgres> + Type<sqlx::Postgres> + TiberiusDecode
    {
        match &self.inner {
            #[cfg(feature = "sqlite")]
            RowInner::Sqlite(r) => Ok(r.try_get(index)?),
            #[cfg(feature = "mssql")]
            RowInner::Mssql(r) => r.try_get_by_posision(index),
            #[cfg(feature = "postgres")]
            RowInner::Postgres(r) => Ok(r.try_get(index)?),
            #[cfg(feature = "mysql")]
            RowInner::Mysql(r) => Ok(r.try_get(index)?),
            #[cfg(any(feature = "turso", feature = "turso-sync"))]
            RowInner::Turso(r) => r.try_get(index),
        }
    }


}





#[cfg(all(feature = "sqlite", feature = "postgres", feature = "turso", not(feature = "mysql"), not(feature = "mssql")))]
impl Row {

    /// gets the value for a column in the row by its name. 
    /// Errors: 
    ///  * if column missing
    ///  * if column could not be deserialized into requested type <T>
    pub fn get<T>(&self, name: &str) -> Result<T>
      where T: for<'r> Decode<'r, sqlx::Sqlite> + Type<sqlx::Sqlite> + for<'r> Decode<'r, sqlx::Postgres> + Type<sqlx::Postgres> + crate::turso::TryFromTursoValue
    {
        match &self.inner {
            #[cfg(feature = "sqlite")]
            RowInner::Sqlite(r) => Ok(r.try_get(name)?),
            #[cfg(feature = "mssql")]
            RowInner::Mssql(r) => r.try_get(name),
            #[cfg(feature = "postgres")]
            RowInner::Postgres(r) => Ok(r.try_get(name)?),
            #[cfg(feature = "mysql")]
            RowInner::Mysql(r) => Ok(r.try_get(name)?),
            #[cfg(any(feature = "turso", feature = "turso-sync"))]
            RowInner::Turso(r) => { let index = r.columns.iter().position(|c| c == name).ok_or_else(|| crate::Error::ColumnNotFound(name.to_string()))?; r.try_get(index) },
        }
    }

    /// gets the value for a column in the row by its index (position, zero based index). 
    /// Errors: 
    ///  * if column missing, out of bounds
    ///  * if column could not be deserialized into requested type <T>
    pub fn get_by_position<T>(&self, index: usize) -> Result<T>
      where T: for<'r> Decode<'r, sqlx::Sqlite> + Type<sqlx::Sqlite> + for<'r> Decode<'r, sqlx::Postgres> + Type<sqlx::Postgres> + crate::turso::TryFromTursoValue
    {
        match &self.inner {
            #[cfg(feature = "sqlite")]
            RowInner::Sqlite(r) => Ok(r.try_get(index)?),
            #[cfg(feature = "mssql")]
            RowInner::Mssql(r) => r.try_get_by_posision(index),
            #[cfg(feature = "postgres")]
            RowInner::Postgres(r) => Ok(r.try_get(index)?),
            #[cfg(feature = "mysql")]
            RowInner::Mysql(r) => Ok(r.try_get(index)?),
            #[cfg(any(feature = "turso", feature = "turso-sync"))]
            RowInner::Turso(r) => r.try_get(index),
        }
    }


}





#[cfg(all(feature = "sqlite", feature = "mysql", feature = "mssql", not(feature = "postgres"), not(feature = "turso")))]
impl Row {

    /// gets the value for a column in the row by its name. 
    /// Errors: 
    ///  * if column missing
    ///  * if column could not be deserialized into requested type <T>
    pub fn get<T>(&self, name: &str) -> Result<T>
      where T: for<'r> Decode<'r, sqlx::Sqlite> + Type<sqlx::Sqlite> + for<'r> Decode<'r, sqlx::MySql> + Type<sqlx::MySql> + TiberiusDecode
    {
        match &self.inner {
            #[cfg(feature = "sqlite")]
            RowInner::Sqlite(r) => Ok(r.try_get(name)?),
            #[cfg(feature = "mssql")]
            RowInner::Mssql(r) => r.try_get(name),
            #[cfg(feature = "postgres")]
            RowInner::Postgres(r) => Ok(r.try_get(name)?),
            #[cfg(feature = "mysql")]
            RowInner::Mysql(r) => Ok(r.try_get(name)?),
            #[cfg(any(feature = "turso", feature = "turso-sync"))]
            RowInner::Turso(r) => { let index = r.columns.iter().position(|c| c == name).ok_or_else(|| crate::Error::ColumnNotFound(name.to_string()))?; r.try_get(index) },
        }
    }

    /// gets the value for a column in the row by its index (position, zero based index). 
    /// Errors: 
    ///  * if column missing, out of bounds
    ///  * if column could not be deserialized into requested type <T>
    pub fn get_by_position<T>(&self, index: usize) -> Result<T>
      where T: for<'r> Decode<'r, sqlx::Sqlite> + Type<sqlx::Sqlite> + for<'r> Decode<'r, sqlx::MySql> + Type<sqlx::MySql> + TiberiusDecode
    {
        match &self.inner {
            #[cfg(feature = "sqlite")]
            RowInner::Sqlite(r) => Ok(r.try_get(index)?),
            #[cfg(feature = "mssql")]
            RowInner::Mssql(r) => r.try_get_by_posision(index),
            #[cfg(feature = "postgres")]
            RowInner::Postgres(r) => Ok(r.try_get(index)?),
            #[cfg(feature = "mysql")]
            RowInner::Mysql(r) => Ok(r.try_get(index)?),
            #[cfg(any(feature = "turso", feature = "turso-sync"))]
            RowInner::Turso(r) => r.try_get(index),
        }
    }


}





#[cfg(all(feature = "sqlite", feature = "mysql", feature = "turso", not(feature = "postgres"), not(feature = "mssql")))]
impl Row {

    /// gets the value for a column in the row by its name. 
    /// Errors: 
    ///  * if column missing
    ///  * if column could not be deserialized into requested type <T>
    pub fn get<T>(&self, name: &str) -> Result<T>
      where T: for<'r> Decode<'r, sqlx::Sqlite> + Type<sqlx::Sqlite> + for<'r> Decode<'r, sqlx::MySql> + Type<sqlx::MySql> + crate::turso::TryFromTursoValue
    {
        match &self.inner {
            #[cfg(feature = "sqlite")]
            RowInner::Sqlite(r) => Ok(r.try_get(name)?),
            #[cfg(feature = "mssql")]
            RowInner::Mssql(r) => r.try_get(name),
            #[cfg(feature = "postgres")]
            RowInner::Postgres(r) => Ok(r.try_get(name)?),
            #[cfg(feature = "mysql")]
            RowInner::Mysql(r) => Ok(r.try_get(name)?),
            #[cfg(any(feature = "turso", feature = "turso-sync"))]
            RowInner::Turso(r) => { let index = r.columns.iter().position(|c| c == name).ok_or_else(|| crate::Error::ColumnNotFound(name.to_string()))?; r.try_get(index) },
        }
    }

    /// gets the value for a column in the row by its index (position, zero based index). 
    /// Errors: 
    ///  * if column missing, out of bounds
    ///  * if column could not be deserialized into requested type <T>
    pub fn get_by_position<T>(&self, index: usize) -> Result<T>
      where T: for<'r> Decode<'r, sqlx::Sqlite> + Type<sqlx::Sqlite> + for<'r> Decode<'r, sqlx::MySql> + Type<sqlx::MySql> + crate::turso::TryFromTursoValue
    {
        match &self.inner {
            #[cfg(feature = "sqlite")]
            RowInner::Sqlite(r) => Ok(r.try_get(index)?),
            #[cfg(feature = "mssql")]
            RowInner::Mssql(r) => r.try_get_by_posision(index),
            #[cfg(feature = "postgres")]
            RowInner::Postgres(r) => Ok(r.try_get(index)?),
            #[cfg(feature = "mysql")]
            RowInner::Mysql(r) => Ok(r.try_get(index)?),
            #[cfg(any(feature = "turso", feature = "turso-sync"))]
            RowInner::Turso(r) => r.try_get(index),
        }
    }


}





#[cfg(all(feature = "sqlite", feature = "mssql", feature = "turso", not(feature = "postgres"), not(feature = "mysql")))]
impl Row {

    /// gets the value for a column in the row by its name. 
    /// Errors: 
    ///  * if column missing
    ///  * if column could not be deserialized into requested type <T>
    pub fn get<T>(&self, name: &str) -> Result<T>
      where T: for<'r> Decode<'r, sqlx::Sqlite> + Type<sqlx::Sqlite> + TiberiusDecode + crate::turso::TryFromTursoValue
    {
        match &self.inner {
            #[cfg(feature = "sqlite")]
            RowInner::Sqlite(r) => Ok(r.try_get(name)?),
            #[cfg(feature = "mssql")]
            RowInner::Mssql(r) => r.try_get(name),
            #[cfg(feature = "postgres")]
            RowInner::Postgres(r) => Ok(r.try_get(name)?),
            #[cfg(feature = "mysql")]
            RowInner::Mysql(r) => Ok(r.try_get(name)?),
            #[cfg(any(feature = "turso", feature = "turso-sync"))]
            RowInner::Turso(r) => { let index = r.columns.iter().position(|c| c == name).ok_or_else(|| crate::Error::ColumnNotFound(name.to_string()))?; r.try_get(index) },
        }
    }

    /// gets the value for a column in the row by its index (position, zero based index). 
    /// Errors: 
    ///  * if column missing, out of bounds
    ///  * if column could not be deserialized into requested type <T>
    pub fn get_by_position<T>(&self, index: usize) -> Result<T>
      where T: for<'r> Decode<'r, sqlx::Sqlite> + Type<sqlx::Sqlite> + TiberiusDecode + crate::turso::TryFromTursoValue
    {
        match &self.inner {
            #[cfg(feature = "sqlite")]
            RowInner::Sqlite(r) => Ok(r.try_get(index)?),
            #[cfg(feature = "mssql")]
            RowInner::Mssql(r) => r.try_get_by_posision(index),
            #[cfg(feature = "postgres")]
            RowInner::Postgres(r) => Ok(r.try_get(index)?),
            #[cfg(feature = "mysql")]
            RowInner::Mysql(r) => Ok(r.try_get(index)?),
            #[cfg(any(feature = "turso", feature = "turso-sync"))]
            RowInner::Turso(r) => r.try_get(index),
        }
    }


}





#[cfg(all(feature = "postgres", feature = "mysql", feature = "mssql", not(feature = "sqlite"), not(feature = "turso")))]
impl Row {

    /// gets the value for a column in the row by its name. 
    /// Errors: 
    ///  * if column missing
    ///  * if column could not be deserialized into requested type <T>
    pub fn get<T>(&self, name: &str) -> Result<T>
      where T: for<'r> Decode<'r, sqlx::Postgres> + Type<sqlx::Postgres> + for<'r> Decode<'r, sqlx::MySql> + Type<sqlx::MySql> + TiberiusDecode
    {
        match &self.inner {
            #[cfg(feature = "sqlite")]
            RowInner::Sqlite(r) => Ok(r.try_get(name)?),
            #[cfg(feature = "mssql")]
            RowInner::Mssql(r) => r.try_get(name),
            #[cfg(feature = "postgres")]
            RowInner::Postgres(r) => Ok(r.try_get(name)?),
            #[cfg(feature = "mysql")]
            RowInner::Mysql(r) => Ok(r.try_get(name)?),
            #[cfg(any(feature = "turso", feature = "turso-sync"))]
            RowInner::Turso(r) => { let index = r.columns.iter().position(|c| c == name).ok_or_else(|| crate::Error::ColumnNotFound(name.to_string()))?; r.try_get(index) },
        }
    }

    /// gets the value for a column in the row by its index (position, zero based index). 
    /// Errors: 
    ///  * if column missing, out of bounds
    ///  * if column could not be deserialized into requested type <T>
    pub fn get_by_position<T>(&self, index: usize) -> Result<T>
      where T: for<'r> Decode<'r, sqlx::Postgres> + Type<sqlx::Postgres> + for<'r> Decode<'r, sqlx::MySql> + Type<sqlx::MySql> + TiberiusDecode
    {
        match &self.inner {
            #[cfg(feature = "sqlite")]
            RowInner::Sqlite(r) => Ok(r.try_get(index)?),
            #[cfg(feature = "mssql")]
            RowInner::Mssql(r) => r.try_get_by_posision(index),
            #[cfg(feature = "postgres")]
            RowInner::Postgres(r) => Ok(r.try_get(index)?),
            #[cfg(feature = "mysql")]
            RowInner::Mysql(r) => Ok(r.try_get(index)?),
            #[cfg(any(feature = "turso", feature = "turso-sync"))]
            RowInner::Turso(r) => r.try_get(index),
        }
    }


}





#[cfg(all(feature = "postgres", feature = "mysql", feature = "turso", not(feature = "sqlite"), not(feature = "mssql")))]
impl Row {

    /// gets the value for a column in the row by its name. 
    /// Errors: 
    ///  * if column missing
    ///  * if column could not be deserialized into requested type <T>
    pub fn get<T>(&self, name: &str) -> Result<T>
      where T: for<'r> Decode<'r, sqlx::Postgres> + Type<sqlx::Postgres> + for<'r> Decode<'r, sqlx::MySql> + Type<sqlx::MySql> + crate::turso::TryFromTursoValue
    {
        match &self.inner {
            #[cfg(feature = "sqlite")]
            RowInner::Sqlite(r) => Ok(r.try_get(name)?),
            #[cfg(feature = "mssql")]
            RowInner::Mssql(r) => r.try_get(name),
            #[cfg(feature = "postgres")]
            RowInner::Postgres(r) => Ok(r.try_get(name)?),
            #[cfg(feature = "mysql")]
            RowInner::Mysql(r) => Ok(r.try_get(name)?),
            #[cfg(any(feature = "turso", feature = "turso-sync"))]
            RowInner::Turso(r) => { let index = r.columns.iter().position(|c| c == name).ok_or_else(|| crate::Error::ColumnNotFound(name.to_string()))?; r.try_get(index) },
        }
    }

    /// gets the value for a column in the row by its index (position, zero based index). 
    /// Errors: 
    ///  * if column missing, out of bounds
    ///  * if column could not be deserialized into requested type <T>
    pub fn get_by_position<T>(&self, index: usize) -> Result<T>
      where T: for<'r> Decode<'r, sqlx::Postgres> + Type<sqlx::Postgres> + for<'r> Decode<'r, sqlx::MySql> + Type<sqlx::MySql> + crate::turso::TryFromTursoValue
    {
        match &self.inner {
            #[cfg(feature = "sqlite")]
            RowInner::Sqlite(r) => Ok(r.try_get(index)?),
            #[cfg(feature = "mssql")]
            RowInner::Mssql(r) => r.try_get_by_posision(index),
            #[cfg(feature = "postgres")]
            RowInner::Postgres(r) => Ok(r.try_get(index)?),
            #[cfg(feature = "mysql")]
            RowInner::Mysql(r) => Ok(r.try_get(index)?),
            #[cfg(any(feature = "turso", feature = "turso-sync"))]
            RowInner::Turso(r) => r.try_get(index),
        }
    }


}





#[cfg(all(feature = "postgres", feature = "mssql", feature = "turso", not(feature = "sqlite"), not(feature = "mysql")))]
impl Row {

    /// gets the value for a column in the row by its name. 
    /// Errors: 
    ///  * if column missing
    ///  * if column could not be deserialized into requested type <T>
    pub fn get<T>(&self, name: &str) -> Result<T>
      where T: for<'r> Decode<'r, sqlx::Postgres> + Type<sqlx::Postgres> + TiberiusDecode + crate::turso::TryFromTursoValue
    {
        match &self.inner {
            #[cfg(feature = "sqlite")]
            RowInner::Sqlite(r) => Ok(r.try_get(name)?),
            #[cfg(feature = "mssql")]
            RowInner::Mssql(r) => r.try_get(name),
            #[cfg(feature = "postgres")]
            RowInner::Postgres(r) => Ok(r.try_get(name)?),
            #[cfg(feature = "mysql")]
            RowInner::Mysql(r) => Ok(r.try_get(name)?),
            #[cfg(any(feature = "turso", feature = "turso-sync"))]
            RowInner::Turso(r) => { let index = r.columns.iter().position(|c| c == name).ok_or_else(|| crate::Error::ColumnNotFound(name.to_string()))?; r.try_get(index) },
        }
    }

    /// gets the value for a column in the row by its index (position, zero based index). 
    /// Errors: 
    ///  * if column missing, out of bounds
    ///  * if column could not be deserialized into requested type <T>
    pub fn get_by_position<T>(&self, index: usize) -> Result<T>
      where T: for<'r> Decode<'r, sqlx::Postgres> + Type<sqlx::Postgres> + TiberiusDecode + crate::turso::TryFromTursoValue
    {
        match &self.inner {
            #[cfg(feature = "sqlite")]
            RowInner::Sqlite(r) => Ok(r.try_get(index)?),
            #[cfg(feature = "mssql")]
            RowInner::Mssql(r) => r.try_get_by_posision(index),
            #[cfg(feature = "postgres")]
            RowInner::Postgres(r) => Ok(r.try_get(index)?),
            #[cfg(feature = "mysql")]
            RowInner::Mysql(r) => Ok(r.try_get(index)?),
            #[cfg(any(feature = "turso", feature = "turso-sync"))]
            RowInner::Turso(r) => r.try_get(index),
        }
    }


}





#[cfg(all(feature = "mysql", feature = "mssql", feature = "turso", not(feature = "sqlite"), not(feature = "postgres")))]
impl Row {

    /// gets the value for a column in the row by its name. 
    /// Errors: 
    ///  * if column missing
    ///  * if column could not be deserialized into requested type <T>
    pub fn get<T>(&self, name: &str) -> Result<T>
      where T: for<'r> Decode<'r, sqlx::MySql> + Type<sqlx::MySql> + TiberiusDecode + crate::turso::TryFromTursoValue
    {
        match &self.inner {
            #[cfg(feature = "sqlite")]
            RowInner::Sqlite(r) => Ok(r.try_get(name)?),
            #[cfg(feature = "mssql")]
            RowInner::Mssql(r) => r.try_get(name),
            #[cfg(feature = "postgres")]
            RowInner::Postgres(r) => Ok(r.try_get(name)?),
            #[cfg(feature = "mysql")]
            RowInner::Mysql(r) => Ok(r.try_get(name)?),
            #[cfg(any(feature = "turso", feature = "turso-sync"))]
            RowInner::Turso(r) => { let index = r.columns.iter().position(|c| c == name).ok_or_else(|| crate::Error::ColumnNotFound(name.to_string()))?; r.try_get(index) },
        }
    }

    /// gets the value for a column in the row by its index (position, zero based index). 
    /// Errors: 
    ///  * if column missing, out of bounds
    ///  * if column could not be deserialized into requested type <T>
    pub fn get_by_position<T>(&self, index: usize) -> Result<T>
      where T: for<'r> Decode<'r, sqlx::MySql> + Type<sqlx::MySql> + TiberiusDecode + crate::turso::TryFromTursoValue
    {
        match &self.inner {
            #[cfg(feature = "sqlite")]
            RowInner::Sqlite(r) => Ok(r.try_get(index)?),
            #[cfg(feature = "mssql")]
            RowInner::Mssql(r) => r.try_get_by_posision(index),
            #[cfg(feature = "postgres")]
            RowInner::Postgres(r) => Ok(r.try_get(index)?),
            #[cfg(feature = "mysql")]
            RowInner::Mysql(r) => Ok(r.try_get(index)?),
            #[cfg(any(feature = "turso", feature = "turso-sync"))]
            RowInner::Turso(r) => r.try_get(index),
        }
    }


}





#[cfg(all(feature = "sqlite", feature = "postgres", feature = "mysql", feature = "mssql", not(feature = "turso")))]
impl Row {

    /// gets the value for a column in the row by its name. 
    /// Errors: 
    ///  * if column missing
    ///  * if column could not be deserialized into requested type <T>
    pub fn get<T>(&self, name: &str) -> Result<T>
      where T: for<'r> Decode<'r, sqlx::Sqlite> + Type<sqlx::Sqlite> + for<'r> Decode<'r, sqlx::Postgres> + Type<sqlx::Postgres> + for<'r> Decode<'r, sqlx::MySql> + Type<sqlx::MySql> + TiberiusDecode
    {
        match &self.inner {
            #[cfg(feature = "sqlite")]
            RowInner::Sqlite(r) => Ok(r.try_get(name)?),
            #[cfg(feature = "mssql")]
            RowInner::Mssql(r) => r.try_get(name),
            #[cfg(feature = "postgres")]
            RowInner::Postgres(r) => Ok(r.try_get(name)?),
            #[cfg(feature = "mysql")]
            RowInner::Mysql(r) => Ok(r.try_get(name)?),
            #[cfg(any(feature = "turso", feature = "turso-sync"))]
            RowInner::Turso(r) => { let index = r.columns.iter().position(|c| c == name).ok_or_else(|| crate::Error::ColumnNotFound(name.to_string()))?; r.try_get(index) },
        }
    }

    /// gets the value for a column in the row by its index (position, zero based index). 
    /// Errors: 
    ///  * if column missing, out of bounds
    ///  * if column could not be deserialized into requested type <T>
    pub fn get_by_position<T>(&self, index: usize) -> Result<T>
      where T: for<'r> Decode<'r, sqlx::Sqlite> + Type<sqlx::Sqlite> + for<'r> Decode<'r, sqlx::Postgres> + Type<sqlx::Postgres> + for<'r> Decode<'r, sqlx::MySql> + Type<sqlx::MySql> + TiberiusDecode
    {
        match &self.inner {
            #[cfg(feature = "sqlite")]
            RowInner::Sqlite(r) => Ok(r.try_get(index)?),
            #[cfg(feature = "mssql")]
            RowInner::Mssql(r) => r.try_get_by_posision(index),
            #[cfg(feature = "postgres")]
            RowInner::Postgres(r) => Ok(r.try_get(index)?),
            #[cfg(feature = "mysql")]
            RowInner::Mysql(r) => Ok(r.try_get(index)?),
            #[cfg(any(feature = "turso", feature = "turso-sync"))]
            RowInner::Turso(r) => r.try_get(index),
        }
    }


}





#[cfg(all(feature = "sqlite", feature = "postgres", feature = "mysql", feature = "turso", not(feature = "mssql")))]
impl Row {

    /// gets the value for a column in the row by its name. 
    /// Errors: 
    ///  * if column missing
    ///  * if column could not be deserialized into requested type <T>
    pub fn get<T>(&self, name: &str) -> Result<T>
      where T: for<'r> Decode<'r, sqlx::Sqlite> + Type<sqlx::Sqlite> + for<'r> Decode<'r, sqlx::Postgres> + Type<sqlx::Postgres> + for<'r> Decode<'r, sqlx::MySql> + Type<sqlx::MySql> + crate::turso::TryFromTursoValue
    {
        match &self.inner {
            #[cfg(feature = "sqlite")]
            RowInner::Sqlite(r) => Ok(r.try_get(name)?),
            #[cfg(feature = "mssql")]
            RowInner::Mssql(r) => r.try_get(name),
            #[cfg(feature = "postgres")]
            RowInner::Postgres(r) => Ok(r.try_get(name)?),
            #[cfg(feature = "mysql")]
            RowInner::Mysql(r) => Ok(r.try_get(name)?),
            #[cfg(any(feature = "turso", feature = "turso-sync"))]
            RowInner::Turso(r) => { let index = r.columns.iter().position(|c| c == name).ok_or_else(|| crate::Error::ColumnNotFound(name.to_string()))?; r.try_get(index) },
        }
    }

    /// gets the value for a column in the row by its index (position, zero based index). 
    /// Errors: 
    ///  * if column missing, out of bounds
    ///  * if column could not be deserialized into requested type <T>
    pub fn get_by_position<T>(&self, index: usize) -> Result<T>
      where T: for<'r> Decode<'r, sqlx::Sqlite> + Type<sqlx::Sqlite> + for<'r> Decode<'r, sqlx::Postgres> + Type<sqlx::Postgres> + for<'r> Decode<'r, sqlx::MySql> + Type<sqlx::MySql> + crate::turso::TryFromTursoValue
    {
        match &self.inner {
            #[cfg(feature = "sqlite")]
            RowInner::Sqlite(r) => Ok(r.try_get(index)?),
            #[cfg(feature = "mssql")]
            RowInner::Mssql(r) => r.try_get_by_posision(index),
            #[cfg(feature = "postgres")]
            RowInner::Postgres(r) => Ok(r.try_get(index)?),
            #[cfg(feature = "mysql")]
            RowInner::Mysql(r) => Ok(r.try_get(index)?),
            #[cfg(any(feature = "turso", feature = "turso-sync"))]
            RowInner::Turso(r) => r.try_get(index),
        }
    }


}





#[cfg(all(feature = "sqlite", feature = "postgres", feature = "mssql", feature = "turso", not(feature = "mysql")))]
impl Row {

    /// gets the value for a column in the row by its name. 
    /// Errors: 
    ///  * if column missing
    ///  * if column could not be deserialized into requested type <T>
    pub fn get<T>(&self, name: &str) -> Result<T>
      where T: for<'r> Decode<'r, sqlx::Sqlite> + Type<sqlx::Sqlite> + for<'r> Decode<'r, sqlx::Postgres> + Type<sqlx::Postgres> + TiberiusDecode + crate::turso::TryFromTursoValue
    {
        match &self.inner {
            #[cfg(feature = "sqlite")]
            RowInner::Sqlite(r) => Ok(r.try_get(name)?),
            #[cfg(feature = "mssql")]
            RowInner::Mssql(r) => r.try_get(name),
            #[cfg(feature = "postgres")]
            RowInner::Postgres(r) => Ok(r.try_get(name)?),
            #[cfg(feature = "mysql")]
            RowInner::Mysql(r) => Ok(r.try_get(name)?),
            #[cfg(any(feature = "turso", feature = "turso-sync"))]
            RowInner::Turso(r) => { let index = r.columns.iter().position(|c| c == name).ok_or_else(|| crate::Error::ColumnNotFound(name.to_string()))?; r.try_get(index) },
        }
    }

    /// gets the value for a column in the row by its index (position, zero based index). 
    /// Errors: 
    ///  * if column missing, out of bounds
    ///  * if column could not be deserialized into requested type <T>
    pub fn get_by_position<T>(&self, index: usize) -> Result<T>
      where T: for<'r> Decode<'r, sqlx::Sqlite> + Type<sqlx::Sqlite> + for<'r> Decode<'r, sqlx::Postgres> + Type<sqlx::Postgres> + TiberiusDecode + crate::turso::TryFromTursoValue
    {
        match &self.inner {
            #[cfg(feature = "sqlite")]
            RowInner::Sqlite(r) => Ok(r.try_get(index)?),
            #[cfg(feature = "mssql")]
            RowInner::Mssql(r) => r.try_get_by_posision(index),
            #[cfg(feature = "postgres")]
            RowInner::Postgres(r) => Ok(r.try_get(index)?),
            #[cfg(feature = "mysql")]
            RowInner::Mysql(r) => Ok(r.try_get(index)?),
            #[cfg(any(feature = "turso", feature = "turso-sync"))]
            RowInner::Turso(r) => r.try_get(index),
        }
    }


}





#[cfg(all(feature = "sqlite", feature = "mysql", feature = "mssql", feature = "turso", not(feature = "postgres")))]
impl Row {

    /// gets the value for a column in the row by its name. 
    /// Errors: 
    ///  * if column missing
    ///  * if column could not be deserialized into requested type <T>
    pub fn get<T>(&self, name: &str) -> Result<T>
      where T: for<'r> Decode<'r, sqlx::Sqlite> + Type<sqlx::Sqlite> + for<'r> Decode<'r, sqlx::MySql> + Type<sqlx::MySql> + TiberiusDecode + crate::turso::TryFromTursoValue
    {
        match &self.inner {
            #[cfg(feature = "sqlite")]
            RowInner::Sqlite(r) => Ok(r.try_get(name)?),
            #[cfg(feature = "mssql")]
            RowInner::Mssql(r) => r.try_get(name),
            #[cfg(feature = "postgres")]
            RowInner::Postgres(r) => Ok(r.try_get(name)?),
            #[cfg(feature = "mysql")]
            RowInner::Mysql(r) => Ok(r.try_get(name)?),
            #[cfg(any(feature = "turso", feature = "turso-sync"))]
            RowInner::Turso(r) => { let index = r.columns.iter().position(|c| c == name).ok_or_else(|| crate::Error::ColumnNotFound(name.to_string()))?; r.try_get(index) },
        }
    }

    /// gets the value for a column in the row by its index (position, zero based index). 
    /// Errors: 
    ///  * if column missing, out of bounds
    ///  * if column could not be deserialized into requested type <T>
    pub fn get_by_position<T>(&self, index: usize) -> Result<T>
      where T: for<'r> Decode<'r, sqlx::Sqlite> + Type<sqlx::Sqlite> + for<'r> Decode<'r, sqlx::MySql> + Type<sqlx::MySql> + TiberiusDecode + crate::turso::TryFromTursoValue
    {
        match &self.inner {
            #[cfg(feature = "sqlite")]
            RowInner::Sqlite(r) => Ok(r.try_get(index)?),
            #[cfg(feature = "mssql")]
            RowInner::Mssql(r) => r.try_get_by_posision(index),
            #[cfg(feature = "postgres")]
            RowInner::Postgres(r) => Ok(r.try_get(index)?),
            #[cfg(feature = "mysql")]
            RowInner::Mysql(r) => Ok(r.try_get(index)?),
            #[cfg(any(feature = "turso", feature = "turso-sync"))]
            RowInner::Turso(r) => r.try_get(index),
        }
    }


}





#[cfg(all(feature = "postgres", feature = "mysql", feature = "mssql", feature = "turso", not(feature = "sqlite")))]
impl Row {

    /// gets the value for a column in the row by its name. 
    /// Errors: 
    ///  * if column missing
    ///  * if column could not be deserialized into requested type <T>
    pub fn get<T>(&self, name: &str) -> Result<T>
      where T: for<'r> Decode<'r, sqlx::Postgres> + Type<sqlx::Postgres> + for<'r> Decode<'r, sqlx::MySql> + Type<sqlx::MySql> + TiberiusDecode + crate::turso::TryFromTursoValue
    {
        match &self.inner {
            #[cfg(feature = "sqlite")]
            RowInner::Sqlite(r) => Ok(r.try_get(name)?),
            #[cfg(feature = "mssql")]
            RowInner::Mssql(r) => r.try_get(name),
            #[cfg(feature = "postgres")]
            RowInner::Postgres(r) => Ok(r.try_get(name)?),
            #[cfg(feature = "mysql")]
            RowInner::Mysql(r) => Ok(r.try_get(name)?),
            #[cfg(any(feature = "turso", feature = "turso-sync"))]
            RowInner::Turso(r) => { let index = r.columns.iter().position(|c| c == name).ok_or_else(|| crate::Error::ColumnNotFound(name.to_string()))?; r.try_get(index) },
        }
    }

    /// gets the value for a column in the row by its index (position, zero based index). 
    /// Errors: 
    ///  * if column missing, out of bounds
    ///  * if column could not be deserialized into requested type <T>
    pub fn get_by_position<T>(&self, index: usize) -> Result<T>
      where T: for<'r> Decode<'r, sqlx::Postgres> + Type<sqlx::Postgres> + for<'r> Decode<'r, sqlx::MySql> + Type<sqlx::MySql> + TiberiusDecode + crate::turso::TryFromTursoValue
    {
        match &self.inner {
            #[cfg(feature = "sqlite")]
            RowInner::Sqlite(r) => Ok(r.try_get(index)?),
            #[cfg(feature = "mssql")]
            RowInner::Mssql(r) => r.try_get_by_posision(index),
            #[cfg(feature = "postgres")]
            RowInner::Postgres(r) => Ok(r.try_get(index)?),
            #[cfg(feature = "mysql")]
            RowInner::Mysql(r) => Ok(r.try_get(index)?),
            #[cfg(any(feature = "turso", feature = "turso-sync"))]
            RowInner::Turso(r) => r.try_get(index),
        }
    }


}





#[cfg(all(feature = "sqlite", feature = "postgres", feature = "mysql", feature = "mssql", feature = "turso"))]
impl Row {

    /// gets the value for a column in the row by its name. 
    /// Errors: 
    ///  * if column missing
    ///  * if column could not be deserialized into requested type <T>
    pub fn get<T>(&self, name: &str) -> Result<T>
      where T: for<'r> Decode<'r, sqlx::Sqlite> + Type<sqlx::Sqlite> + for<'r> Decode<'r, sqlx::Postgres> + Type<sqlx::Postgres> + for<'r> Decode<'r, sqlx::MySql> + Type<sqlx::MySql> + TiberiusDecode + crate::turso::TryFromTursoValue
    {
        match &self.inner {
            #[cfg(feature = "sqlite")]
            RowInner::Sqlite(r) => Ok(r.try_get(name)?),
            #[cfg(feature = "mssql")]
            RowInner::Mssql(r) => r.try_get(name),
            #[cfg(feature = "postgres")]
            RowInner::Postgres(r) => Ok(r.try_get(name)?),
            #[cfg(feature = "mysql")]
            RowInner::Mysql(r) => Ok(r.try_get(name)?),
            #[cfg(any(feature = "turso", feature = "turso-sync"))]
            RowInner::Turso(r) => { let index = r.columns.iter().position(|c| c == name).ok_or_else(|| crate::Error::ColumnNotFound(name.to_string()))?; r.try_get(index) },
        }
    }

    /// gets the value for a column in the row by its index (position, zero based index). 
    /// Errors: 
    ///  * if column missing, out of bounds
    ///  * if column could not be deserialized into requested type <T>
    pub fn get_by_position<T>(&self, index: usize) -> Result<T>
      where T: for<'r> Decode<'r, sqlx::Sqlite> + Type<sqlx::Sqlite> + for<'r> Decode<'r, sqlx::Postgres> + Type<sqlx::Postgres> + for<'r> Decode<'r, sqlx::MySql> + Type<sqlx::MySql> + TiberiusDecode + crate::turso::TryFromTursoValue
    {
        match &self.inner {
            #[cfg(feature = "sqlite")]
            RowInner::Sqlite(r) => Ok(r.try_get(index)?),
            #[cfg(feature = "mssql")]
            RowInner::Mssql(r) => r.try_get_by_posision(index),
            #[cfg(feature = "postgres")]
            RowInner::Postgres(r) => Ok(r.try_get(index)?),
            #[cfg(feature = "mysql")]
            RowInner::Mysql(r) => Ok(r.try_get(index)?),
            #[cfg(any(feature = "turso", feature = "turso-sync"))]
            RowInner::Turso(r) => r.try_get(index),
        }
    }


}

