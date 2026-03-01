use anyhow::Result;
use dotenv::dotenv;

#[derive(Debug)]
pub struct AppConfig {
    pub database_url: String,
    pub redis_url: String,
    pub google_client_id: String,
    pub google_client_secret: String,
    pub google_redirect_uri: String,
    pub session_secret: String,
}

impl AppConfig {
    pub fn from_env() -> Result<Self> {
        dotenv().ok();

        Ok(Self {
            database_url: std::env::var("DATABASE_URL")?,
            redis_url: std::env::var("REDIS_URL")?,
            google_client_id: std::env::var("GOOGLE_CLIENT_ID")?,
            google_client_secret: std::env::var("GOOGLE_CLIENT_SECRET")?,
            google_redirect_uri: std::env::var("GOOGLE_REDIRECT_URI")?,
            session_secret: std::env::var("SESSION_SECRET")?,
        })
    }
}
