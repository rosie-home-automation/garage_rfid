#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate garage_rfid;

use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

use garage_rfid::models::user::User;
use garage_rfid::models::credential::Credential;

fn establish_connection() -> SqliteConnection {
  dotenv().ok();
  let database_url = env::var("DATABASE_URL")
    .expect("Environment variable DATABASE_URL is required.");
  SqliteConnection::establish(&database_url)
    .expect(&format!("Error connecting to {}", database_url))
}

fn creat_user(connection: &SqliteConnection, name: String) -> User {
  User::create(connection, name)
}

fn create_credential(connection: &SqliteConnection, user: &User) -> Credential {
  let name = "Taco RFID".to_string();
  let variety = "RFID".to_string();
  let value = "101100101100".to_string();
  Credential::create(connection, user.id, name, variety, value)
}

fn test1() {
  let connection = establish_connection();
  let user = creat_user(&connection, "Matilda".to_string());
  println!("CREATED USER {:?}", user);
}

fn main() {
  test1();
}
