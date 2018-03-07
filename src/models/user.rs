use diesel;
use diesel::prelude::*;
use schema::users;

#[derive(Debug, Queryable)]
pub struct User {
  id: i32,
  name: String
}

#[derive(Insertable)]
#[table_name="users"]
pub struct NewUser {
  name: String
}

impl User {
  pub fn new(name: String) -> NewUser {
    NewUser { name: name }
  }
}

impl NewUser {
  pub fn save(&self, conn: &SqliteConnection) -> User {
    let new_user_id = diesel::insert_into(users::table)
      .values(self)
      .execute(conn)
      .expect("Failed to create user.");
    users::dsl::users.find(new_user_id as i32)
      .first(conn)
      .expect(&format!("Unable to find user with id {}", new_user_id))
  }
}
