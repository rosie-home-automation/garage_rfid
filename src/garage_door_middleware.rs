use gotham::handler::HandlerFuture;
use gotham::middleware::Middleware;
use gotham::state::State;
use std::sync::{Arc, Mutex};

use garage_door::GarageDoor;

#[derive(Clone, StateData)]
pub struct GarageDoorMiddleware {
  pub garage_door: Arc<Mutex<GarageDoor>>,
}

#[derive(Clone, NewMiddleware)]
pub struct GarageDoorMiddlewareImpl {
  garage_door: Arc<Mutex<GarageDoor>>,
}

impl GarageDoorMiddlewareImpl {
  pub fn new(
    garage_door: Arc<Mutex<GarageDoor>>
  )
    -> GarageDoorMiddlewareImpl
  {
    GarageDoorMiddlewareImpl {
      garage_door: garage_door
    }
  }
}

impl Middleware for GarageDoorMiddlewareImpl {
  fn call<Chain>(self, mut state: State, chain: Chain) -> Box<HandlerFuture>
  where
    Chain: FnOnce(State) -> Box<HandlerFuture>,
  {
    let garage_door_middleware = GarageDoorMiddleware { garage_door: self.garage_door.clone() };
    state.put(garage_door_middleware);

    chain(state)
  }
}

// impl GarageDoorMiddleware {
//   pub fn new(garage_door: Arc<Mutex<GarageDoor>>)
//     -> GarageDoorMiddleware
//   {
//     GarageDoorMiddleware {
//       garage_door: Arc<Mutex<GarageDoor>>,
//     }
//   }
// }
