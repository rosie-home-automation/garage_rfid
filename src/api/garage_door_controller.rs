use gotham::http::response::create_response;
use gotham::state::{FromState, State};
use gotham::handler::{HandlerFuture, IntoHandlerError};
use futures::{future, Future, Stream};
use hyper::{self, Body, Response, StatusCode};
use mime;
use serde_json;
use slog;

use api::controller_helper::ControllerHelper;

pub struct GarageDoorController;

impl GarageDoorController {
  pub fn show(mut state: State) -> (State, Response) {
    let logger = ControllerHelper::logger(&state);
    let response = create_response(
      &state,
      StatusCode::Ok,
      Some(("\"INDEX: blah\"".to_string().into_bytes(), mime::APPLICATION_JSON)),
    );
    (state, response)
  }

  pub fn toggle(mut state: State) -> (State, Response) {
    let logger = ControllerHelper::logger(&state);
    let response = create_response(
      &state,
      StatusCode::Ok,
      Some(("\"TOGGLE: blah\"".to_string().into_bytes(), mime::APPLICATION_JSON)),
    );
    (state, response)
  }

  pub fn open(mut state: State) -> (State, Response) {
    let logger = ControllerHelper::logger(&state);
    let response = create_response(
      &state,
      StatusCode::Ok,
      Some(("\"OPEN: blah\"".to_string().into_bytes(), mime::APPLICATION_JSON)),
    );
    (state, response)
  }

  pub fn close(mut state: State) -> (State, Response) {
    let logger = ControllerHelper::logger(&state);
    let response = create_response(
      &state,
      StatusCode::Ok,
      Some(("\"CLOSE: blah\"".to_string().into_bytes(), mime::APPLICATION_JSON)),
    );
    (state, response)
  }
}
