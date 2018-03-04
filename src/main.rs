extern crate config;
extern crate serde;

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate error_chain;

extern crate mio;
extern crate sysfs_gpio;

mod errors;
mod configuration;
mod great_manager;
mod key_mapper;
mod rfid_reader;
mod rfid_buffer;

use great_manager::GreatManager;

fn main() {
  match GreatManager::new() {
    Ok(mut great_manager) => great_manager.start(),
    Err(err) => println!("Error {:?}", err)
  }
}
