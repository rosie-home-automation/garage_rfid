use chrono::prelude::{NaiveDateTime, Utc};
use diesel;
use diesel::prelude::*;
use slog;
use uuid::Uuid;

use schema::users;

#[derive(Debug, Identifiable, Insertable, Queryable)]
#[table_name="users"]
pub struct User {
  pub id: String,
  pub name: String,
  pub created_at: NaiveDateTime,
  pub updated_at: NaiveDateTime,
}

impl User {
  pub fn create(logger: &slog::Logger, conn: &SqliteConnection, name: String) -> User {
    let logger = logger.clone();
    let id = Uuid::new_v4().to_string();
    let now = Utc::now().naive_utc();
    let new_user = User { id: id, name: name, created_at: now, updated_at: now };
    let result =  diesel::insert_into(users::table)
      .values(&new_user)
      .execute(conn);

    match result {
      Ok(_num_rows) => {
        info!(logger, "Created user"; "user" => format!("{:?}", new_user));
        new_user
      },
      Err(err) => {
        crit!(logger, "Failed to create user"; "user" => format!("{:?}", new_user), "error" => format!("{:?}", err));
        panic!("TODO: Remove panic for User::create");
      }
    }
  }
}
