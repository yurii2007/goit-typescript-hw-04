use domain::user::{User, UserId};
use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct ApiResponseUser {
  id: UserId,
  name: String,
  email: String,
}

impl From<User> for ApiResponseUser {
  fn from(value: User) -> Self {
    Self {
      id: value.id,
      name: value.name,
      email: value.email,
    }
  }
}
