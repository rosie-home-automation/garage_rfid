use gotham::http::response::create_response;
use gotham::state::{FromState, State};
use gotham::handler::{HandlerFuture, IntoHandlerError};
use futures::{future, Future, Stream};
use hyper::{self, Body, Response, StatusCode};
use mime;
use serde_json;
use slog;

use api::controller_helper::ControllerHelper;

#[derive(Debug, Serialize)]
pub struct Show {
  status: String,
}

pub struct GarageDoorController;

impl GarageDoorController {
  pub fn show(state: State) -> (State, Response) {
    let logger = ControllerHelper::logger(&state);
    let garage_door = ControllerHelper::garage_door(&state);
    let status = garage_door.lock().unwrap().status();
    let show = Show { status: status };
    let show_json_result = serde_json::to_string(&show);
    match show_json_result {
      Ok(show_json) => {
        let response = create_response(
          &state,
          StatusCode::Ok,
          Some((show_json.into_bytes(), mime::APPLICATION_JSON)),
        );
        (state, response)
      },
      Err(err) => {
        error!(logger, "Error converting show to json."; "err" => ?err); // TODO: Add trace id
        let response = create_response(
          &state,
          StatusCode::InternalServerError,
          Some((vec![], mime::APPLICATION_JSON)),
        );
        (state, response)
      }
    }
  }

  pub fn toggle(state: State) -> (State, Response) {
    let logger = ControllerHelper::logger(&state);
    let garage_door = ControllerHelper::garage_door(&state);
    garage_door.lock().unwrap().toggle();
    let response = create_response(
      &state,
      StatusCode::Ok,
      Some(("\"Door toggled\"".to_string().into_bytes(), mime::APPLICATION_JSON)),
    );
    (state, response)
  }

  pub fn open(state: State) -> (State, Response) {
    let logger = ControllerHelper::logger(&state);
    let garage_door = ControllerHelper::garage_door(&state);
    garage_door.lock().unwrap().open();
    let response = create_response(
      &state,
      StatusCode::Ok,
      Some(("\"Door opened\"".to_string().into_bytes(), mime::APPLICATION_JSON)),
    );
    (state, response)
  }

  pub fn close(state: State) -> (State, Response) {
    let logger = ControllerHelper::logger(&state);
    let garage_door = ControllerHelper::garage_door(&state);
    garage_door.lock().unwrap().close();
    let response = create_response(
      &state,
      StatusCode::Ok,
      Some(("\"Door closed\"".to_string().into_bytes(), mime::APPLICATION_JSON)),
    );
    (state, response)
  }
}
