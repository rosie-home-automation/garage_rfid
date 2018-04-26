use slog;
use sysfs_gpio;

use configuration::Configuration;

#[derive(Debug)]
pub struct GarageDoor {
  sensor_gpio: usize,
  opener_gpio: usize,
  open_led_gpio: usize,
  closed_led_gpio: usize,
  async_pin_pollers: Vec<sysfs_gpio::AsyncPinPoller>,
  logger: slog::Logger,
}

impl GarageDoor {
  pub fn new(logger: slog::Logger, configuration: &Configuration) -> Self {
    let sensor_gpio = configuration.garage_door.sensor_gpio;
    let opener_gpio = configuration.garage_door.opener_gpio;
    let open_led_gpio = configuration.garage_door.open_led_gpio;
    let closed_led_gpio = configuration.garage_door.closed_led_gpio;
    let async_pin_pollers = Vec::new();
    let mut garage_door = GarageDoor {
      sensor_gpio: sensor_gpio,
      opener_gpio: opener_gpio,
      open_led_gpio: open_led_gpio,
      closed_led_gpio: closed_led_gpio,
      async_pin_pollers: async_pin_pollers,
      logger: logger,
    };
    garage_door.setup_logger();
    garage_door
  }

  pub fn start(&self) {

  }

  fn setup_logger(&mut self) {
    self.logger = self.logger.new(o!("garage_door" => format!("{:?}", self)));
  }
}
