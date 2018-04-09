use gotham;
use gotham::router::Router;
use gotham::router::builder::*;
use gotham::pipeline::new_pipeline;
use gotham::pipeline::single::single_pipeline;
use slog;

use configuration::Configuration;
use database::Database;
use request_logging_middleware::RequestLoggingMiddleware;
use users_controller::{UsersController, UserPathParams};

#[derive(Debug)]
pub struct HttpServer {
  address: String,
  port: usize,
  database: Database,
  logger: slog::Logger,
}

impl HttpServer {
  pub fn new(configuration: &Configuration, database: Database, logger: slog::Logger) -> HttpServer {
    let address = configuration.http_server.address.clone();
    let port = configuration.http_server.port;
    let mut http_server = HttpServer {
      address: address,
      port: port,
      database: database,
      logger: logger,
    };
    http_server.setup_logger();
    http_server
  }

  pub fn start(&self) {
    let addr = format!("{}:{}", self.address, self.port);
    gotham::start(&addr, self.router());
    info!(self.logger, "Listening for requests."; "addr" => %addr);
  }

  fn router(&self) -> Router {
    let users_controller = UsersController::new(self.database.clone(), self.logger.clone());
    let request_middleware = RequestLoggingMiddleware::new(self.logger.clone());
    let (chain, pipelines) = single_pipeline(new_pipeline().add(request_middleware).build());
    build_router(chain, pipelines, |route| {
      route.scope("/api/v1", |route| {
        {
          let users_controller = users_controller.clone();
          route.get("/users")
            .to_new_handler(move || Ok(|state| users_controller.index(state)));
        }
        {
          let users_controller = users_controller.clone();
          route.get("/users/:id")
            .with_path_extractor::<UserPathParams>()
            .to_new_handler(move || Ok(|state| users_controller.show(state)));
        }
      });
    })
  }

  fn setup_logger(&mut self) {
    self.logger = self.logger.new(o!("http_server" => format!("{:?}", self)));
  }
}
