use bus;
use futures::{Future, Stream};
use slog;
use std::fmt::{ Debug, Formatter, Result };
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use sysfs_gpio;
use tokio_core::reactor::Core;

use configuration::Configuration;
use gpio_util::GpioUtil;
use slacker::Slacker;

pub struct GarageDoor {
  sensor_gpio: usize,
  async_pin_pollers: Vec<sysfs_gpio::AsyncPinPoller>,
  opener: sysfs_gpio::Pin,
  open_led: Arc<sysfs_gpio::Pin>,
  closed_led: Arc<sysfs_gpio::Pin>,
  slacker: Arc<Mutex<Slacker>>,
  logger: slog::Logger,
  is_open: Arc<AtomicBool>,
  bus: Arc<Mutex<bus::Bus<String>>>,
}

impl Debug for GarageDoor {
  fn fmt(&self, f: &mut Formatter) -> Result {
    write!(
      f,
      "GarageDoor {{ sensor_gpio: {}, opener: {:?}, open_led: {:?}, closed_led: {:?}, slacker: {:?} }}",
      &self.sensor_gpio, &self.opener, &self.open_led, &self.closed_led, &self.slacker
    )
  }
}

impl GarageDoor {
  pub fn new(logger: slog::Logger, configuration: &Configuration, slacker: Arc<Mutex<Slacker>>)
    -> Self
  {
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
    let bus = bus::Bus::new(10);
    let bus = Arc::new(Mutex::new(bus));
    let mut garage_door = GarageDoor {
      sensor_gpio: sensor_gpio,
      opener: opener,
      open_led: open_led,
      closed_led: closed_led,
      async_pin_pollers: async_pin_pollers,
      slacker: slacker,
      logger: logger,
      is_open: is_open,
      bus: bus,
    };
    garage_door.setup_logger();
    garage_door
  }

  pub fn start(&self) {
    self.setup_sensor();
  }

  pub fn subscribe(&self) -> bus::BusReader<String> {
    self.bus.lock().unwrap().add_rx()
  }

  fn setup_sensor(&self) {
    let sensor_gpio = self.sensor_gpio;
    let logger = self.logger.clone();
    let is_open = self.is_open.clone();
    let open_led = self.open_led.clone();
    let closed_led = self.closed_led.clone();
    let slacker = self.slacker.clone();
    let bus = self.bus.clone();
    let _executor_thread = thread::spawn(move || {
      let mut l = Core::new().expect("New tokio core.");
      let handle = l.handle();
      let is_open = is_open.clone();
      info!(logger, "Setting up sensor gpio"; "sensor_gpio" => sensor_gpio);
      let sensor_pin = GpioUtil::setup_input_pin(sensor_gpio, sysfs_gpio::Edge::BothEdges);
      let bus2 = bus.clone();
      handle.spawn(sensor_pin.get_value_stream()
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
                slacker.lock().unwrap().send_text("Garage door opened", logger.clone());
                bus2.lock().unwrap().broadcast("opened".to_string());
                open_led.set_value(1).unwrap();
                closed_led.set_value(0).unwrap();
              },
              false => {
                slacker.lock().unwrap().send_text("Garage door closed", logger.clone());
                bus2.lock().unwrap().broadcast("closed".to_string());
                open_led.set_value(0).unwrap();
                closed_led.set_value(1).unwrap();
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
    if !self.is_open() {
      self.trigger_opener();
      info!(self.logger, "Open triggered.");
    }
  }

  pub fn close(&self) {
    if self.is_open() {
      self.trigger_opener();
      info!(self.logger, "Close triggered.");
    }
  }

  pub fn toggle(&self) {
    self.trigger_opener();
    info!(self.logger, "Toggle triggered.");
  }

  fn trigger_opener(&self) {
    self.opener.set_value(1).unwrap();
    thread::sleep(Duration::from_millis(100));
    self.opener.set_value(0).unwrap();
  }

  fn setup_logger(&mut self) {
    self.logger = self.logger.new(o!("garage_door" => format!("{:?}", self)));
  }

  pub fn is_open(&self) -> bool {
    self.is_open.load(Ordering::Relaxed)
  }
}
