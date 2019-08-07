use diesel;
use gotham::state::{FromState, State};
use r2d2;
use r2d2_diesel;
use slog;
use std::sync::{Arc, Mutex};

use diesel_middleware::DieselMiddleware;
use garage_door::GarageDoor;
use garage_door_middleware::GarageDoorMiddleware;
use logger_middleware::LoggerMiddleware;

pub struct ControllerHelper;

impl ControllerHelper {

  pub fn connection(state: &State)
    -> r2d2::PooledConnection<r2d2_diesel::ConnectionManager<diesel::SqliteConnection>>
  {
    let diesel_middleware = DieselMiddleware::borrow_from(&state);
    diesel_middleware.pool.get().expect("Expected a connection")
  }

  pub fn garage_door(state: &State) -> Arc<Mutex<GarageDoor>> {
    let garage_door_middleware = GarageDoorMiddleware::borrow_from(&state);
    garage_door_middleware.garage_door.clone()
  }

  pub fn logger(state: &State) -> slog::Logger {
    let logger_middleware = LoggerMiddleware::borrow_from(&state);
    logger_middleware.logger.clone()
  }
}
