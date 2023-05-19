use std::{error::Error, fmt::Display};

use diesel::{Connection, ConnectionResult, SqliteConnection};
pub struct Database {
    pub connection: SqliteConnection,
}

#[derive(Debug)]
pub enum DatabaseError {
    ConnectionError(diesel::ConnectionError),
    QueryError(diesel::result::Error),
}

impl Display for DatabaseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DatabaseError::ConnectionError(error) => {
                write!(f, "Connection error: {}", error)
            }
            DatabaseError::QueryError(error) => {
                write!(f, "Query error: {}", error)
            }
        }
    }
}

impl Error for DatabaseError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            DatabaseError::ConnectionError(error) => Some(error),
            DatabaseError::QueryError(error) => Some(error),
        }
    }
}

impl Database {
    /// Creates a new `Database` object.
    pub fn new(connection: SqliteConnection) -> Self {
        Self { connection }
    }

    /// Tries to create a new `Database` object from the given URL.
    pub fn from_url(url: impl AsRef<str>) -> ConnectionResult<Self> {
        let connection = SqliteConnection::establish(url.as_ref())?;
        Ok(Self::new(connection))
    }
}

pub trait Operator<Queriable, Insertable> {
    fn get(&mut self, id_: i32) -> Result<Queriable, DatabaseError>;

    fn create(
        &mut self,
        insertable: impl Into<Insertable>,
    ) -> Result<(), DatabaseError>;

    fn update(&mut self, queriable: Queriable) -> Result<(), DatabaseError>;

    fn delete(&mut self, id_: i32) -> Result<(), DatabaseError>;
}
