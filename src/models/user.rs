use chrono::prelude::{NaiveDateTime, Utc};
use diesel;
use diesel::prelude::*;
use uuid::Uuid;

use schema::users;

#[derive(Debug, Identifiable, Insertable, Queryable)]
#[table_name="users"]
pub struct User {
  pub id: String,
  pub name: String,
  pub created_at: NaiveDateTime,
  pub updated_at: NaiveDateTime
}

impl User {
  pub fn create(conn: &SqliteConnection, name: String) -> User {
    let id = Uuid::new_v4().to_string();
    let now = Utc::now().naive_utc();
    let new_user = User { id: id, name: name, created_at: now, updated_at: now };
    diesel::insert_into(users::table)
      .values(&new_user)
      .execute(conn)
      .expect("Failed to create user.");
    new_user
  }
}
