use api::ApiConfig;
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    dotenv().ok();

    let api_config = ApiConfig::from_env().expect("Failed to load config");

    let db = PgPoolOptions::new()
        .max_connections(5)
        .connect(&api_config.database_url)
        .await
        .expect("Could not establish database connection");

    println!("Connected to the postgres");
    // let redis_store = RedisSessionStore::new(&config.redis_url)
    //     .await
    //     .expect("Could not establish connection with Redis");

    // let secret_key = Key::from(config.session_secret.as_bytes());
    //
    // let google_provider = Arc::new(GoogleProvider::new(
    //     &config.google_client_id,
    //     &config.google_client_secret,
    //     &config.google_redirect_uri,
    // ));
    //
    // let auth_state = web::Data::new(AuthState {
    //     provider: google_provider,
    //     db: db.clone(),
    // });
    api::run(api_config).await
}
