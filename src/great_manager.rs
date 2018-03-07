use config;
use configuration::Configuration;
use database::Database;
use rfid_reader::RfidReader;
use models::user::User;
use std::thread;
use std::time::Duration;

pub struct GreatManager {
  pub configuration: Configuration,
  pub database: Database,
  pub rfid_reader: RfidReader
}

impl GreatManager {
  pub fn new() -> Result<GreatManager, config::ConfigError> {
    let configuration = Configuration::new()?;
    let rfid_reader = RfidReader::new(&configuration);
    let database = Database::new(&configuration);
    Ok(GreatManager { configuration, rfid_reader, database })
  }

  pub fn start(&mut self) {
    self.rfid_reader.start();
    self.create_user();
    loop {
      thread::sleep(Duration::from_millis(1000));
    }
  }

  fn create_user(&self) {
    let conn = self.database.connection();
    let new_user = User::new("Taco".to_string());
    let user = new_user.save(&conn);
    println!("New user {:?}", user);
  }
}
