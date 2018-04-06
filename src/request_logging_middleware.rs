use hyper::{Body, Headers, HttpVersion, Method, Uri};
use gotham::handler::HandlerFuture;
use gotham::middleware::Middleware;
use gotham::state::{FromState, State};
use slog;

#[derive(Clone, NewMiddleware)]
pub struct RequestLoggingMiddleware {
  logger: slog::Logger,
}

impl RequestLoggingMiddleware {
  pub fn new(logger: slog::Logger) -> RequestLoggingMiddleware {
    RequestLoggingMiddleware {
      logger: logger,
    }
  }
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
      let body = Body::borrow_from(&state);
      info!(self.logger, "Request recieved."; "method" => ?method, "uri" => ?uri,
        "http_version" => ?http_version, "headers" => ?headers, "body" => ?body
      );
    }

    chain(state)
  }
}
