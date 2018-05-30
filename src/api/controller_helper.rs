use diesel;
use gotham::state::{FromState, State};
use r2d2;
use r2d2_diesel;
use slog;

use diesel_middleware::DieselMiddleware;
use logger_middleware::LoggerMiddleware;

pub struct ControllerHelper;

impl ControllerHelper {

  pub fn connection(state: &State)
    -> r2d2::PooledConnection<r2d2_diesel::ConnectionManager<diesel::SqliteConnection>>
  {
    let diesel_middleware = DieselMiddleware::borrow_from(&state);
    diesel_middleware.pool.get().expect("Expected a connection")
  }

  pub fn logger(state: &State) -> slog::Logger {
    let logger_middleware = LoggerMiddleware::borrow_from(&state);
    logger_middleware.logger.clone()
  }
}
