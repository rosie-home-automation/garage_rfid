/**
 * An example to work out collecting the inputs within a thread and taking the proper actions on
 * them.
 * This example does not require a raspberry pi.
 */

use std::thread;
use std::sync::mpsc;
use std::sync::mpsc::{Sender, Receiver, RecvTimeoutError};
use std::time::Duration;

use std::collections::HashMap;

struct KeyMapper<'a> {
  mapping: HashMap<&'a str, &'a str>
}

impl<'a> KeyMapper<'a> {
  pub fn new() -> KeyMapper<'a> {
    let mut mapping = HashMap::new();
    mapping.insert("00011110", "1");
    mapping.insert("00101101", "2");
    mapping.insert("00111100", "3");
    mapping.insert("01001011", "4");
    mapping.insert("01011010", "5");
    mapping.insert("01101001", "6");
    mapping.insert("01111000", "7");
    mapping.insert("10000111", "8");
    mapping.insert("10010110", "9");
    mapping.insert("00001111", "0");
    mapping.insert("10100101", "*");
    mapping.insert("10110100", "#");
    KeyMapper { mapping: mapping }
  }

  pub fn key(&self, key: &str) -> Option<&'a str> {
    match self.mapping.get(key) {
      Some(value) => Some(value),
      None => None
    }
  }
}

struct RfidReader<'a> {
  rx: Receiver<u8>,
  bit_buffer: Vec<u8>,
  pin_key_buffer: Vec<&'a str>,
  key_mapper: KeyMapper<'a>
}

impl<'a> RfidReader<'a> {
  pub fn new(rx: Receiver<u8>) -> RfidReader<'a> {
    let bit_buffer =  Vec::new();
    let pin_key_buffer = Vec::new();
    let key_mapper = KeyMapper::new();
    RfidReader {
      rx: rx,
      bit_buffer: bit_buffer,
      pin_key_buffer: pin_key_buffer,
      key_mapper: key_mapper
    }
  }

  pub fn start(&mut self) {
    loop {
      read_data();
    }
  }

  fn read_data(&self) {
    match self.rx.recv_timeout(Duration::from_millis(1000)) {
      Ok(bit) => self.add_bit(bit),
      Err(RecvTimeoutError::Timeout) => {
        let value: String =
        println!("Buffer Timeout: trying to match {}", self.bits());
        match self.key_mapper.key(&self.bits()) {
          Some("#") => {
            println!("AUTHORIZE PIN: {:?}", self.pin_key_buffer);
            self.clear_bit_buffer();
          },
          Some("*") => {
            println!("CLEAR");
            self.clear_bit_buffer();
          },
          Some(digit) => {
            self.clear_bit_buffer();
            self.pin_key_buffer.push(digit);
            println!("DATA {:?}", self.pin_key_buffer);
          },
          None => {
            println!("AUTHORIZE RFID: {:?}", self.bit_buffer);
            self.clear_bit_buffer();
          }
        }
      },
      Err(err) =>  println!("Buffer Error: {:?}", err)
    }
    println!("BIT BUFFER {:?}", self.bit_buffer);
  }

  fn add_bit(&mut self, bit: u8) {
    self.bit_buffer.push(bit);
  }

  fn bits(&self) -> String {
    self.bit_buffer.iter()
      .map(|bit| bit.to_string())
      .collect();
  }

  fn add_pin_key(&mut self, pin_key: &str) {
    self.pin_key_buffer.push(pin_key)
  }

  fn clear_bit_buffer(&mut self) {
    self.bit_buffer.clear();
  }
}

fn buffer() -> Result<(), std::sync::mpsc::SendError<u8>> {
  let (tx, rx): (Sender<u8>, Receiver<u8>) = mpsc::channel();
  let data_thread = thread::spawn(move || {
    let mut rfid_reader = RfidReader::new(rx);
    rfid_reader.start();
  });

  // 1 - 00011110
  tx.send(0)?;
  tx.send(0)?;
  tx.send(0)?;
  tx.send(1)?;
  tx.send(1)?;
  tx.send(1)?;
  tx.send(1)?;
  tx.send(0)?;

  thread::sleep(Duration::from_millis(700));
  // 2 - 00101101
  tx.send(0)?;
  tx.send(0)?;
  tx.send(1)?;
  tx.send(0)?;
  tx.send(1)?;
  tx.send(1)?;
  tx.send(0)?;
  tx.send(1)?;

  thread::sleep(Duration::from_millis(700));
  // 3 - 00111100
  tx.send(0)?;
  tx.send(0)?;
  tx.send(1)?;
  tx.send(1)?;
  tx.send(1)?;
  tx.send(1)?;
  tx.send(0)?;
  tx.send(0)?;

  thread::sleep(Duration::from_millis(700));
  // # - 10110100
  tx.send(1)?;
  tx.send(0)?;
  tx.send(1)?;
  tx.send(1)?;
  tx.send(0)?;
  tx.send(1)?;
  tx.send(0)?;
  tx.send(0)?;
  // drop(tx);

  data_thread.join().unwrap();

  Ok(())
}

fn main() {
  match buffer() {
    Ok(()) => println!("Done!"),
    Err(err) => println!("Main Error {:?}", err)
  }
}
