#[cfg(feature = "mio-evented")]
use mio::{Events, Poll, PollOpt, Ready, Token};
#[cfg(feature = "mio-evented")]
use sysfs_gpio::{AsyncPinPoller, Direction, Edge, Pin};

use std::thread;
use std::time::Duration;
use std::sync::mpsc;
use std::sync::mpsc::{ Sender, Receiver };

use rfid_buffer::RfidBuffer;

#[cfg(feature = "mio-evented")]
pub struct RfidReader {
  pin_configs: Vec<(usize, usize)>
}

#[cfg(feature = "mio-evented")]
impl RfidReader {
  pub fn new() -> RfidReader {
    let pin_configs: Vec<(usize, usize)> = vec![(17, 0), (27, 1)];
    RfidReader { pin_configs: pin_configs }
  }

  pub fn start(&mut self) {
    let poller = Poll::new().unwrap();
    let _async_pin_pollers: Vec<AsyncPinPoller> = self.async_pin_pollers(&poller);
    let (tx, rx): (Sender<u8>, Receiver<u8>) = mpsc::channel();
    let _buffer_thread = thread::spawn(move || {
      let mut rfid_buffer = RfidBuffer::new(rx);
      rfid_buffer.start();
    });

    let mut events = Events::with_capacity(1024);
    loop {
      poller.poll(&mut events, Some(Duration::from_millis(1000))).unwrap();
      for event in &events {
        match event.token() {
          Token(i) => tx.send(i as u8).unwrap()
        }
      }
    }
  }

  fn async_pin_pollers(&self, poller: &Poll) -> Vec<AsyncPinPoller> {
    self.pin_token_pairs().iter()
      .map(|&(ref pin, token)| {
        println!("Setting up gpio{}", pin.get_pin_num());
        pin.export().unwrap();
        thread::sleep(Duration::from_millis(100)); // Delay for sysfs to export the files
        pin.set_direction(Direction::In).unwrap();
        pin.set_edge(Edge::FallingEdge).unwrap();

        let async_pin_poller = pin.get_async_poller().unwrap();
        poller.register(&async_pin_poller, token, Ready::readable(), PollOpt::edge()).unwrap();
        async_pin_poller // Need to keep a valid reference to the async poller
      })
      .collect::<Vec<AsyncPinPoller>>()
  }

  fn pin_token_pairs(&self) -> Vec<(Pin, Token)> {
    self.pin_configs.iter()
      .map(|&(pin_num, value)| (Pin::new(pin_num as u64), Token(value)))
      .collect::<Vec<(Pin, Token)>>()
  }
}
