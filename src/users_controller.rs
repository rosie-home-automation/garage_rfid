use diesel;
use diesel::prelude::*;
use gotham::http::response::create_response;
use gotham::state::{FromState, State};
use gotham::handler::{HandlerFuture, IntoHandlerError};
use futures::{future, Future, Stream};
use hyper::{self, Body, Response, StatusCode};
use mime;
use serde_json;
use std;

use api::controller_helper::ControllerHelper;
use api::users_update_action::UsersUpdateAction;
use models::user::User;
use schema::users::dsl::users;

#[derive(Deserialize, StateData, StaticResponseExtender)]
pub struct UserPathParams {
  pub id: String
}

#[derive(Debug, Deserialize)]
struct UserCreateParams {
  name: String
}

#[derive(Debug)]
enum UsersControllerError {
  BodyParseError(std::string::FromUtf8Error),
  HyperError(hyper::Error),
}

impl From<std::string::FromUtf8Error> for UsersControllerError {
  fn from(err: std::string::FromUtf8Error) -> UsersControllerError {
    UsersControllerError::BodyParseError(err)
  }
}

impl From<hyper::Error> for UsersControllerError {
  fn from(err: hyper::Error) -> UsersControllerError {
    UsersControllerError::HyperError(err)
  }
}

pub struct UsersController;

impl UsersController {
  pub fn index(state: State) -> (State, Response) {
    let connection = ControllerHelper::connection(&state);
    let logger = ControllerHelper::logger(&state);
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
    let connection = ControllerHelper::connection(&state);
    let logger = ControllerHelper::logger(&state);
    let UserPathParams { id } = UserPathParams::take_from(&mut state);
    let user_response = users.find(&id).first::<User>(&*connection);
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
        error!(logger, "Error loading user."; "err" => ?err, "id" => &id); // TODO: Add trace id
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

  pub fn create(mut state: State) -> Box<HandlerFuture> {
    let connection = ControllerHelper::connection(&state);
    let logger = ControllerHelper::logger(&state);
    let f = Body::take_from(&mut state)
      .concat2()
      .then(move |full_body|
        match full_body {
          Ok(valid_body) => {
            let body = String::from_utf8(valid_body.to_vec()).unwrap();
            let new_user_data: UserCreateParams = serde_json::from_str(&body)
              .expect("Failed to parse user from body.");
            let new_user = User::create(&logger, &*connection, &new_user_data.name);
            let new_user_json = serde_json::to_string(&new_user).unwrap();
            info!(logger, "Create request"; "body" => &body, "new_user" => ?new_user);
            let response = create_response(
              &state,
              StatusCode::Ok,
              Some((new_user_json.into_bytes(), mime::APPLICATION_JSON)),
            );
            future::ok((state, response))
          },
          Err(err) => {
            error!(logger, "Error during create"; "err" => ?err);
            future::err((state, err.into_handler_error()))
          },
        }
      );

    Box::new(f)
  }

  pub fn update(mut state: State) -> Box<HandlerFuture> {
    let connection = ControllerHelper::connection(&state);
    let logger = ControllerHelper::logger(&state);
    let UserPathParams { id } = UserPathParams::take_from(&mut state);
    let f = Body::take_from(&mut state)
      .concat2()
      .then(move |full_body|
        match full_body {
          Ok(valid_body) => {
            let body_result = String::from_utf8(valid_body.to_vec());
            match body_result {
              Ok(body) => {
                let update_user_json_result = UsersUpdateAction::update(
                  &*connection,
                  &logger,
                  &id,
                  &body
                );
                match update_user_json_result {
                  Ok(update_user_json) => {
                    info!(logger, "Create request"; "body" => &body);
                    let response = create_response(
                      &state,
                      StatusCode::Ok,
                      Some((update_user_json.into_bytes(), mime::APPLICATION_JSON)),
                    );
                    future::ok((state, response))
                  },
                  Err(err) => {
                    error!(logger, "Error during update"; "err" => ?err);
                    future::err((state, err.into_handler_error()))
                  }
                }
              },
              Err(err) => {
                error!(logger, "Error during update"; "err" => ?err);
                future::err((state, err.into_handler_error()))
              },
            }
          },
          Err(err) => {
            error!(logger, "Error during update"; "err" => ?err);
            future::err((state, err.into_handler_error()))
          },
        }
      );

    Box::new(f)
  }

  // fn json_body_string(full_body: hyper::Chunk)
  //   -> Result<String, UsersControllerError>
  // {
  //   match full_body {
  //     Ok(valid_body) => {
  //       let body = String::from_utf8(valid_body.to_vec())?;
  //       Ok(body)
  //     },
  //     Err(err) => Err(UsersControllerError::HyperError(err)),
  //   }
  // }

  // fn json_body_string(valid_body: hyper::Chunk)
  //   -> Result<String, UsersControllerError>
  // {
  //   let body = String::from_utf8(valid_body.to_vec())?;
  //   Ok(body)
  // }
}
