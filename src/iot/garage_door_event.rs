use serde_json;
use webthing;

pub struct GarageDoorEvent(webthing::BaseEvent);

impl GarageDoorEvent {
  pub fn new(data: Option<serde_json::Value>) -> GarageDoorEvent {
    GarageDoorEvent(webthing::BaseEvent::new("garage_door_event".to_owned(), data))
  }
}

impl webthing::Event for GarageDoorEvent {
  fn get_name(&self) -> String {
    self.0.get_name()
  }

  fn get_data(&self) -> Option<serde_json::Value> {
    self.0.get_data()
  }

  fn get_time(&self) -> String {
    self.0.get_time()
  }
}
