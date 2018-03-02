use std::thread;
use std::sync::mpsc;
use std::sync::mpsc::{ Sender, Receiver, RecvTimeoutError };
use std::time::{ Duration, Instant };

use key_mapper::KeyMapper;

const RFID_WAIT_TIMEOUT_MS: u64 = 1000;
const RFID_READ_TIMEOUT_MS: u64 = 120;
const PIN_KEY_BUFFER_STALE_TIMEOUT_SEC: u64 = 15;

pub struct RfidBuffer<'a> {
  rx: Receiver<u8>,
  bit_buffer: Vec<u8>,
  pin_key_buffer_last_add_dt: Instant,
  pin_key_buffer: Vec<&'a str>,
  key_mapper: KeyMapper<'a>
}

impl<'a> RfidBuffer<'a> {
  pub fn new(rx: Receiver<u8>) -> RfidBuffer<'a> {
    let bit_buffer =  Vec::new();
    let pin_key_buffer = Vec::new();
    let pin_key_buffer_last_add_dt =  Instant::now();
    let key_mapper = KeyMapper::new();
    RfidBuffer {
      rx: rx,
      bit_buffer: bit_buffer,
      pin_key_buffer: pin_key_buffer,
      pin_key_buffer_last_add_dt: pin_key_buffer_last_add_dt,
      key_mapper: key_mapper
    }
  }

  pub fn start(&mut self) {
    loop {
      self.wait_for_data();
    }
  }

  fn wait_for_data(&mut self) {
    let timeout = Duration::from_millis(RFID_WAIT_TIMEOUT_MS);
    match self.rx.recv_timeout(timeout) {
      Ok(bit) => {
        self.add_bit(bit);
        self.read_data();
      },
      Err(RecvTimeoutError::Timeout) => {
        println!("TIMEOUT: BITS {:?}, PIN KEYS: {:?}", self.bits(), self.pin_keys());
        if self.pin_key_buffer_is_stale() {
          self.clear_pin_key_buffer()
        }
      },
      Err(err) =>  println!("Buffer Error: {:?}", err)
    }
  }

  fn read_data(&mut self) {
    let timeout = Duration::from_millis(RFID_READ_TIMEOUT_MS);
    loop {
      match self.rx.recv_timeout(timeout) {
        Ok(bit) => self.add_bit(bit),
        Err(RecvTimeoutError::Timeout) => {
          println!("Buffer Timeout: trying to match {}", self.bits());
          match self.key_mapper.key(&self.bits()) {
            Some("#") => {
              println!("AUTHORIZE PIN: {:?}", self.pin_keys());
              self.clear_pin_key_buffer();
            },
            Some("*") => {
              println!("CLEAR PIN");
              self.clear_pin_key_buffer();
            },
            Some(pin_key) => {
              self.add_pin_key(pin_key);
              println!("PIN {:?}", self.pin_key_buffer);
            },
            None => {
              println!("AUTHORIZE RFID: {:?}", self.bits());
              self.clear_pin_key_buffer();
            }
          }
          self.clear_bit_buffer();
          break;
        },
        Err(err) => {
          println!("Buffer Error: {:?}", err);
          break;
        }
      }
      println!("BITS {:?}, PIN KEYS: {:?}", self.bits(), self.pin_keys());
    }
  }

  fn add_bit(&mut self, bit: u8) {
    self.bit_buffer.push(bit);
  }

  fn bits(&self) -> String {
    self.bit_buffer.iter()
      .map(|bit| bit.to_string())
      .collect::<String>()
  }

  fn pin_key_buffer_is_stale(&self) -> bool {
    if self.pin_key_buffer.is_empty() { return false }
    let timeout = Duration::from_secs(PIN_KEY_BUFFER_STALE_TIMEOUT_SEC);
    self.pin_key_buffer_last_add_dt.elapsed() > timeout
  }

  fn clear_bit_buffer(&mut self) {
    println!("CLEAR BIT BUFFER");
    self.bit_buffer.clear();
  }

  fn add_pin_key(&mut self, pin_key: &'a str) {
    self.pin_key_buffer.push(pin_key);
    self.pin_key_buffer_last_add_dt = Instant::now();
  }

  fn pin_keys(&self) -> String {
    self.pin_key_buffer.concat()
  }

  fn clear_pin_key_buffer(&mut self) {
    println!("CLEAR PIN KEY BUFFER");
    self.pin_key_buffer.clear();
  }
}
