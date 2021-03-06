extern crate bus;
extern crate chrono;
extern crate config;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate error_chain;
extern crate futures;
// extern crate gotham;
// #[macro_use]
// extern crate gotham_derive;
extern crate hyper;
extern crate mime;
extern crate mio;
#[macro_use]
extern crate nanoid;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
extern crate slack_hook;
#[macro_use]
extern crate slog;
extern crate slog_async;
extern crate slog_json;
extern crate slog_term;
extern crate sysfs_gpio;
extern crate tokio_core;
extern crate uuid;
extern crate webthing;

pub mod great_manager;

// pub mod api;
pub mod bouncer;
pub mod configuration;
pub mod database;
// pub mod diesel_middleware;
pub mod errors;
pub mod garage_door;
// pub mod garage_door_middleware;
pub mod gpio_util;
// pub mod http_server;
pub mod iot;
pub mod key_mapper;
// pub mod logger_middleware;
pub mod models;
// pub mod request_logging_middleware;
pub mod rfid_buffer;
pub mod rfid_reader;
pub mod root_logger;
pub mod schema;
pub mod slacker;
// pub mod users_controller;
