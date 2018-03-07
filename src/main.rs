extern crate config;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate error_chain;
extern crate mio;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate sysfs_gpio;

mod errors;
mod configuration;
mod database;
mod great_manager;
mod key_mapper;
mod rfid_reader;
mod rfid_buffer;
mod schema;
mod models;

use great_manager::GreatManager;

fn main() {
  println!("START");
  match GreatManager::new() {
    Ok(mut great_manager) => great_manager.start(),
    Err(err) => println!("Error {:?}", err)
  }
  println!("DONE");
}
