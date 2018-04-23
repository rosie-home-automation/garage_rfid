use diesel;
use diesel::prelude::*;
use serde_json;
use slog;
use std::error;
use std::fmt;

use models::user::User;

#[derive(Debug)]
pub enum UserUpdateActionError {
  Diesel(diesel::result::Error),
  Json(serde_json::Error),
}

impl fmt::Display for UserUpdateActionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UserUpdateActionError::Diesel(ref err) => write!(f, "Diesel error: {}", err),
            UserUpdateActionError::Json(ref err) => write!(f, "Json error: {}", err),
        }
    }
}

impl error::Error for UserUpdateActionError {
    fn description(&self) -> &str {
        match *self {
            UserUpdateActionError::Diesel(ref err) => err.description(),
            UserUpdateActionError::Json(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            UserUpdateActionError::Diesel(ref err) => Some(err),
            UserUpdateActionError::Json(ref err) => Some(err),
        }
    }
}

impl From<diesel::result::Error> for UserUpdateActionError {
  fn from(err: diesel::result::Error) -> UserUpdateActionError {
    UserUpdateActionError::Diesel(err)
  }
}

impl From<serde_json::Error> for UserUpdateActionError {
  fn from(err: serde_json::Error) -> UserUpdateActionError {
    UserUpdateActionError::Json(err)
  }
}

#[derive(Debug, Deserialize)]
struct Params {
  name: String,
}

pub struct UsersUpdateAction;

impl UsersUpdateAction {
  pub fn update(connection: &SqliteConnection, _logger: &slog::Logger, id: &str, body: &str)
    -> Result<String, UserUpdateActionError>
  {
    let mut user = User::find(&connection, &id)?;
    let user_data: Params = serde_json::from_str(&body)?;
    user.name = user_data.name.clone();
    user.save(&connection)?;
    let user_json = serde_json::to_string(&user)?;
    Ok(user_json)
  }
}
