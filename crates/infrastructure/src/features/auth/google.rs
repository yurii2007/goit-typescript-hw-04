use app::auth::AuthProviderPort;
use async_trait::async_trait;
use domain::auth::ProviderUser;
use oauth2::{
    AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, RedirectUrl, Scope,
    TokenResponse, TokenUrl, basic::BasicClient, reqwest::async_http_client,
};
use serde::Deserialize;

pub struct GoogleProvider {
    client: BasicClient,
}

#[derive(Deserialize)]
struct GoogleUserInfo {
    sub: String,
    email: String,
    name: Option<String>,
    #[allow(dead_code)]
    picture: Option<String>,
}

impl GoogleProvider {
    pub fn new(client_id: &str, client_secret: &str, redirect_uri: &str) -> Self {
        let client = BasicClient::new(
            ClientId::new(client_id.to_string()),
            Some(ClientSecret::new(client_secret.to_string())),
            AuthUrl::new("https://accounts.google.com/o/oauth2/v2/auth".to_string()).unwrap(),
            Some(TokenUrl::new("https://oauth2.googleapis.com/token".to_string()).unwrap()),
        )
        .set_redirect_uri(RedirectUrl::new(redirect_uri.to_string()).unwrap());

        Self { client }
    }
}

#[async_trait]
impl AuthProviderPort for GoogleProvider {
    fn authorization_url(&self) -> (String, String) {
        let (url, state) = self
            .client
            .authorize_url(CsrfToken::new_random)
            .add_scope(Scope::new("email".to_string()))
            .add_scope(Scope::new("openid".to_string()))
            .add_scope(Scope::new("profile".to_string()))
            .url();

        (url.to_string(), state.secret().clone())
    }

    async fn exchange_code(
        &self,
        code: &str,
        state: &str,
        expected_state: &str,
    ) -> anyhow::Result<ProviderUser> {
        if state != expected_state {
            anyhow::bail!("OAuth state mismatch - possible CSRF attack");
        }

        let token = self
            .client
            .exchange_code(AuthorizationCode::new(code.to_string()))
            .request_async(async_http_client)
            .await?;

        let user_info: GoogleUserInfo = reqwest::Client::new()
            .get("https://www.googleapis.com/oauth2/v3/userinfo")
            .bearer_auth(token.access_token().secret())
            .send()
            .await?
            .json()
            .await?;

        Ok(ProviderUser {
            provider_id: user_info.sub,
            email: user_info.email,
            name: user_info.name.unwrap_or("-".to_string()),
        })
    }
}
