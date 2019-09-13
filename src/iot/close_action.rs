use serde_json;
use std::sync::{Arc, RwLock, Weak};
use std::thread;
use uuid::Uuid;
use webthing::{ Action, BaseAction, Thing };

pub struct CloseAction(BaseAction);

impl CloseAction {
  pub fn new(
      input: Option<serde_json::Map<String, serde_json::Value>>,
      thing: Weak<RwLock<Box<dyn Thing>>>,
  ) -> CloseAction {
      CloseAction(BaseAction::new(
          // nanoid!(),
          Uuid::new_v4().to_string(),
          "close".to_owned(),
          input,
          thing,
      ))
  }
}

impl Action for CloseAction {
    fn set_href_prefix(&mut self, prefix: String) {
        self.0.set_href_prefix(prefix)
    }

    fn get_id(&self) -> String {
        self.0.get_id()
    }

    fn get_name(&self) -> String {
        self.0.get_name()
    }

    fn get_href(&self) -> String {
        self.0.get_href()
    }

    fn get_status(&self) -> String {
        self.0.get_status()
    }

    fn get_time_requested(&self) -> String {
        self.0.get_time_requested()
    }

    fn get_time_completed(&self) -> Option<String> {
        self.0.get_time_completed()
    }

    fn get_input(&self) -> Option<serde_json::Map<String, serde_json::Value>> {
        self.0.get_input()
    }

    fn get_thing(&self) -> Option<Arc<RwLock<Box<Thing>>>> {
        self.0.get_thing()
    }

    fn set_status(&mut self, status: String) {
        self.0.set_status(status)
    }

    fn start(&mut self) {
        self.0.start()
    }

    fn perform_action(&mut self) {
        let thing = self.get_thing();
        if thing.is_none() {
            return;
        }

        let thing = thing.unwrap();
        let input = self.get_input().unwrap().clone();
        let name = self.get_name();
        let id = self.get_id();

        thread::spawn(move || {
            let thing = thing.clone();
            let mut thing = thing.write().unwrap();

            // TODO: Tell the garage door to close

            thing.finish_action(name, id);
        });
    }

    fn cancel(&mut self) {
        self.0.cancel()
    }

    fn finish(&mut self) {
        self.0.finish()
    }
}
