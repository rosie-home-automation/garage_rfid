#[cfg(feature = "tokio")]
extern crate futures;
#[cfg(feature = "tokio")]
extern crate sysfs_gpio;
#[cfg(feature = "tokio")]
extern crate tokio_core;

use std::thread;
#[cfg(feature = "tokio")]
use futures::{Future, Stream, Sink};
#[cfg(feature = "tokio")]
use sysfs_gpio::{Direction, Edge, Pin};
#[cfg(feature = "tokio")]
use tokio_core::reactor::Core;
#[cfg(feature = "tokio")]
use futures::sync::mpsc;

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

#[cfg(feature = "tokio")]
fn stream() -> StreamResult {
  let pin_nums = [(17, 0), (27, 1)];
  let pins: Vec<_> = pin_nums.iter()
    .map(|&(pin_num, value)| (value, Pin::new(pin_num)))
    .collect();
  let mut core = Core::new()?;
  let handle = core.handle();
  let (tx_master, mut rx) = mpsc::channel(50);
  for &(value, ref pin) in pins.iter() {
    println!("Spawning {}", pin.get_pin_num());
    let tx = tx_master.clone();
    pin.export()?;
    pin.set_direction(Direction::In)?;
    pin.set_edge(Edge::FallingEdge)?;
    handle.spawn(pin.get_stream(&handle)?
      .for_each(move |_| {
        println!("PPP{}", value);
        tx.clone()
          .send(value)
          .then(|tx|
            match tx {
              Ok(_tx) => {
                println!("Sink flushed!");
                Ok(())
              }
              Err(err) => {
                println!("Error: {:?}", err);
                Err(err)
              }
            }
          );
        Ok(())
      })
      .map_err(|_| ())
    );
  }

  // thread::spawn(move || {
    loop {
      println!("Loop");
      core.turn(Some(std::time::Duration::from_millis(1000)));
      // rx.for_each(|value| {
      //     println!("VAL{}", value);
      //   });
      match rx.poll() {
        Ok(value) => {
          println!("P{:?}", value);
          ()
        }
        Err(_) => ()
      }
      // Ok(())
    }
  // }

  // core.run(r2).unwrap();
  // Ok(())
}

#[cfg(feature = "tokio")]
fn main() {
  match stream() {
    Ok(()) => println!("Stream complete"),
    Err(err) => println!("Error: {:?}", err),
  }
}

#[cfg(not(feature = "tokio"))]
fn main() {
  println!("This example requires the `tokio` feature to be enabled.");
}
