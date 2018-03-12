#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate garage_rfid;

use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

use garage_rfid::models::user::User;
use garage_rfid::models::credential::Credential;
use garage_rfid::schema::users;

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
  Credential::create(connection, &user.id, name, variety, value)
}

fn test1() {
  let connection = establish_connection();
  // let user = creat_user(&connection, "Matilda".to_string());
  // let credential = create_credential(&connection, &user);
  // println!("CREATED USER {:?}", user);

  let user_id = "dea3f799-f6da-423d-aecf-21518a8b76ad";
  let user = users::table.find(user_id).first::<User>(&connection)
    .expect(&format!("Error finding user for id {}", user_id));
  let credentials = Credential::belonging_to(&user).load::<Credential>(&connection)
    .expect("Error loading credentials");
  println!("Credentials {:?}", credentials);
}

fn main() {
  test1();
}
