use mio::{Events, Poll, PollOpt, Ready, Token};
use sysfs_gpio::{AsyncPinPoller, Direction, Edge, Pin};

use std::thread;
use std::time::Duration;
use std::sync::mpsc;
use std::sync::mpsc::{ Sender, Receiver };

use configuration::Configuration;
use rfid_buffer::RfidBuffer;

pub struct RfidReader {
  data_0_gpio: usize,
  data_1_gpio: usize,
  _green_led_gpio: usize,
  _red_led_gpio: usize,
  wait_timeout_ms: usize,
  read_timeout_ms: usize,
  pin_key_timeout_secs: usize,
  async_pin_pollers: Vec<AsyncPinPoller>
}

impl RfidReader {
  pub fn new(configuration: &Configuration) -> RfidReader {
    let data_0_gpio = configuration.rfid_reader.data_0_gpio;
    let data_1_gpio = configuration.rfid_reader.data_1_gpio;
    let _green_led_gpio = configuration.rfid_reader.green_led_gpio;
    let _red_led_gpio = configuration.rfid_reader.red_led_gpio;
    let wait_timeout_ms = configuration.rfid_reader.wait_timeout_ms;
    let read_timeout_ms = configuration.rfid_reader.read_timeout_ms;
    let pin_key_timeout_secs = configuration.rfid_reader.pin_key_timeout_secs;
    let async_pin_pollers = Vec::new();
    RfidReader {
      data_0_gpio,
      data_1_gpio,
      _green_led_gpio,
      _red_led_gpio,
      wait_timeout_ms,
      read_timeout_ms,
      pin_key_timeout_secs,
      async_pin_pollers
    }
  }

  pub fn start(&mut self) {
    let poller = Poll::new().unwrap();
    let async_pin_poller_0 = self.setup_data_pin(self.data_0_gpio, 0, &poller); // Need to keep a valid reference to the async poller
    let async_pin_poller_1 = self.setup_data_pin(self.data_1_gpio, 1, &poller); // Need to keep a valid reference to the async poller
    self.async_pin_pollers.push(async_pin_poller_0);
    self.async_pin_pollers.push(async_pin_poller_1);

    let wait_timeout_ms = self.wait_timeout_ms;
    let read_timeout_ms = self.read_timeout_ms;
    let pin_key_timeout_secs = self.pin_key_timeout_secs;
    let (tx, rx): (Sender<u8>, Receiver<u8>) = mpsc::channel();
    let _buffer_thread = thread::spawn(move || {
      let mut rfid_buffer = RfidBuffer::new(
        rx,
        wait_timeout_ms,
        read_timeout_ms,
        pin_key_timeout_secs
      );
      rfid_buffer.start();
    });

    let _poller_thread = thread::spawn(move || {
      let mut events = Events::with_capacity(1024);
      loop {
        poller.poll(&mut events, Some(Duration::from_millis(1000))).unwrap();
        for event in &events {
          match event.token() {
            Token(i) => tx.send(i as u8).unwrap()
          }
        }
      }
    });
  }

  fn setup_data_pin(&self, pin_num: usize, value: usize, poller: &Poll) -> AsyncPinPoller {
    println!("Setting up gpio{}", pin_num);
    let pin = Pin::new(pin_num as u64);
    let token = Token(value);
    pin.export().unwrap();
    thread::sleep(Duration::from_millis(100)); // Delay for sysfs to export the files
    pin.set_direction(Direction::In).unwrap();
    pin.set_edge(Edge::FallingEdge).unwrap();

    let async_pin_poller = pin.get_async_poller().unwrap();
    poller.register(&async_pin_poller, token, Ready::readable(), PollOpt::edge()).unwrap();
    async_pin_poller
  }
}
