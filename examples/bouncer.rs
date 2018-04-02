#[macro_use]
extern crate slog;
extern crate slog_async;
extern crate slog_term;
extern crate garage_rfid;

use slog::*;

use garage_rfid::configuration::Configuration;
use garage_rfid::database::Database;
use garage_rfid::bouncer::{ Bouncer, Variety };

fn setup_slog() -> slog::Logger  {
  let decorator = slog_term::TermDecorator::new().build();
  let stdout_drain = slog_term::CompactFormat::new(decorator).build().fuse();
  let stdout_drain = slog_async::Async::new(stdout_drain).build().fuse();
  slog::Logger::root(stdout_drain, o!("version" => env!("CARGO_PKG_VERSION")))
}

fn test1() {
  let logger = setup_slog();
  let configuration = Configuration::new().unwrap();
  info!(logger, "CONF"; "configuration" => format!("{:?}", configuration));
  let database = Database::new(logger.clone(), &configuration);
  let connection = database.connection();
  let bouncer = Bouncer::new(connection);
  let status = bouncer.is_authorized(logger.clone(), Variety::RFID, "101100101100").unwrap();
  info!(logger, "Status Success {:?}", status);
  let status = bouncer.is_authorized(logger.clone(), Variety::PIN, "234").unwrap();
  info!(logger, "Status Fail {:?}", status);
}

fn main() {
  test1();
}
