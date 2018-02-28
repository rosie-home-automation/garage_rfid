#[cfg(feature = "mio-evented")]
extern crate mio;
#[cfg(feature = "mio-evented")]
extern crate sysfs_gpio;

#[cfg(feature = "mio-evented")]
use mio::{Events, Poll, PollOpt, Ready, Token};
#[cfg(feature = "mio-evented")]
use sysfs_gpio::{AsyncPinPoller, Direction, Edge, Pin};
use std::thread;
use std::time::Duration;

#[derive(Debug)]
enum StreamError {
  Io(std::io::Error),
  SysfsGpio(sysfs_gpio::Error),
}
type StreamResult = Result<(), StreamError>;

impl From<std::io::Error> for StreamError {
  fn from(err: std::io::Error) -> StreamError {
    StreamError::Io(err)
  }
}

impl From<sysfs_gpio::Error> for StreamError {
  fn from(err: sysfs_gpio::Error) -> StreamError {
    StreamError::SysfsGpio(err)
  }
}

const TOKEN_0: Token = Token(0);
const TOKEN_1: Token = Token(1);

#[cfg(feature = "mio-evented")]
fn stream() -> StreamResult {
  let pin_nums = [(17, TOKEN_0), (27, TOKEN_1)];
  // let pin_nums = [(17, TOKEN_0)];
  let pins: Vec<_> = pin_nums.iter()
    .map(|&(pin_num, token)| (token, Pin::new(pin_num)))
    .collect();

  let poll = Poll::new()?;
  let async_pin_pollers: Vec<AsyncPinPoller> = pins.iter()
    .map(|&(token, ref pin)| {
      println!("Setting up gpio{}", pin.get_pin_num());
      pin.export().unwrap();
      thread::sleep(Duration::from_millis(100)); // Delay for sysfs to export the files
      pin.set_direction(Direction::In).unwrap();
      pin.set_edge(Edge::FallingEdge).unwrap();

      let async_pin_poller = pin.get_async_poller().unwrap();
      poll.register(&async_pin_poller, token, Ready::readable(), PollOpt::edge()).unwrap();
      async_pin_poller // Need to keep a valid reference to the async poller
    })
    .collect();

  let mut events = Events::with_capacity(1024);
  loop {
    poll.poll(&mut events, Some(Duration::from_millis(1000)))?;
    for event in &events {
      match event.token() {
        Token(i) => {
          // data.push(i as u8);
        }
      }
    }
  }
}

#[cfg(feature = "mio-evented")]
fn main() {
  match stream() {
    Ok(()) => println!("Stream complete"),
    Err(err) => println!("Error: {:?}", err),
  }
}

#[cfg(not(feature = "mio-evented"))]
fn main() {
  println!("This example requires the `mio-evented` feature to be enabled.");
}
