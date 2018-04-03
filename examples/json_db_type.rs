#[macro_use]
extern crate diesel;
extern crate slog;
extern crate garage_rfid;

use diesel::prelude::*;

use garage_rfid::great_manager::GreatManager;
use garage_rfid::log_data::LogData;
use garage_rfid::models::log::Log;
use garage_rfid::schema::logs;

fn create_log(logger: slog::Logger, connection: &SqliteConnection, user_id: &str) -> Log {
  let data = LogData::AuthFailData { variety: "RFID".to_string(), value: "12345".to_string() };
  Log::create(logger.clone(), connection, "TestModule", "create_log", Some(user_id), Some(data))
}

fn test1() {
  let great_manager = GreatManager::new().unwrap();
  let connection = great_manager.database.connection();
  let logger = great_manager.root_logger().clone();
  // let user = creat_user(&connection, "Matilda".to_string());
  // let credential = create_credential(&connection, &user);
  // println!("CREATED USER {:?}", user);

  // let user_id = "dea3f799-f6da-423d-aecf-21518a8b76ad";
  // let log = create_log(logger.clone(), &connection, &user_id);
  // println!("Log {:?}", log);
  let logs = logs::table.load::<Log>(&connection)
    .expect("Error loading logs");
  println!("Logs {:?}", logs);
}

fn main() {
  test1();
}
