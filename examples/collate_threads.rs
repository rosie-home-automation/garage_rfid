/**
 * An example of an early attempt at using Tokio to collect values from 2 producers.
 * Never finished it, possibly didn't keep a reference alive for long enough.
 * This example is not finished.
 * This example doesn't require a raspberry pi.
 */

extern crate futures;
extern crate tokio_core;

use std::{thread, time};
use futures::{Future, Stream, Sink};
use tokio_core::reactor::Core;
use futures::sync::mpsc;

#[derive(Debug)]
enum StreamError {
  Io(std::io::Error)
}
type StreamResult = Result<(), StreamError>;

impl From<std::io::Error> for StreamError {
  fn from(err: std::io::Error) -> StreamError {
    StreamError::Io(err)
  }
}

fn stream() -> StreamResult {
  const scale: u64 = 100;
  let mut core = Core::new()?;
  let (tx_master, rx) = mpsc::channel(50);
  let t1_tx = tx_master.clone();
  let t2_tx = tx_master.clone();
  let t1 = thread::spawn(move || {
    let durs = [0, scale, scale, 5 * scale];
    loop {
      for &dur in durs.iter() {
        thread::sleep(time::Duration::from_millis(dur));
        println!("Sending 0");
        t1_tx.clone()
          .send(0)
          .then(|tx|
            match tx {
              Ok(_tx) => {
                println!("Flushed");
                Ok(())
              }
              Err(err) => {
                println!("Error: {:?}", err);
                Err(err)
              }
            }
          )
          .map_err(|_err| ());
      }
      thread::sleep(time::Duration::from_millis(3000));
    }
  });
  let t2 = thread::spawn(move || {
    let durs = [3 * scale, scale, scale, scale];
    loop {
      for &dur in durs.iter() {
        thread::sleep(time::Duration::from_millis(dur));
        println!("Sending 1");
        t2_tx.clone()
          .send(1)
          .then(|tx|
            match tx {
              Ok(_tx) => {
                println!("Flushed");
                Ok(())
              }
              Err(err) => {
                println!("Error: {:?}", err);
                Err(err)
              }
            }
          )
          .map_err(|_err| ());
      }
      thread::sleep(time::Duration::from_millis(3000 + scale));
    }
  });

  // let r2 = rx.for_each(|value| {
  //   println!("Rx{:?}", value);
  // });

  // core.run(r2).unwrap();
  t1.join();
  t2.join();
  Ok(())
}

fn main() {
  match stream() {
    Ok(()) => println!("Stream complete"),
    Err(err) => println!("Error: {:?}", err),
  }
}
