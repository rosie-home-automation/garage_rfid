use diesel::deserialize::{self, FromSql};
use diesel::sqlite::Sqlite;
use diesel::sql_types::*;
use diesel::serialize::{self, Output, ToSql};
use serde_json;
use std::io::Write;

#[derive(Debug, Serialize, Deserialize)]
#[derive(AsExpression, FromSqlRow, Copy, Clone)]
#[sql_type = "Text"]
pub enum LogData {
  AuthSuccessData,
  AuthFailData,
}

#[derive(Debug)]
pub struct AuthSuccessData {
  pub credential_id: String,
}

#[derive(Debug)]
pub struct AuthFailData {
  pub variety: String,
  pub value: String,
}

impl FromSql<Text, Sqlite> for LogData {
  fn from_sql(bytes: Option<&[u8]>) -> deserialize::Result<Self> {
    match <String as FromSql<Text, Sqlite>>::from_sql(bytes)? {
      Some(s) => {
        let result: LogData = serde_json::from_str(s);
        Ok(result)
      },
      None => Ok(None),
    }
  }
}

impl ToSql<Text, Sqlite> for LogData {
  fn to_sql<W: Write>(&self, out: &mut Output<W, Sqlite>) -> serialize::Result {
    let x = serde_json::to_string(*self);
    match x {
      Ok(result) => ToSql::<Text, Sqlite>::to_sql(&x, out),
      Err(err) => err,
    }
  }
}
