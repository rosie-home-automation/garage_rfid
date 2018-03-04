use config::{ Config, ConfigError, File };

#[derive(Debug, Deserialize)]
pub struct DatabaseConfiguration {
  pub url: String
}

#[derive(Debug, Deserialize)]
pub struct GarageDoorConfiguration {
  pub opener_gpio: usize,
  pub sensor_gpio: usize,
  pub open_led_gpio: usize,
  pub closed_led_gpio: usize
}

#[derive(Debug, Deserialize)]
pub struct RfidReaderConfiguration {
  pub data_0_gpio: usize,
  pub data_1_gpio: usize,
  pub green_led_gpio: usize,
  pub red_led_gpio: usize,
  pub wait_timeout_ms: usize,
  pub read_timeout_ms: usize,
  pub pin_key_timeout_secs: usize
}

#[derive(Debug, Deserialize)]
pub struct Configuration {
  pub database: DatabaseConfiguration,
  pub garage_door: GarageDoorConfiguration,
  pub rfid_reader: RfidReaderConfiguration
}

impl Configuration {
  pub fn new() -> Result<Self, ConfigError> {
    let mut configuration = Config::new();
    configuration.merge(File::with_name("config"))?;
    configuration.try_into()
  }
}
