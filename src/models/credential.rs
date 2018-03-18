use chrono::prelude::{NaiveDateTime, Utc};
use diesel;
use diesel::prelude::*;
use slog;
use uuid::Uuid;

use models::user::User;
use schema::credentials;

#[derive(Associations, Debug, Identifiable, Insertable, Queryable)]
#[belongs_to(User)]
#[table_name = "credentials"]
pub struct Credential {
  pub id: String,
  pub user_id: String,
  pub name: String,
  pub variety: String,
  pub value: String,
  pub created_at: NaiveDateTime,
  pub updated_at: NaiveDateTime,
}

impl Credential {
  pub fn create(
    logger: &slog::Logger,
    conn: &SqliteConnection,
    user_id: &str,
    name: String,
    variety: String,
    value: String,
  ) -> Credential {
    let logger = logger.clone();
    let id = Uuid::new_v4().to_string();
    let now = Utc::now().naive_utc();
    let new_credential = Credential {
      id: id,
      user_id: user_id.to_string(),
      name: name,
      variety: variety,
      value: value,
      created_at: now,
      updated_at: now,
    };
    let result = diesel::insert_into(credentials::table)
      .values(&new_credential)
      .execute(conn);
    match result {
      Ok(_num_rows) => {
        info!(logger, "Created credential"; "credential" => format!("{:?}", new_credential));
        new_credential
      },
      Err(err) => {
        error!(logger, "Failed to create credential";
          "credential" => format!("{:?}", new_credential),
          "err" => format!("{:?}", err)
        );
        panic!("TODO: Remove panic for Credential::create");
      }
    }
  }
}
