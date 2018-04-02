#[macro_use]
extern crate diesel;
#[macro_use]
extern crate slog;
extern crate slog_async;
extern crate slog_atomic;
extern crate slog_json;
extern crate slog_term;
extern crate garage_rfid;

use diesel::prelude::*;
use slog::*;
use std::sync::Mutex;
use std::fs::OpenOptions;

use garage_rfid::configuration::Configuration;
use garage_rfid::database::Database;
use garage_rfid::models::user::User;
use garage_rfid::models::credential::Credential;
use garage_rfid::schema::credentials;
use garage_rfid::schema::users;


fn creat_user(logger: &Logger, database: &Database, name: String) -> User {
  User::create(&logger, &database.connection(), name)
}

fn create_credential(logger: &Logger, database: &Database, user: &User) -> Credential {
  let name = "Taco RFID".to_string();
  let variety = "RFID".to_string();
  let value = "101100101100".to_string();
  Credential::create(&logger, &database.connection(), &user.id, name, variety, value)
}

fn setup_slog() -> Logger  {
  let decorator = slog_term::TermDecorator::new().build();
  let stdout_drain = slog_term::CompactFormat::new(decorator).build().fuse();
  let stdout_drain = slog_async::Async::new(stdout_drain).build().fuse();
  Logger::root(stdout_drain, o!("version" => env!("CARGO_PKG_VERSION")))
}

fn test1() {
  let logger = setup_slog();
  let configuration = Configuration::new().unwrap();
  info!(logger, "CONF"; "configuration" => format!("{:?}", configuration));
  let database = Database::new(logger.clone(), &configuration);
  let connection = database.connection();
  // let user = creat_user(&logger, &database, "Matilda2".to_string());
  // let credential = create_credential(&logger, &database, &user);

  // let user_id = "dea3f799-f6da-423d-aecf-21518a8b76ad";
  // let user = users::table.find(user_id).first::<User>(&connection)
  // let user = users::table.first::<User>(&connection)
  //   .expect(&format!("Error finding user"));
  // let credentials = Credential::belonging_to(&user).load::<Credential>(&connection)
  //   .expect("Error loading credentials");
  // println!("Credentials {:?}", credentials);

  use garage_rfid::schema::credentials::dsl::*;

  let credential = credentials
    .filter(variety.eq("RFID").and(value.eq("1011001011001")))
    .first::<Credential>(&connection)
    .optional()
    .expect("Error finding credential");
  info!(logger, "Cred {:?}", credential);
}

fn main() {
  test1();
}
