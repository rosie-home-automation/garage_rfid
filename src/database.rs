use diesel::prelude::{Connection, SqliteConnection};
use std::fmt::{Debug, Formatter, Result};
use r2d2::Pool;
use r2d2_diesel::ConnectionManager;
use slog;

use configuration::Configuration;

#[derive(Clone)]
pub struct Database {
  database_url: String,
  logger: slog::Logger,
  pool: Pool<ConnectionManager<SqliteConnection>>,
}

impl Database {
  pub fn new(logger: slog::Logger, configuration: &Configuration) -> Database {
    let database_url = configuration.database.url.clone();
    let logger = logger.new(o!("database_url" => database_url.to_string()));
    let pool = Database::build_pool(database_url.clone());
    Database { database_url: database_url, logger: logger, pool: pool }
  }

  fn build_pool(database_url: String) -> Pool<ConnectionManager<SqliteConnection>> {
    let manager = ConnectionManager::<SqliteConnection>::new(database_url.clone());
    Pool::builder().build(manager).expect("Failed to create pool.")
  }

  pub fn connection(&self) -> SqliteConnection {
    match SqliteConnection::establish(&self.database_url) {
      Ok(conn) => conn,
      Err(err) => {
        error!(self.logger, "Error connecting to the database."; "err" => ?err);
        panic!("TODO: Remove panic for Database::connection");
      }
    }
  }

  pub fn pool(&self) -> &Pool<ConnectionManager<SqliteConnection>> {
    &self.pool
  }
}

impl Debug for Database {
  fn fmt(&self, f: &mut Formatter) -> Result {
    write!(
      f,
      "Database {{ database_url: {}, logger: {:?}, skipped_attributes: ['pool'] }}",
      &self.database_url, &self.logger
    )
  }
}
