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
  let (tx_master, mut rx) = mpsc::channel(50);
  let t1_tx = tx_master.clone();
  let t2_tx = tx_master.clone();
  core.handle().spawn(|_| {
    let durs = [0, scale, scale, 5 * scale];
    loop {
      for &dur in durs.iter() {
        thread::sleep(time::Duration::from_millis(dur));
        println!("Tx 0");
        t1_tx.clone()
          .send(Ok(0))
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
  core.handle().spawn(|_| {
    let durs = [3 * scale, scale, scale, scale];
    loop {
      for &dur in durs.iter() {
        thread::sleep(time::Duration::from_millis(dur));
        println!("Tx 1");
        t2_tx.clone()
          .send(Ok(1))
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
    Ok(())
  });

  // loop {
  //   match rx.poll() {
  //     value => println!("Rx {:?}", value)
  //   }
  // }

  let r2 = rx.for_each(|value| {
    println!("Rx {:?}", value);
  });
  core.run(r2).unwrap();
  // t1.join();
  // t2.join();
  Ok(())
}

fn main() {
  match stream() {
    Ok(()) => println!("Stream complete"),
    Err(err) => println!("Error: {:?}", err),
  }
}
