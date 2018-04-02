use config;
use slog;
use std::thread;
use std::time::Duration;

use configuration::Configuration;
use database::Database;
use root_logger::RootLogger;
use rfid_reader::RfidReader;

pub struct GreatManager {
  pub configuration: Configuration,
  pub database: Database,
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
    let rfid_reader = RfidReader::new(logger.clone(), &configuration, database.clone());
    info!(logger, "Initialized");
    Ok(GreatManager {
      configuration: configuration,
      database: database,
      rfid_reader: rfid_reader,
      root_logger: root_logger,
    })
  }

  pub fn start(&mut self) {
    info!(self.root_logger(), "Starting...");
    self.rfid_reader.start();
    loop {
      thread::sleep(Duration::from_millis(1000));
    }
  }

  pub fn root_logger(&self) -> &slog::Logger {
    &self.root_logger.root_logger
  }
}
