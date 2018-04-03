use chrono::prelude::{NaiveDateTime, Utc};
use diesel;
use diesel::prelude::*;
use slog;
use uuid::Uuid;

// use log_data::LogData;
use models::user::User;
use schema::logs;
use log_data::LogData;

#[derive(Debug)]
#[derive(Associations, Identifiable, Insertable, Queryable)]
#[belongs_to(User)]
#[table_name = "logs"]
#[derive(Serialize, Deserialize)]
pub struct Log {
  pub id: String,
  pub module: String,
  pub action: String,
  pub user_id: Option<String>,
  // pub data: Option<LogData>,
  pub data: Option<LogData>,
  pub created_at: NaiveDateTime,
}

impl Log {
  pub fn create(
    logger: slog::Logger,
    conn: &SqliteConnection,
    module: &str,
    action: &str,
    user_id: Option<&str>,
    data: Option<LogData>,
  ) -> Log {
    let id = Uuid::new_v4().to_string();
    let now = Utc::now().naive_utc();
    let new_log = Log {
      id: id,
      module: module.to_string(),
      action: action.to_string(),
      user_id: Log::format_user_id(user_id),
      // data: Log::format_data(data),
      data: data,
      created_at: now,
    };
    let result = diesel::insert_into(logs::table)
      .values(&new_log)
      .execute(conn);
    match result {
      Ok(_num_rows) => {
        info!(logger, "Created log"; "log" => ?new_log);
        new_log
      },
      Err(err) => {
        error!(logger, "Failed to create log"; "log" => ?new_log, "err" => %err);
        panic!("TODO: Remove panic for Log::create");
      }
    }
  }

  fn format_user_id(user_id: Option<&str>) -> Option<String> {
    match user_id {
      Some(id) => Some(id.to_string()),
      None => None
    }
  }

  // fn format_data(data: Option<&str>) -> Option<String> {
  //   match data {
  //     Some(json) => Some(json.to_string()),
  //     None => None
  //   }
  // }
}
