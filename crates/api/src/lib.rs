use std::sync::Arc;

use actix_cors::Cors;
use actix_session::{SessionMiddleware, storage::RedisSessionStore};
use actix_web::{App, HttpServer, cookie, middleware::Logger, web};
use infrastructure::auth::GoogleProvider;
use infrastructure::user::PgUserRepo;
use sqlx::PgPool;

use app::auth::AuthService;

mod config;
mod features;
mod middleware;
mod routes;
mod session;

pub use config::ApiConfig;

use middleware::auth::RequireAuth;
use routes::auth::{AuthState, callback, login, logout};

use crate::routes::auth::get_current_user;

pub async fn run(config: ApiConfig, db: PgPool) -> Result<(), std::io::Error> {
  println!("Connecting to redis...");
  let redis_store = RedisSessionStore::new(&config.redis_url)
    .await
    .expect("Could not establish connection with Redis");
  println!("Connected to redis");

  let secret_key = cookie::Key::from(config.session_secret.as_bytes());

  let google_provider = GoogleProvider::new(
    &config.google_client_id,
    &config.google_client_secret,
    &config.google_redirect_uri,
  );

  let user_repo = PgUserRepo::new(db);
  let auth_service = Arc::new(AuthService::new(google_provider, user_repo));

  let auth_state = web::Data::new(AuthState {
    auth_service,
    redirect_url: config.client_external_hostname.clone(),
  });

  println!("Starting an app");
  HttpServer::new(move || {
    let cors = Cors::default()
      .allowed_origin(&config.client_internal_hostname)
      .allowed_origin(&config.client_external_hostname)
      .allowed_origin("https://accounts.google.com")
      .allowed_methods(vec!["GET", "POST", "PUT", "PATCH", "DELETE", "OPTIONS"])
      .allow_any_header()
      .supports_credentials();

    App::new()
      .wrap(Logger::default())
      .wrap(cors)
      .wrap(
        SessionMiddleware::builder(redis_store.clone(), secret_key.clone())
          .cookie_same_site(cookie::SameSite::None)
          .cookie_secure(true)
          .build(),
      )
      .app_data(auth_state.clone())
      .service(
        web::scope("/auth")
          .route("/", web::get().to(get_current_user))
          .route("/google/login", web::get().to(login))
          .route("/google/callback", web::get().to(callback)),
      )
      .route("/auth/logout", web::get().to(logout))
      .service(
        web::scope("/api")
          .wrap(RequireAuth)
          .route("/status", web::get().to(async || "success")),
      )
  })
  .bind("0.0.0.0:8080")?
  .run()
  .await
}
