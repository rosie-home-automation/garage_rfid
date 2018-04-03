use diesel::deserialize::{self, FromSql};
use diesel::sqlite::Sqlite;
use diesel::sql_types::*;
use diesel::serialize::{self, Output, ToSql};
use diesel::backend::Backend;
use serde_json;
use std::io::Write;

#[derive(Debug, Serialize, Deserialize)]
#[derive(AsExpression, FromSqlRow, Clone)]
#[sql_type = "Text"]
pub enum LogData {
  AuthSuccessData { credential_id: String },
  AuthFailData { variety: String, value: String },
}

impl<DB> FromSql<Text, DB> for LogData  where DB: Backend, String: FromSql<Text, DB> {
  fn from_sql(bytes: Option<&DB::RawValue>) -> deserialize::Result<Self> {
    match <String as FromSql<Text, DB>>::from_sql(bytes)? {
      s => {
        let result: LogData = serde_json::from_str(&s).unwrap();
        Ok(result)
      }
    }
  }
}

impl ToSql<Text, Sqlite> for LogData {
  fn to_sql<W: Write>(&self, out: &mut Output<W, Sqlite>) -> serialize::Result {
    let x = serde_json::to_string(&self);
    match x {
      Ok(result) => ToSql::<Text, Sqlite>::to_sql(&result, out),
      _ => panic!("ToSql: Not sure what happened"), // Should probably handle an error
      //  Err(err) => err,
    }
  }
}
