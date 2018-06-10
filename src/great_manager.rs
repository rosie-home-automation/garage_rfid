use config;
use slog;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use configuration::Configuration;
use database::Database;
use garage_door::GarageDoor;
use http_server::HttpServer;
use root_logger::RootLogger;
use rfid_reader::RfidReader;

pub struct GreatManager {
  pub configuration: Configuration,
  pub database: Database,
  pub garage_door: Arc<Mutex<GarageDoor>>,
  pub http_server: HttpServer,
  pub rfid_reader: RfidReader,
  pub root_logger: RootLogger,
}

impl GreatManager {
  pub fn new() -> Result<GreatManager, config::ConfigError> {
    let configuration = Configuration::new()?;
    let root_logger = RootLogger::new(&configuration);
    let logger = root_logger.root_logger.clone();
    info!(logger, "Initializing...");
    let database = Database::new(logger.clone(), &configuration);
    let garage_door = GarageDoor::new(logger.clone(), &configuration);
    let garage_door = Arc::new(Mutex::new(garage_door));
    let rfid_reader = RfidReader::new(logger.clone(), &configuration, database.clone());
    let http_server = HttpServer::new(
      &configuration,
      database.clone(),
      logger.clone(),
      garage_door.clone(),
    );
    info!(logger, "Initialized");
    Ok(GreatManager {
      configuration: configuration,
      database: database,
      garage_door: garage_door,
      http_server: http_server,
      rfid_reader: rfid_reader,
      root_logger: root_logger,
    })
  }

  pub fn start(&mut self) {
    info!(self.root_logger(), "Starting...");
    self.garage_door.lock().unwrap().start();
    self.rfid_reader.start();
    self.http_server.start();
    loop {
      thread::sleep(Duration::from_millis(1000));
    }
  }

  pub fn root_logger(&self) -> &slog::Logger {
    &self.root_logger.root_logger
  }
}
