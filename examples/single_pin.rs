/**
 * An example of synchronously/blockingly polling a single pin. Used to determine if the library
 * worked on the raspberry pi and was nice to use.
 * It requires a raspberry pi.
 */

extern crate sysfs_gpio;

use sysfs_gpio::{Direction, Edge, Pin};

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

fn stream() -> StreamResult {
  let pin_nums = [(17, 0)];
  let pins: Vec<_> = pin_nums.iter()
    .map(|&(pin_num, value)| (value, Pin::new(pin_num)))
    .collect();

  for &(value, ref pin) in pins.iter() {
    println!("Spawning {}", pin.get_pin_num());
    pin.with_exported(|| {
      pin.set_direction(Direction::In)?;
      pin.set_edge(Edge::FallingEdge)?;

      let mut poller = pin.get_poller()?;
      loop {
        match poller.poll(1000)? {
          Some(_) => println!("V{}", value),
          None => println!(".")
        }
      }
    })?;
  }
  Ok(())
}

fn main() {
  match stream() {
    Ok(()) => println!("Stream complete"),
    Err(err) => println!("Error: {:?}", err),
  }
}
