extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
#[macro_use]
extern crate slog;
extern crate slog_async;
extern crate slog_term;
extern crate garage_rfid;


use slog::*;

use garage_rfid::configuration::Configuration;
use garage_rfid::database::Database;
use garage_rfid::models::log::Log;

fn create_log(logger: slog::Logger, database: &Database) -> Log {
  let module = "ExampleJson";
  let action = "create_log";
  let user_id = None;
  let data = None;
  Log::create(&logger, &database.connection(), module, action, user_id, data)
}

fn setup_slog() -> slog::Logger  {
  let decorator = slog_term::TermDecorator::new().build();
  let stdout_drain = slog_term::CompactFormat::new(decorator).build().fuse();
  let stdout_drain = slog_async::Async::new(stdout_drain).build().fuse();
  slog::Logger::root(stdout_drain, o!("version" => env!("CARGO_PKG_VERSION")))
}

fn test1() {
  let logger = setup_slog();
  let configuration = Configuration::new().unwrap();
  info!(&logger, "CONF"; "configuration" => ?configuration);
  let database = Database::new(logger.clone(), &configuration);
  let log = create_log(logger.clone(), &database);
  let json_log = serde_json::to_string(&log).unwrap();
  info!(&logger, "LOG"; "json_log" => &json_log);
  let result: Log = serde_json::from_str(&json_log).unwrap();
  info!(&logger, "LOG"; "result" => ?result);
}

fn main() {
  test1();
}
