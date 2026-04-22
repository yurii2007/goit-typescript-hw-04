use crate::auth::port::AuthProviderPort;
use crate::user::UserRepo;
use domain::user::User;

pub struct AuthService<A: AuthProviderPort, R: UserRepo> {
    provider: A,
    user_repo: R,
}

impl<A: AuthProviderPort, R: UserRepo> AuthService<A, R> {
    pub fn new(provider: A, user_repo: R) -> Self {
        Self {
            provider,
            user_repo,
        }
    }

    pub fn get_login_url(&self) -> (String, String) {
        self.provider.authorization_url()
    }

    pub async fn handle_callback(
        &self,
        code: &str,
        state: &str,
        expected_state: &str,
    ) -> anyhow::Result<User> {
        let provider_user = self.provider.exchange_code(code, state, expected_state).await?;
        let user = self.user_repo.upsert_user(&provider_user.email, &provider_user.name).await?;
        Ok(user)
    }
}
