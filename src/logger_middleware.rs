use gotham::handler::HandlerFuture;
use gotham::middleware::Middleware;
use gotham::state::State;
use slog;

#[derive(Clone, StateData)]
pub struct LoggerMiddleware {
  pub logger: slog::Logger,
}

#[derive(Clone, NewMiddleware)]
pub struct LoggerMiddlewareImpl {
  logger: slog::Logger,
}

impl LoggerMiddlewareImpl {
  pub fn new(logger: slog::Logger)
    -> LoggerMiddlewareImpl
  {
    LoggerMiddlewareImpl {
      logger: logger,
    }
  }
}

impl Middleware for LoggerMiddlewareImpl {
  fn call<Chain>(self, mut state: State, chain: Chain) -> Box<HandlerFuture>
  where
    Chain: FnOnce(State) -> Box<HandlerFuture>,
  {
    let logger_middleware = LoggerMiddleware{ logger: self.logger };
    state.put(logger_middleware);

    chain(state)
  }
}

impl LoggerMiddleware {
  pub fn new(logger: slog::Logger)
    -> LoggerMiddleware
  {
    LoggerMiddleware {
      logger: logger,
    }
  }
}
