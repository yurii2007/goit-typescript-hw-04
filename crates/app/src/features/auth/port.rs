use async_trait::async_trait;
use domain::auth::ProviderUser;
use domain::user::UserId;

#[async_trait]
pub trait AuthProviderPort: Send + Sync {
  fn authorization_url(&self) -> (String, String);

  async fn exchange_code(
    &self,
    code: &str,
    state: &str,
    expected_state: &str,
  ) -> anyhow::Result<ProviderUser>;
}

#[async_trait]
pub trait SessionPort: Send + Sync {
  async fn set_user_id(&self, user_id: UserId) -> Result<(), String>;
  async fn get_user_id(&self) -> Result<Option<UserId>, String>;
  async fn clear(&self) -> Result<(), String>;
}
