use chrono::prelude::{NaiveDateTime, Utc};
use diesel;
use diesel::prelude::*;
use uuid::Uuid;

use models::user::User;
use schema::credentials;

// #[derive(Debug, PartialEq, FromSqlRow, AsExpression)]
// #[sql_type = "VARCHAR"]
// pub enum Variety {
//   PIN,
//   RFID,
// }

// impl deserialize::ToSql<sql_types::Text, backend::Sqlite> for Variety {
//   fn to_sql<W: std::io::Write>(&self, out: &mut serialize::Output<W, backend::Sqlite>) -> serialize::Result {
//     match *self {
//       Variety::PIN => out.write_all(b"PIN")?,
//       Variety::RFID => out.write_all(b"RFID")?,
//     }
//     Ok(IsNull::No)
//   }
// }

// impl deserialize::FromSql<sql_types::Text, backend::Sqlite> for Variety {
//   fn from_sql(bytes: Option<&[u8]>) -> deserialize::Result<Self> {
//     match not_none!(bytes) {
//       b"PIN" => Ok(Variety::PIN),
//       b"RFID" => Ok(Variety::RFID),
//       _ => Err("Unrecognized enum variant".into()),
//     }
//   }
// }

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
    conn: &SqliteConnection,
    user_id: &str,
    name: String,
    variety: String,
    value: String,
  ) -> Credential {
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
    diesel::insert_into(credentials::table)
      .values(&new_credential)
      .execute(conn)
      .expect("Failed to create credential.");
    new_credential
  }
}
