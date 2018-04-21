use diesel::prelude::SqliteConnection;
use gotham::handler::HandlerFuture;
use gotham::middleware::Middleware;
use gotham::state::State;
use r2d2;
use r2d2_diesel;
use std::panic::{catch_unwind, AssertUnwindSafe};

#[derive(StateData)]
pub struct DieselMiddleware {
  pub pool: AssertUnwindSafe<r2d2::Pool<r2d2_diesel::ConnectionManager<SqliteConnection>>>,
}

#[derive(NewMiddleware)]
pub struct DieselMiddlewareImpl {
  pool: AssertUnwindSafe<r2d2::Pool<r2d2_diesel::ConnectionManager<SqliteConnection>>>,
}

impl DieselMiddlewareImpl {
  pub fn new(pool: r2d2::Pool<r2d2_diesel::ConnectionManager<SqliteConnection>>)
    -> DieselMiddlewareImpl
  {
    let pool = AssertUnwindSafe(pool);
    DieselMiddlewareImpl {
      pool: pool,
    }
  }
}

impl Middleware for DieselMiddlewareImpl {
  fn call<Chain>(self, mut state: State, chain: Chain) -> Box<HandlerFuture>
  where
    Chain: FnOnce(State) -> Box<HandlerFuture>,
  {
    let diesel_middleware = DieselMiddleware{ pool: self.pool };
    state.put(diesel_middleware);

    chain(state)
  }
}

impl Clone for DieselMiddlewareImpl {
  fn clone(&self) -> DieselMiddlewareImpl {
    match catch_unwind(|| self.pool.clone()) {
      Ok(pool) => {
        let pool = AssertUnwindSafe(pool);
        DieselMiddlewareImpl {
          pool: pool,
        }
      },
      Err(err) => {
        panic!("R2D2 pool clone caused a panic {:?}", err);
      }
    }
  }
}

impl DieselMiddleware {
  pub fn new(pool: r2d2::Pool<r2d2_diesel::ConnectionManager<SqliteConnection>>)
    -> DieselMiddleware
  {
    let pool = AssertUnwindSafe(pool);
    DieselMiddleware {
      pool: pool,
    }
  }
}

impl Clone for DieselMiddleware {
  fn clone(&self) -> DieselMiddleware {
    match catch_unwind(|| self.pool.clone()) {
      Ok(pool) => {
        let pool = AssertUnwindSafe(pool);
        DieselMiddleware {
          pool: pool,
        }
      },
      Err(err) => {
        panic!("R2D2 pool clone caused a panic {:?}", err);
      }
    }
  }
}
