use bus;
use std::sync::{ Arc, Mutex, RwLock, Weak };
use std::thread;
use webthing;
use webthing::Thing;

use garage_door::GarageDoor;

struct Generator;

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
      // "fade" => Some(Box::new(FadeAction::new(input, thing))),
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
        Box::new(Generator),
        None,
        None,
      );
      server.create();
      server.start();
    });
    let mut garage_door_rx = self.subscribe_garage_door();
    let _rx_thread = thread::spawn(move || {
      'read_rx_loop: loop {
        let message = match garage_door_rx.recv() {
          Ok(msg) => msg,
          _ => "Failed to receive".to_string(),
        };
        println!("IOT RX {}", message);
      }
    });
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
      "description": "Whether the garage door is open"
    });
    let is_open = garage_door.lock().unwrap().is_open();
    let on_description = on_description.as_object().unwrap().clone();
    thing.add_property(Box::new(webthing::BaseProperty::new(
      "open".to_owned(),
      json!(is_open),
      None,
      Some(on_description),
    )));
    Arc::new(RwLock::new(Box::new(thing)))
  }

  fn subscribe_garage_door(&self) -> bus::BusReader<String> {
    self.garage_door.lock().unwrap().subscribe()
  }
}
