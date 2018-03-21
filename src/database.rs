use diesel::prelude::{Connection, SqliteConnection};
use slog;

use configuration::Configuration;

pub struct Database {
  database_url: String,
  logger: slog::Logger,
}

impl Database {
  pub fn new(logger: slog::Logger, configuration: &Configuration) -> Database {
    let database_url = configuration.database.url.clone();
    let logger = logger.new(o!("database_url" => database_url.to_string()));
    Database { database_url: database_url, logger: logger }
  }

  pub fn connection(&self) -> SqliteConnection {
    match SqliteConnection::establish(&self.database_url) {
      Ok(conn) => conn,
      Err(err) => {
        error!(self.logger, "Error connecting to the database."; "err" => %err);
        panic!("TODO: Remove panic for Database::connection");
      }
    }
  }
}
