use bus;
use serde_json;
use std::sync::{ Arc, Mutex, RwLock, Weak };
use std::thread;
use webthing;
use webthing::Thing;

use garage_door::GarageDoor;
use iot::close_action::CloseAction;
use iot::garage_door_event::GarageDoorEvent;

struct Generator {
  ttt: String,
}

impl Generator {
  pub fn new(ttt: String) -> Generator {
    Generator { ttt: ttt }
  }
}

impl webthing::server::ActionGenerator for Generator {
  fn generate(
    &self,
    thing: Weak<RwLock<Box<dyn webthing::Thing>>>,
    name: String,
    input: Option<&serde_json::Value>,
  ) -> Option<Box<dyn webthing::Action>> {
    let input = match input {
      Some(v) => match v.as_object() {
        Some(o) => Some(o.clone()),
        None => None,
      },
      None => None,
    };

    let name: &str = &name;
    match name {
      "close" => Some(Box::new(CloseAction::new(input, thing))),
      _ => None,
    }
  }
}

pub struct IotThing {
  garage_door: Arc<Mutex<GarageDoor>>,
  thing: Arc<RwLock<Box<dyn webthing::Thing + 'static>>>,
}

impl IotThing {
  pub fn new(garage_door: Arc<Mutex<GarageDoor>>) -> Self {
    let thing = IotThing::make_thing(garage_door.clone());
    IotThing { garage_door: garage_door, thing: thing }
  }

  pub fn start(&self) {
    let thing = self.thing.clone();
    let _server_thread = thread::spawn(move || {
      let mut server = webthing::WebThingServer::new(
        webthing::ThingsType::Single(thing),
        Some(8888),
        None,
        None,
        Box::new(Generator::new("hello".to_string())),
        None,
        None,
      );
      server.create();
      server.start();
    });
    let mut garage_door_rx = self.subscribe_garage_door();
    let thing = self.thing.clone();
    let _rx_thread = thread::spawn(move || {
      'read_rx_loop: loop {
        let message = match garage_door_rx.recv() {
          Ok(msg) => msg,
          _ => "Failed to receive".to_string(),
        };
        match message.as_ref() {
          "opened" => IotThing::set_open(true, thing.clone()),
          "closed" => IotThing::set_open(false, thing.clone()),
          _ => {},
        }
      }
    });
  }

  // pub fn broadcast_action(action: String) {
  //   let action : &str = &action;
  //   match action {
  //     "close" => { println!("CLOSE ACTION!!!") }
  //   }
  // }

  fn set_open(is_open: bool, thing: Arc<RwLock<Box<dyn webthing::Thing>>>) {
    let value = json!(is_open);
    let message = match is_open {
      true => "Garage door opened",
      false => "Garage door closed",
    };

    {
      let mut thing = thing.write().unwrap();
      let prop = thing.find_property("open".to_owned()).unwrap();
      prop.set_value(value.clone());
    }
    thing.write().unwrap().property_notify("open".to_owned(), value);
    thing.write().unwrap().add_event(Box::new(GarageDoorEvent::new(Some(json!(message)))));
  }

  fn make_thing(garage_door: Arc<Mutex<GarageDoor>>) -> Arc<RwLock<Box<dyn webthing::Thing + 'static>>> {
    let mut thing = webthing::BaseThing::new(
      "urn:dev:ops:rfid-garage-door-01".to_owned(),
      "RFID Garage Door".to_owned(),
      Some(vec!["DoorSensor".to_owned()]),
      Some("Rfid reader and garage door".to_owned()),
    );
    let on_description = json!({
      "@type": "OpenProperty",
      "title": "Open",
      "type": "boolean",
      "description": "Whether the garage door is open",
    });
    let is_open = garage_door.lock().unwrap().is_open();
    let on_description = on_description.as_object().unwrap().clone();
    thing.add_property(Box::new(webthing::BaseProperty::new(
      "open".to_owned(),
      json!(is_open),
      None,
      Some(on_description),
    )));

    let garage_door_event_description = json!({
      "description": "The garage opened or closed",
      "type": "string",
    });
    let garage_door_event_description = garage_door_event_description.as_object().unwrap().clone();
    thing.add_available_event("garage_door_event".to_owned(), garage_door_event_description);

    let close_metadata = json!({
        "title": "Close",
        "description": "Close the garage door",
    });
    let close_metadata = close_metadata.as_object().unwrap().clone();
    thing.add_available_action("close".to_owned(), close_metadata);

    Arc::new(RwLock::new(Box::new(thing)))
  }

  fn subscribe_garage_door(&self) -> bus::BusReader<String> {
    self.garage_door.lock().unwrap().subscribe()
  }
}
