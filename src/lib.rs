use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenvy::dotenv;
use std::env;
pub mod models;
mod schema;
pub mod ops;

pub enum DataBaseError {
  ConnectionError,
  QueryError,
  InsertError,
  DeleteError,
  UpdateError,
  UnknownError,    
}


pub fn establish_connection() -> Result<PgConnection, DataBaseError> {
  dotenv().ok();

  let database_url = env::var("DATABASE_URL")
    .expect("DATABASE_URL must be set");
  
  PgConnection::establish(&database_url)
    .map_err(|_| DataBaseError::ConnectionError)
}
mod tests {
  
}
