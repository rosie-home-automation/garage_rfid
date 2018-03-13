#[macro_use]
extern crate slog;
extern crate slog_async;
extern crate slog_atomic;
extern crate slog_term;
extern crate garage_rfid;

use slog::*;

use garage_rfid::configuration::Configuration;
use garage_rfid::database::Database;
use garage_rfid::models::user::User;
use garage_rfid::models::credential::Credential;
use garage_rfid::schema::users;


fn creat_user(logger: &Logger, database: &Database, name: String) -> User {
  User::create(&logger, &database.connection(), name)
}

fn create_credential(database: &Database, user: &User) -> Credential {
  let name = "Taco RFID".to_string();
  let variety = "RFID".to_string();
  let value = "101100101100".to_string();
  Credential::create(&database.connection(), &user.id, name, variety, value)
}

fn setup_slog() -> Logger  {
  let decorator = slog_term::PlainDecorator::new(std::io::stdout());
  let drain = slog_term::CompactFormat::new(decorator).build().fuse();
  let drain = slog_async::Async::new(drain).build().fuse();
  // let drain = slog_atomic::AtomicSwitch::new(drain);
  // let ctrl = drain.ctrl();
  // let drain = slog::LevelFilter::new(drain, Level::Warning).ignore_res().map(Fuse);
  Logger::root(drain, o!("version" => env!("CARGO_PKG_VERSION")))
}

fn test1() {
  let logger = setup_slog();
  let configuration = Configuration::new().unwrap();
  println!("CONF {:?}", configuration);
  let database = Database::new(&configuration);
  let connection = database.connection();
  let user = creat_user(&logger, &database, "Matilda".to_string());
  // let credential = create_credential(&database, &user);
  trace!(logger, "TRACE CREATED USER"; "user" => format!("{:?}", user));
  info!(logger, "INFO CREATED USER"; "user" => format!("{:?}", user));
  debug!(logger, "DEBUG CREATED USER"; "user" => format!("{:?}", user));
  warn!(logger, "WARN CREATED USER"; "user" => format!("{:?}", user));
  error!(logger, "ERROR CREATED USER"; "user" => format!("{:?}", user));
  crit!(logger, "CRIT CREATED USER"; "user" => format!("{:?}", user));

  // let user_id = "dea3f799-f6da-423d-aecf-21518a8b76ad";
  // let user = users::table.find(user_id).first::<User>(&connection)
  //   .expect(&format!("Error finding user for id {}", user_id));
  // let credentials = Credential::belonging_to(&user).load::<Credential>(&connection)
  //   .expect("Error loading credentials");
  // println!("Credentials {:?}", credentials);
}

fn main() {
  test1();
}
