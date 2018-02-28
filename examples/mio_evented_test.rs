/**
 * An example that helped me figure out an issue I was having with the mio_evented example.
 * I needed to keep some variables around longer to extend their lifetime.
 * This example requires a raspberry pi.
 */

#[cfg(feature = "mio-evented")]
extern crate mio;
#[cfg(feature = "mio-evented")]
extern crate sysfs_gpio;

#[cfg(feature = "mio-evented")]
use mio::{Events, Poll, PollOpt, Ready, Token};
#[cfg(feature = "mio-evented")]
use mio::unix::EventedFd;
#[cfg(feature = "mio-evented")]
use sysfs_gpio::{Direction, Edge, Pin};
use std::fs::File;
use std::os::unix::io::AsRawFd;
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

  let poll = Poll::new()?;

  let pin1 = Pin::new(17);
  let async_pin_poller1 = pin1.get_async_poller()?;
  // let devfile1 = File::open(&"/sys/class/gpio/gpio17/value")?;
  // // let devfile_fd1 = devfile1.as_raw_fd();
  // let async_pin_poller1 = EventedFd(&devfile1.as_raw_fd());
  poll.register(&async_pin_poller1, TOKEN_0, Ready::readable(), PollOpt::edge())?;

  let devfile2 = File::open(&"/sys/class/gpio/gpio27/value")?;
  let devfile_fd2 = devfile2.as_raw_fd();
  let async_pin_poller2 = EventedFd(&devfile_fd2);
  poll.register(&async_pin_poller2, TOKEN_1, Ready::readable(), PollOpt::edge())?;

  let mut events = Events::with_capacity(1024);
  loop {
    poll.poll(&mut events, Some(Duration::from_millis(1000)))?;
    for event in &events {
      match event.token() {
        Token(i) => {
          println!("V{}", i);
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
