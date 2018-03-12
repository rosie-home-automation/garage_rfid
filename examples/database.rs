#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

table! {
    users (id) {
        id -> Integer,
        name -> Text,
    }
}

#[derive(Queryable)]
struct User {
  id: usize,
  name: String
}

#[derive(Insertable)]
#[table_name="users"]
struct NewUser<'a> {
  name: &'a str
}

fn establish_connection() -> SqliteConnection {
  dotenv().ok();
  let database_url = env::var("DATABASE_URL")
    .expect("Environment variable DATABASE_URL is required.");
  SqliteConnection::establish(&database_url)
    .expect(&format!("Error connecting to {}", database_url))
}

fn creat_user(connection: &SqliteConnection, name: &str) -> usize {
  let new_user = NewUser { name: name };
  diesel::insert_into(users::table)
    .values(new_user)
    .execute(connection)
    .expect("Error creating new user");
}

fn test1() {
  let connection = establish_connection();
  let id = creat_user(connection, "Matilda");
  println!("CREATED USER ID {}", id);
}

fn main() {
  test1();
}
