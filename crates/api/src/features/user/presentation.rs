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

#[derive(Serialize, Debug)]
pub struct AuthStatusResponse {
  pub data: AuthStatusData,
  pub status: u16,
}

#[derive(Serialize, Debug)]
pub struct AuthStatusData {
  #[serde(skip_serializing_if = "Option::is_none")]
  pub user: Option<ApiResponseUser>,
}
