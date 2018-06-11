use slog;
use slog::{ Drain, Duplicate, Fuse };
use slog_async;
use slog_json;
use slog_term;
use std::fs::OpenOptions;
use std::sync::Mutex;

use configuration::Configuration;

pub struct RootLogger {
  pub level: slog::Level,
  pub log_file_path: String,
  pub root_logger: slog::Logger
}

impl RootLogger {
  pub fn new(configuration: &Configuration) -> RootLogger {
    let log_file_path = configuration.logging.log_file_path.clone();
    let level = RootLogger::log_level(&configuration.logging.level);
    let root_logger = RootLogger::setup_logger(log_file_path.clone());
    RootLogger {
      log_file_path: log_file_path,
      level: level,
      root_logger: root_logger,
    }
  }

  fn log_level(level: &str) -> slog::Level {
    match level {
      "INFO" => slog::Level::Info,
      "WARNING" => slog::Level::Warning,
      "ERROR" => slog::Level::Error,
      "CRITICAL" => slog::Level::Critical,
      _ => slog::Level::Info
    }
  }

  fn setup_logger(log_file_path: String) -> slog::Logger {
    let decorator = slog_term::TermDecorator::new().build();
    let stdout_drain = slog_term::CompactFormat::new(decorator).build().fuse();
    let stdout_drain = slog_async::Async::new(stdout_drain).build().fuse();
    let log_file = OpenOptions::new()
      .create(true)
      .write(true)
      .append(true)
      .open(log_file_path)
      .unwrap();
    let json_drain = Mutex::new(slog_json::Json::default(log_file)).map(Fuse);
    slog::Logger::root(
      Duplicate::new(stdout_drain, json_drain).fuse(),
      o!("version" => env!("CARGO_PKG_VERSION"))
    )
  }
}
