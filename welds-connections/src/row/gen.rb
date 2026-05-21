
head = %Q|
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
            RowInner::SqliteSync(r) => { let index = r.columns.iter().position(\|c\| c == name).ok_or(crate::Error::ColumnNotFound(name.to_string()))?; Ok(r.try_get(index)?) },
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
            RowInner::Turso(r) => { let index = r.columns.iter().position(\|c\| c == name).ok_or_else(\|\| crate::Error::ColumnNotFound(name.to_string()))?; r.try_get(index) },
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
            RowInner::SqliteSync(r) => { let index = r.columns.iter().position(\|c\| c == name).ok_or(crate::Error::ColumnNotFound(name.to_string()))?; Ok(r.try_get(index)?) },
            #[cfg(any(feature = "turso", feature = "turso-sync"))]
            RowInner::Turso(r) => { let index = r.columns.iter().position(\|c\| c == name).ok_or_else(\|\| crate::Error::ColumnNotFound(name.to_string()))?; r.try_get(index) },
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
|

def blocky(cfg, wheres)
  %Q|

#{cfg}
impl Row {

    /// gets the value for a column in the row by its name. 
    /// Errors: 
    ///  * if column missing
    ///  * if column could not be deserialized into requested type <T>
    pub fn get<T>(&self, name: &str) -> Result<T>
      where T: #{wheres}
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
            RowInner::Turso(r) => { let index = r.columns.iter().position(\|c\| c == name).ok_or_else(\|\| crate::Error::ColumnNotFound(name.to_string()))?; r.try_get(index) },
        }
    }

    /// gets the value for a column in the row by its index (position, zero based index). 
    /// Errors: 
    ///  * if column missing, out of bounds
    ///  * if column could not be deserialized into requested type <T>
    pub fn get_by_position<T>(&self, index: usize) -> Result<T>
      where T: #{wheres}
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

|


end

p = [
  ["sqlite"  , "for<'r> Decode<'r, sqlx::Sqlite> + Type<sqlx::Sqlite>"],
  ["postgres", "for<'r> Decode<'r, sqlx::Postgres> + Type<sqlx::Postgres>"],
  ["mysql"   , "for<'r> Decode<'r, sqlx::MySql> + Type<sqlx::MySql>"],
  ["mssql"   , "TiberiusDecode"],
  ["turso"   , "crate::turso::TryFromTursoValue"],
]

cc = p.combination(1) + p.combination(2) + p.combination(3) + p.combination(4) + p.combination(5)

all = ["sqlite", "postgres", "mysql", "mssql", "turso"]

full = head

cc.each do |c| 

  enabled_list = c.map{|a| a[0] } 
  disabled_list = all - enabled_list
  
  enabled = enabled_list.map{|f| "feature = \"#{f}\""}
  disabled = disabled_list.map{|f| "not(feature = \"#{f}\")"}
  rules = enabled + disabled
  cfgs = "#[cfg(all(#{rules.join(", ")}))]"

  wheres = c.map{|a| a[1]}
  wheres = wheres.join(" + ")

  full = full + "\n\n" + blocky(cfgs, wheres)
end


puts full

