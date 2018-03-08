use diesel;
use diesel::prelude::*;
use schema::users;

no_arg_sql_function!(last_insert_rowid, diesel::sql_types::Integer);

#[derive(Debug, Insertable, Queryable)]
#[table_name="users"]
pub struct User {
  pub id: Option<i32>,
  pub name: String
}

impl User {
  pub fn new(name: String) -> User {
    User { id: None, name: name }
  }

  pub fn save(&self, conn: &SqliteConnection) -> User {
    diesel::insert_into(users::table)
      .values(self)
      .execute(conn)
      .expect("Failed to create user.");
    let id: i32 = diesel::select(last_insert_rowid).first(conn).expect("Expected last insert id.");
    users::dsl::users.find(id)
      .first(conn)
      .expect(&format!("Unable to find user with id {}", id))
  }
}
