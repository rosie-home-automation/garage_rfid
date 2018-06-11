use diesel;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use std::fmt;
use std::sync::{Arc, Mutex};
use slog;

use models::credential::Credential;
use schema::credentials;
use slacker::Slacker;

#[derive(Debug)]
pub enum Variety {
  RFID,
  PIN
}
impl fmt::Display for Variety {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    fmt::Debug::fmt(self, f)
  }
}

pub struct Bouncer {
  connection: SqliteConnection,
  slacker: Arc<Mutex<Slacker>>,
}

impl Bouncer {
  pub fn new(connection: SqliteConnection, slacker: Arc<Mutex<Slacker>>) -> Bouncer {
    Bouncer {
      connection: connection,
      slacker: slacker,
    }
  }

  pub fn is_authorized(&self, logger: slog::Logger, variety: Variety, value: &str)
    -> Result<bool, diesel::result::Error>
  {
    let credential_option = credentials::dsl::credentials
      .filter(credentials::dsl::variety.eq(variety.to_string())
      .and(credentials::dsl::value.eq(value)))
      .first::<Credential>(&self.connection)
      .optional()?;

    match credential_option {
      Some(credential) => {
        info!(
          logger,
          "Bouncer approved.";
          "module" => "Bouncer", "action" => "is_authorized", "status" => "approved",
          "variety" => ?variety, "credential" => ?credential
        );
        self.slacker.lock().unwrap().send_text(
          format!("Bouncer approved {variety} {name}.", name = &credential.name, variety = &variety).as_str(),
          logger
        );
        Ok(true)
      },
      None => {
        info!(
          logger,
          "Bouncer denied.";
          "module" => "Bouncer", "action" => "is_authorized", "status" => "denied",
          "variety" => ?variety, "value" => value
        );
        self.slacker.lock().unwrap().send_text(
          format!("Bouncer denied {variety} {value}.", variety = &variety, value = &value).as_str(),
          logger
        );
        Ok(false)
      },
    }
  }
}

impl fmt::Debug for Bouncer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Bouncer {{ }}")
    }
}
