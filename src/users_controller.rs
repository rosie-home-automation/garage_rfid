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
use std::panic::{catch_unwind, AssertUnwindSafe};

use database::Database;
use models::user::User;
use schema::users::dsl::*;

#[derive(Deserialize, StateData, StaticResponseExtender)]
pub struct UserPathParams {
  pub id: String
}

pub struct UsersController {
  pool: AssertUnwindSafe<r2d2::Pool<r2d2_diesel::ConnectionManager<SqliteConnection>>>,
  logger: slog::Logger,
}

impl UsersController {
  pub fn new(
    pool: r2d2::Pool<r2d2_diesel::ConnectionManager<SqliteConnection>>,
    logger: slog::Logger
  ) -> UsersController {
    let pool = AssertUnwindSafe(pool);
    UsersController {
      pool: pool,
      logger: logger,
    }
  }

  pub fn index(&self, state: State) -> (State, Response) {
    let connection = self.connection();
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

  pub fn show(&self, mut state: State) -> (State, Response) {
    let connection = self.connection();
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
            error!(self.logger, "Error converting user to json."; "err" => ?err); // TODO: Add trace id
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
        error!(self.logger, "Error loading user."; "err" => ?err, "id" => &user_id); // TODO: Add trace id
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

  fn connection(&self)
    -> r2d2::PooledConnection<r2d2_diesel::ConnectionManager<diesel::SqliteConnection>>
  {
    self.pool.get().expect("Could not get database connection.")
  }
}

impl Clone for UsersController {
    fn clone(&self) -> UsersController {
      let pool = AssertUnwindSafe(self.pool.clone());
      let logger = self.logger.clone();
      UsersController {
        pool: pool,
        logger: logger,
      }
    }
}
