use async_trait::async_trait;
use domain::user::{User, UserId};

#[async_trait]
pub trait UserRepo: Send + Sync {
  async fn get_user_by_id(&self, user_id: UserId) -> anyhow::Result<Option<User>>;
  async fn upsert_user(&self, email: &str, name: &str) -> anyhow::Result<User>;
}
