//! A Hello World example application for working with Gotham.

extern crate futures;
extern crate gotham;
#[macro_use]
extern crate gotham_derive;
extern crate hyper;
extern crate mime;
#[macro_use]
extern crate slog;
extern crate garage_rfid;

use hyper::{Body, Headers, HttpVersion, Method, Response, StatusCode, Uri};
use gotham::http::response::create_response;
use gotham::handler::HandlerFuture;
use gotham::middleware::Middleware;
use gotham::state::{FromState, State};
use gotham::router::Router;
use gotham::router::builder::*;
use gotham::pipeline::new_pipeline;
use gotham::pipeline::single::single_pipeline;

use garage_rfid::great_manager::GreatManager;

struct Api {
  great_manager: GreatManager,
}

impl Api {
  fn new() -> Api {
    let great_manager = GreatManager::new().unwrap();
    let logger = great_manager.root_logger().clone();
    Api { great_manager: great_manager }
  }

  fn router(&self) -> Router {
    let logger = self.great_manager.root_logger();
    let users_controller = UsersController { logger: logger.clone() };
    let request_middleware = RequestLoggingMiddleware { logger: logger.clone() };
    let (chain, pipelines) = single_pipeline(new_pipeline().add(request_middleware).build());
    build_router(chain, pipelines, |route| {
      route.scope("/api/v1", |route| {
        route.get("/users").to_new_handler(move || Ok(|state| users_controller.index(state)));
      });
    })
  }

  fn start(&self) {
    let addr = "0.0.0.0:7878";
    println!("Listening for requests at http://{}", addr);
    gotham::start(addr, self.router());
  }
}

#[derive(Clone, NewMiddleware)]
struct RequestLoggingMiddleware {
  logger: slog::Logger
}

impl Middleware for RequestLoggingMiddleware {
  fn call<Chain>(self, state: State, chain: Chain) -> Box<HandlerFuture>
  where
    Chain: FnOnce(State) -> Box<HandlerFuture>,
  {
    {
      let method = Method::borrow_from(&state);
      let uri = Uri::borrow_from(&state);
      let http_version = HttpVersion::borrow_from(&state);
      let headers = Headers::borrow_from(&state);
      info!(self.logger, "Request recieved."; "method" => ?method, "uri" => ?uri,
        "http_version" => ?http_version, "headers" => ?headers
      );
    }

    chain(state)
  }
}

struct UsersController {
  logger: slog::Logger
}

impl UsersController {
  pub fn index(&self, state: State) -> (State, Response) {
    let response = create_response(
      &state,
      StatusCode::Ok,
      Some((String::from("{ \"type\": \"Taco Cat\" }").into_bytes(), mime::APPLICATION_JSON)),
    );

    (state, response)
  }
}

/// Create a `Handler` which is invoked when responding to a `Request`.
///
/// How does a function become a `Handler`?.
/// We've simply implemented the `Handler` trait, for functions that match the signature used here,
/// within Gotham itself.
pub fn say_hello(state: State) -> (State, Response) {
  let res = create_response(
    &state,
    StatusCode::Ok,
    Some((String::from("Hello World!").into_bytes(), mime::TEXT_PLAIN)),
  );

  (state, res)
}

/// Start a server and call the `Handler` we've defined above for each `Request` we receive.
pub fn main() {
  let api = Api::new();
  api.start();
}
