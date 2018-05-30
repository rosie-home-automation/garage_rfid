use mio;
use slog;
use std::thread;
use std::time::Duration;
use std::sync::mpsc;
use sysfs_gpio;

use configuration::Configuration;
use database::Database;
use gpio_util::GpioUtil;
use rfid_buffer::RfidBuffer;

#[derive(Debug)]
pub struct RfidReader {
  data_0_gpio: usize,
  data_1_gpio: usize,
  _green_led_gpio: usize,
  _red_led_gpio: usize,
  wait_timeout_ms: usize,
  read_timeout_ms: usize,
  pin_key_timeout_secs: usize,
  async_pin_pollers: Vec<sysfs_gpio::AsyncPinPoller>,
  logger: slog::Logger,
  database: Database,
}

impl RfidReader {
  pub fn new(logger: slog::Logger, configuration: &Configuration, database: Database)
    -> RfidReader
  {
    let data_0_gpio = configuration.rfid_reader.data_0_gpio;
    let data_1_gpio = configuration.rfid_reader.data_1_gpio;
    let _green_led_gpio = configuration.rfid_reader.green_led_gpio;
    let _red_led_gpio = configuration.rfid_reader.red_led_gpio;
    let wait_timeout_ms = configuration.rfid_reader.wait_timeout_ms;
    let read_timeout_ms = configuration.rfid_reader.read_timeout_ms;
    let pin_key_timeout_secs = configuration.rfid_reader.pin_key_timeout_secs;
    let async_pin_pollers = Vec::new();
    let mut rfid_reader = RfidReader {
      data_0_gpio,
      data_1_gpio,
      _green_led_gpio,
      _red_led_gpio,
      wait_timeout_ms,
      read_timeout_ms,
      pin_key_timeout_secs,
      async_pin_pollers,
      logger: logger,
      database,
    };
    rfid_reader.setup_logger();
    rfid_reader
  }

  pub fn start(&mut self) {
    info!(self.logger, "Starting...");
    let poller = mio::Poll::new().unwrap();
    let async_pin_poller_0 = self.setup_data_pin(self.data_0_gpio, 0, &poller); // Need to keep a valid reference to the async poller
    let async_pin_poller_1 = self.setup_data_pin(self.data_1_gpio, 1, &poller); // Need to keep a valid reference to the async poller
    self.async_pin_pollers.push(async_pin_poller_0);
    self.async_pin_pollers.push(async_pin_poller_1);

    let wait_timeout_ms = self.wait_timeout_ms;
    let read_timeout_ms = self.read_timeout_ms;
    let pin_key_timeout_secs = self.pin_key_timeout_secs;
    let (tx, rx): (mpsc::Sender<u8>, mpsc::Receiver<u8>) = mpsc::channel();
    let logger = self.logger.clone();
    let database = self.database.clone();
    let _buffer_thread = thread::spawn(move || {
      let mut rfid_buffer = RfidBuffer::new(
        logger,
        database,
        rx,
        wait_timeout_ms,
        read_timeout_ms,
        pin_key_timeout_secs,
      );
      rfid_buffer.start();
    });

    let _poller_thread = thread::spawn(move || {
      let mut events = mio::Events::with_capacity(1024);
      loop {
        poller.poll(&mut events, Some(Duration::from_millis(1000))).unwrap();
        for event in &events {
          match event.token() {
            mio::Token(i) => tx.send(i as u8).unwrap()
          }
        }
      }
    });
  }

  fn setup_data_pin(&self, pin_num: usize, value: usize, poller: &mio::Poll)
    -> sysfs_gpio::AsyncPinPoller
  {
    info!(self.logger, "Setting up gpio"; "pin_num" => pin_num);
    let pin = GpioUtil::setup_input_pin(pin_num, sysfs_gpio::Edge::FallingEdge);
    let token = mio::Token(value);
    let async_pin_poller = pin.get_async_poller().unwrap();
    poller.register(&async_pin_poller, token, mio::Ready::readable(), mio::PollOpt::edge())
      .unwrap();
    async_pin_poller
  }

  fn setup_logger(&mut self) {
    self.logger = self.logger.new(o!("rfid_logger" => format!("{:?}", self)));
  }
}
