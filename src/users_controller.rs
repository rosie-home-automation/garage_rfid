use diesel::prelude::*;
use gotham::http::response::create_response;
use gotham::state::State;
use hyper::{Response, StatusCode};
use mime;
use serde_json;
use slog;

use database::Database;
use models::user::User;
use schema::users::dsl::*;

pub struct UsersController {
  database: Database,
  logger: slog::Logger,
}

impl UsersController {
  pub fn new(database: Database, logger: slog::Logger) -> UsersController {
    UsersController {
      database: database,
      logger: logger,
    }
  }

  pub fn index(&self, state: State) -> (State, Response) {
    let connection = self.connection();
    let users_result = users.load::<User>(&connection);
    match users_result {
      Ok(user_list) => {
        let json_users_result = serde_json::to_string(&user_list);
        match json_users_result {
          Ok(json_users) => {
            let response = create_response(
              &state,
              StatusCode::Ok,
              Some((json_users.into_bytes(), mime::APPLICATION_JSON)),
            );
            (state, response)
          },
          Err(err) => {
            error!(self.logger, "Error converting users to json."; "err" => ?err); // TODO: Add trace id
            let response = create_response(
              &state,
              StatusCode::InternalServerError,
              Some((vec![], mime::APPLICATION_JSON)),
            );
            (state, response)
          }
        }
      },
      Err(err) => {
        error!(self.logger, "Error loading users."; "err" => ?err); // TODO: Add trace id
        let response = create_response(
          &state,
          StatusCode::InternalServerError,
          Some((vec![], mime::APPLICATION_JSON)),
        );
        (state, response)
      }
    }
  }

  fn connection(&self) -> SqliteConnection {
    self.database.connection()
  }
}
