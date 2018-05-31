use futures::{Future, Stream};
use slog;
use std::sync::mpsc;
use std::thread;
use sysfs_gpio;
use tokio_core::reactor::Core;

use configuration::Configuration;
use gpio_util::GpioUtil;

#[derive(Debug)]
pub struct GarageDoor {
  sensor_gpio: usize,
  opener_gpio: usize,
  open_led_gpio: usize,
  closed_led_gpio: usize,
  async_pin_pollers: Vec<sysfs_gpio::AsyncPinPoller>,
  logger: slog::Logger,
  tmp_status: bool,
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
      tmp_status: false,
    };
    garage_door.setup_logger();
    garage_door
  }

  pub fn start(&self) {
    // let sensor_gpio = self.sensor_gpio;
    // let logger = self.logger.clone();
    // let _executor_thread = thread::spawn(move || {
    //   let mut l = Core::new().expect("New tokio core.");
    //   let handle = l.handle();
    //   info!(logger, "Setting up sensor gpio"; "sensor_gpio" => sensor_gpio);
    //   let sensor_pin = GpioUtil::setup_input_pin(sensor_gpio, sysfs_gpio::Edge::BothEdges);
    //   handle.spawn(sensor_pin.get_value_stream(&handle)
    //     .expect("Expected to get sensor pin value stream.")
    //     .for_each(move |val| {
    //       println!("Sensor pin changed value to {}", val);
    //       Ok(())
    //     })
    //     .map_err(|_| ())
    //   );

    //   loop {
    //     l.turn(None)
    //   }
    // });
  }

  pub fn status(&self) -> String {
    match self.tmp_status {
      true => "open".to_string(),
      false => "closed".to_string(),
    }
  }

  pub fn open(&mut self) {
    self.tmp_status = true;
  }

  pub fn close(&mut self) {
    self.tmp_status = false;
  }

  pub fn toggle(&mut self) {
    self.tmp_status = !self.tmp_status;
  }

  fn setup_logger(&mut self) {
    self.logger = self.logger.new(o!("garage_door" => format!("{:?}", self)));
  }

  fn is_open(&self) {
    self.tmp_status;
  }
}
