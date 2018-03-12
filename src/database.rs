use diesel::prelude::{Connection, SqliteConnection};
use configuration::Configuration;

pub struct Database {
  database_url: String
}

impl Database {
  pub fn new(configuration: &Configuration) -> Database {
    let database_url = configuration.database.url.clone();
    Database { database_url }
  }

  pub fn connection(&self) -> SqliteConnection {
    SqliteConnection::establish(&self.database_url)
      .expect(&format!("Error connecting to {}", self.database_url))
  }
}
