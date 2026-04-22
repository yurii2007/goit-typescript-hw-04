use anyhow::Result;

#[derive(Debug)]
pub struct ApiConfig {
    pub database_url: String,
    pub redis_url: String,
    pub google_client_id: String,
    pub google_client_secret: String,
    pub google_redirect_uri: String,
    pub session_secret: String,
    pub client_internal_hostname: String,
    pub client_external_hostname: String,
}

impl ApiConfig {
    pub fn from_env() -> Result<Self> {
        Ok(Self {
            database_url: std::env::var("DATABASE_URL")?,
            redis_url: std::env::var("REDIS_URL")?,
            google_client_id: std::env::var("GOOGLE_CLIENT_ID")?,
            google_client_secret: std::env::var("GOOGLE_CLIENT_SECRET")?,
            google_redirect_uri: std::env::var("GOOGLE_REDIRECT_URI")?,
            session_secret: std::env::var("SESSION_SECRET")?,
            client_internal_hostname: std::env::var("CLIENT_INTERNAL_HOSTNAME")?,
            client_external_hostname: std::env::var("CLIENT_EXTERNAL_HOSTNAME")?,
        })
    }
}
