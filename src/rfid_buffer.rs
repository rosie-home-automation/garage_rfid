use diesel;
use slog;
use std::sync::{ mpsc, Arc, Mutex };
use std::time::{ Duration, Instant };

use bouncer::{ Bouncer, Variety };
use database::Database;
use key_mapper::KeyMapper;
use slacker::Slacker;

#[derive(Debug)]
pub struct RfidBuffer<'a> {
  rx: mpsc::Receiver<u8>,
  bit_buffer: Vec<u8>,
  pin_key_buffer: Vec<&'a str>,
  pin_key_buffer_last_add_dt: Instant,
  key_mapper: KeyMapper<'a>,
  wait_timeout: Duration,
  read_timeout: Duration,
  pin_key_timeout: Duration,
  bouncer: Bouncer,
  database: Database,
  logger: slog::Logger,
}

impl<'a> RfidBuffer<'a> {
  pub fn new(
    logger: slog::Logger,
    database: Database,
    rx: mpsc::Receiver<u8>,
    wait_timeout_ms: usize,
    read_timeout_ms: usize,
    pin_key_timeout_secs: usize,
    slacker: Arc<Mutex<Slacker>>,
  ) -> RfidBuffer<'a> {
    let bit_buffer =  Vec::new();
    let pin_key_buffer = Vec::new();
    let pin_key_buffer_last_add_dt =  Instant::now();
    let key_mapper = KeyMapper::new();
    let wait_timeout = Duration::from_millis(wait_timeout_ms as u64);
    let read_timeout = Duration::from_millis(read_timeout_ms as u64);
    let pin_key_timeout = Duration::from_secs(pin_key_timeout_secs as u64);
    let connection = database.connection();
    let bouncer = Bouncer::new(connection, slacker.clone());
    let mut rfid_buffer = RfidBuffer {
      rx: rx,
      bit_buffer: bit_buffer,
      pin_key_buffer: pin_key_buffer,
      pin_key_buffer_last_add_dt: pin_key_buffer_last_add_dt,
      key_mapper: key_mapper,
      wait_timeout: wait_timeout,
      read_timeout: read_timeout,
      pin_key_timeout: pin_key_timeout,
      bouncer: bouncer,
      database: database,
      logger: logger
    };
    rfid_buffer.setup_logger();
    rfid_buffer
  }

  pub fn start(&mut self) {
    info!(self.logger, "Starting...");
    loop {
      self.wait_for_data();
    }
  }

  fn wait_for_data(&mut self) {
    match self.rx.recv_timeout(self.wait_timeout) {
      Ok(bit) => {
        self.add_bit(bit);
        self.read_data();
      },
      Err(mpsc::RecvTimeoutError::Timeout) => {
        // info!(self.logger, "Wait timeout"; "bits" => self.bits(), "pin_keys" => self.pin_keys());
        if self.pin_key_buffer_is_stale() {
          self.clear_pin_key_buffer()
        }
      },
      Err(err) => error!(self.logger, "Buffer error"; "err" => format!("{:?}", err))
    }
  }

  fn read_data(&mut self) {
    'read_data_loop: loop {
      match self.rx.recv_timeout(self.read_timeout) {
        Ok(bit) => self.add_bit(bit),
        Err(mpsc::RecvTimeoutError::Timeout) => {
          self.process_bit_buffer();
          break 'read_data_loop;
        },
        Err(err) => {
          error!(self.logger, "Buffer Error"; "err" => format!("{:?}", err));
          break 'read_data_loop;
        }
      }
      // info!(self.logger, "Loop"; "bits" => self.bits(), "pin_keys" => self.pin_keys());
    }
  }

  fn process_bit_buffer(&mut self) {
    info!(self.logger, "Buffer timeout: trying to match"; "bits" => self.bits());
    if self.bits().len() < 8 {
      info!(self.logger, "TODO: Ignore if there are less than 8 bits");
    }
    match self.key_mapper.key(&self.bits()) {
      Some("#") => {
        self.authorize_pin();
      },
      Some("*") => {
        self.clear_pin_key_buffer();
      },
      Some(pin_key) => {
        self.add_pin_key(pin_key);
      },
      None => {
        self.authorize_rfid();
      }
    }
    self.clear_bit_buffer();
  }

  fn authorize_pin(&mut self) {
    info!(self.logger, "Authorize pin"; "pin_keys" => self.pin_keys());
    let result = self.bouncer.is_authorized(self.logger.clone(), Variety::PIN, &self.pin_keys());
    self.process_authorization(result);
  }

  fn authorize_rfid(&mut self) {
    info!(self.logger, "Authorize rfid"; "bits" => self.bits());
    let result = self.bouncer.is_authorized(self.logger.clone(), Variety::RFID, &self.bits());
    self.process_authorization(result);
  }

  fn process_authorization(&mut self, result: Result<bool, diesel::result::Error>) {
    match result {
      Ok(true) => {
        info!(self.logger, "TODO: Open the garage door!");
      },
      Err(err) => {
        error!(self.logger, "Error authorizing!"; "err" => ?err);
      },
      _ => {}
    }
    self.clear_pin_key_buffer();
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
    self.pin_key_buffer_last_add_dt.elapsed() > self.pin_key_timeout
  }

  fn clear_bit_buffer(&mut self) {
    info!(self.logger, "Clear bit buffer");
    self.bit_buffer.clear();
  }

  fn add_pin_key(&mut self, pin_key: &'a str) {
    self.pin_key_buffer.push(pin_key);
    self.pin_key_buffer_last_add_dt = Instant::now();
    info!(self.logger, "Add pin key"; "pin_keys" => self.pin_keys());
  }

  fn pin_keys(&self) -> String {
    self.pin_key_buffer.concat()
  }

  fn clear_pin_key_buffer(&mut self) {
    info!(self.logger, "Clear pin key buffer");
    self.pin_key_buffer.clear();
  }

  fn setup_logger(&mut self) {
    self.logger = self.logger.new(o!("rfid_buffer" => format!("{:?}", self)));
  }
}
