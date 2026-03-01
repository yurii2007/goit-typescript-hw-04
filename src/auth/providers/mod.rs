use async_trait::async_trait;
use serde::{Deserialize, Serialize};

pub mod google;

#[derive(Debug, Serialize, Deserialize)]
pub struct ProviderUser {
    pub provider_id: String,
    pub email: String,
    pub name: String,
}

#[async_trait]
pub trait OAuthProvider: Send + Sync {
    fn authorization_url(&self) -> (String, String);

    async fn exchange_code(
        &self,
        code: &str,
        state: &str,
        expected_state: &str,
    ) -> anyhow::Result<ProviderUser>;
}
