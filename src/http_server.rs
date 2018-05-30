use gotham;
use gotham::router::Router;
use gotham::router::builder::*;
use gotham::pipeline::new_pipeline;
use gotham::pipeline::single::single_pipeline;
use slog;

use api::garage_door_controller::GarageDoorController;
use configuration::Configuration;
use database::Database;
use diesel_middleware::DieselMiddlewareImpl;
use logger_middleware::LoggerMiddlewareImpl;
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
    let logger_middleware = LoggerMiddlewareImpl::new(self.logger.clone());
    let request_middleware = RequestLoggingMiddleware::new(self.logger.clone());
    let diesel_middleware_impl = DieselMiddlewareImpl::new(self.database.pool().clone());
    let (chain, pipelines) = single_pipeline(new_pipeline()
      .add(logger_middleware)
      .add(request_middleware)
      .add(diesel_middleware_impl)
      .build());
    build_router(chain, pipelines, |route| {
      route.scope("/api/v1", |route| {
        // users endpoints
        route.scope("/users", |route| {
          route.get("/") // users - index
            .to(UsersController::index);
          route.get("/:id") // users - show
            .with_path_extractor::<UserPathParams>()
            .to(UsersController::show);
          route.post("/") // users - create
            .to(UsersController::create);
          route.post("/:id") // users - update
            .with_path_extractor::<UserPathParams>()
            .to(UsersController::update);
        });
        route.scope("/garage_door", |route| {
          route.get("/") // garage_door - show
            .to(GarageDoorController::show);
          route.post("/toggle") // garage_door - toggle
            .to(GarageDoorController::toggle);
          route.post("/open") // garage_door - open
            .to(GarageDoorController::open);
          route.post("/close") // garage_door - close
            .to(GarageDoorController::close)
        })
      });
    })
  }

  fn setup_logger(&mut self) {
    self.logger = self.logger.new(o!("http_server" => format!("{:?}", self)));
  }
}
