use config;
use configuration::Configuration;
use rfid_reader::RfidReader;
use std::thread;
use std::time::Duration;

pub struct GreatManager {
  pub configuration: Configuration,
  pub rfid_reader: RfidReader
}

impl GreatManager {
  pub fn new() -> Result<GreatManager, config::ConfigError> {
    let configuration = Configuration::new()?;
    let rfid_reader = RfidReader::new(&configuration);
    Ok(GreatManager { configuration, rfid_reader })
  }

  pub fn start(&mut self) {
    self.rfid_reader.start();
    loop {
      thread::sleep(Duration::from_millis(1000));
    }
  }
}
