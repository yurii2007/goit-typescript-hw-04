use chrono::{DateTime, Utc};
use uuid::Uuid;

pub type UserId = Uuid;

#[derive(Debug, Clone)]
pub struct User {
    pub id: UserId,
    pub email: String,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub last_login_at: Option<DateTime<Utc>>,
}
