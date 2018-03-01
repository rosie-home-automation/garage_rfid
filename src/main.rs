#[cfg(feature = "mio-evented")]
extern crate mio;
#[cfg(feature = "mio-evented")]
extern crate sysfs_gpio;

#[cfg(feature = "mio-evented")]
mod rfid_reader;
#[cfg(feature = "mio-evented")]
mod key_mapper;
#[cfg(feature = "mio-evented")]
mod rfid_buffer;

use rfid_reader::RfidReader;

#[cfg(feature = "mio-evented")]
fn rfid_reader() {
  let mut reader = RfidReader::new();
  reader.start();
}

#[cfg(feature = "mio-evented")]
fn main() {
  rfid_reader();
}

#[cfg(not(feature = "mio-evented"))]
fn main() {
  println!("This example requires the `mio-evented` feature to be enabled.");
}
