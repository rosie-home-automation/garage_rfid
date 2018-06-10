use futures::{Future, Stream};
use slog;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;
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
  opener: sysfs_gpio::Pin,
  open_led: Arc<sysfs_gpio::Pin>,
  closed_led: Arc<sysfs_gpio::Pin>,
  logger: slog::Logger,
  is_open: Arc<AtomicBool>,
}

impl GarageDoor {
  pub fn new(logger: slog::Logger, configuration: &Configuration) -> Self {
    let sensor_gpio = configuration.garage_door.sensor_gpio;
    let opener_gpio = configuration.garage_door.opener_gpio;
    let open_led_gpio = configuration.garage_door.open_led_gpio;
    let closed_led_gpio = configuration.garage_door.closed_led_gpio;
    let opener = GpioUtil::setup_output_pin(opener_gpio);
    let open_led = GpioUtil::setup_output_pin(open_led_gpio);
    let open_led = Arc::new(open_led);
    let closed_led = GpioUtil::setup_output_pin(closed_led_gpio);
    let closed_led = Arc::new(closed_led);
    let async_pin_pollers = Vec::new();
    let is_open = Arc::new(AtomicBool::new(true));
    let mut garage_door = GarageDoor {
      sensor_gpio: sensor_gpio,
      opener_gpio: opener_gpio,
      open_led_gpio: open_led_gpio,
      closed_led_gpio: closed_led_gpio,
      opener: opener,
      open_led: open_led,
      closed_led: closed_led,
      async_pin_pollers: async_pin_pollers,
      logger: logger,
      is_open: is_open,
    };
    garage_door.setup_logger();
    garage_door
  }

  pub fn start(&self) {
    self.setup_sensor();
  }

  fn setup_sensor(&self) {
    let sensor_gpio = self.sensor_gpio;
    let logger = self.logger.clone();
    let is_open = self.is_open.clone();
    let open_led = self.open_led.clone();
    let closed_led = self.closed_led.clone();
    let _executor_thread = thread::spawn(move || {
      let mut l = Core::new().expect("New tokio core.");
      let handle = l.handle();
      let is_open = is_open.clone();
      info!(logger, "Setting up sensor gpio"; "sensor_gpio" => sensor_gpio);
      let sensor_pin = GpioUtil::setup_input_pin(sensor_gpio, sysfs_gpio::Edge::BothEdges);
      handle.spawn(sensor_pin.get_value_stream(&handle)
        .expect("Expected to get sensor pin value stream.")
        .for_each(move |raw_sensor_value| {
          let sensor_value = raw_sensor_value == 1;
          println!("Sensor pin changed value to {}", sensor_value);
          let is_open = is_open.clone();
          let is_open_value = is_open.load(Ordering::Relaxed);
          if is_open_value != sensor_value {
            is_open.store(sensor_value, Ordering::Relaxed);
            match sensor_value {
              true => {
                open_led.set_value(0).unwrap();
                closed_led.set_value(1).unwrap();
              },
              false => {
                open_led.set_value(1).unwrap();
                closed_led.set_value(0).unwrap();
              }
            }
          }
          Ok(())
        })
        .map_err(|_| ())
      );

      loop {
        l.turn(None)
      }
    });
  }

  pub fn status(&self) -> String {
    match self.is_open() {
      true => "open".to_string(),
      false => "closed".to_string(),
    }
  }

  pub fn open(&self) {
    info!(self.logger, "Open triggered.");
    if !self.is_open() {
      self.trigger_opener();
    }
  }

  pub fn close(&self) {
    info!(self.logger, "Close triggered.");
    if self.is_open() {
      self.trigger_opener();
    }
  }

  pub fn toggle(&self) {
    info!(self.logger, "Toggle triggered.");
    self.trigger_opener();
  }

  fn trigger_opener(&self) {
    self.opener.set_value(1).unwrap();
    thread::sleep(Duration::from_millis(100));
    self.opener.set_value(0).unwrap();
  }

  fn setup_logger(&mut self) {
    self.logger = self.logger.new(o!("garage_door" => format!("{:?}", self)));
  }

  fn is_open(&self) -> bool {
    self.is_open.load(Ordering::Relaxed)
  }
}
