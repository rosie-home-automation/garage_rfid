use diesel;
use diesel::prelude::*;
use gotham::http::response::create_response;
use gotham::state::{FromState, State};
use gotham::handler::{HandlerFuture, IntoHandlerError};
use futures::{future, Future, Stream};
use hyper::{Body, Response, StatusCode};
use mime;
use r2d2;
use r2d2_diesel;
use serde_json;
use slog;

use diesel_middleware::DieselMiddleware;
use logger_middleware::LoggerMiddleware;
use models::user::User;
use schema::users::dsl::*;

#[derive(Deserialize, StateData, StaticResponseExtender)]
pub struct UserPathParams {
  pub id: String
}

pub struct UsersController;

impl UsersController {
  pub fn index(state: State) -> (State, Response) {
    let connection = UsersController::connection(&state);
    let logger = UsersController::logger(&state);
    let users_result = users.load::<User>(&*connection);
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
            error!(logger, "Error converting users to json."; "err" => ?err); // TODO: Add trace id
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
        error!(logger, "Error loading users."; "err" => ?err); // TODO: Add trace id
        let response = create_response(
          &state,
          StatusCode::InternalServerError,
          Some((vec![], mime::APPLICATION_JSON)),
        );
        (state, response)
      }
    }
  }

  pub fn show(mut state: State) -> (State, Response) {
    let connection = UsersController::connection(&state);
    let logger = UsersController::logger(&state);
    let UserPathParams { id: user_id } = UserPathParams::take_from(&mut state);
    let user_response = users.find(&user_id).first::<User>(&*connection);
    match user_response {
      Ok(user) => {
        let user_json_result = serde_json::to_string(&user);
        match user_json_result {
          Ok(user_json) => {
            let response = create_response(
              &state,
              StatusCode::Ok,
              Some((user_json.into_bytes(), mime::APPLICATION_JSON)),
            );
            (state, response)
          },
          Err(err) => {
            error!(logger, "Error converting user to json."; "err" => ?err); // TODO: Add trace id
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
        error!(logger, "Error loading user."; "err" => ?err, "id" => &user_id); // TODO: Add trace id
        match err {
          diesel::result::Error::NotFound => {
            let response = create_response(
              &state,
              StatusCode::NotFound,
              Some((vec![], mime::APPLICATION_JSON)),
            );
            (state, response)
          },
          _ => {
            let response = create_response(
              &state,
              StatusCode::InternalServerError,
              Some((vec![], mime::APPLICATION_JSON)),
            );
            (state, response)
          }
        }
      }
    }
  }

  // pub fn create(state: State) -> Box<HandlerFuture> {
  //   let connection = self.connection();
  //   let f = Body::take_from(&mut state)
  //     .concat2()
  //     .then(|full_body|
  //       match full_body {
  //         Ok(valid_body) => {
  //           let body = String::from_utf8(valid_body.to_vec()).unwrap();
  //           info!(self.logger, "Create request"; "body" => body, "valid_body" => ?valid_body);
  //           let response = create_response(
  //             &state,
  //             StatusCode::Ok,
  //             Some((body.into_bytes(), mime::TEXT_PLAIN)),
  //           );
  //           future::ok((state, response))
  //         },
  //         Err(err) => {
  //           error!(self.logger, "Error during create"; "err" => ?err);
  //           future::err((state, err.into_handler_error()))
  //         },
  //       }
  //     );

  //   Box::new(f)
  // }

  fn connection(state: &State) -> r2d2::PooledConnection<r2d2_diesel::ConnectionManager<diesel::SqliteConnection>> {
    let diesel_middleware = DieselMiddleware::borrow_from(&state);
    diesel_middleware.pool.get().expect("Expected a connection")
  }

  fn logger(state: &State) -> slog::Logger {
    let logger_middleware = LoggerMiddleware::borrow_from(&state);
    logger_middleware.logger.clone()
  }
}
