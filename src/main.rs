use std::sync::Arc;

use actix_session::{SessionMiddleware, storage::RedisSessionStore};
use actix_web::{App, HttpServer, cookie::Key, middleware::Logger, web};
use sqlx::postgres::PgPoolOptions;

use crate::{
    auth::{
        providers::google::GoogleProvider,
        routes::{AuthState, callback, login, logout},
    },
    config::AppConfig,
    middleware::auth::RequireAuth,
};

mod auth;
mod config;
mod db;
mod middleware;
mod session;

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    let config = AppConfig::from_env().expect("Failed to load config");

    let db = PgPoolOptions::new()
        .max_connections(5)
        .connect(&config.database_url)
        .await
        .expect("Could not establish database connection");

    sqlx::migrate!("./migrations")
        .run(&db)
        .await
        .expect("Failed to run migrations");

    let redis_store = RedisSessionStore::new(&config.redis_url)
        .await
        .expect("Could not establish connection with Redis");

    let secret_key = Key::from(config.session_secret.as_bytes());

    let google_provider = Arc::new(GoogleProvider::new(
        &config.google_client_id,
        &config.google_client_secret,
        &config.google_redirect_uri,
    ));

    let auth_state = web::Data::new(AuthState {
        provider: google_provider,
        db: db.clone(),
    });

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(SessionMiddleware::new(
                redis_store.clone(),
                secret_key.clone(),
            ))
            .app_data(auth_state.clone())
            .service(web::scope("/auth/google").route("/login", web::get().to(login)))
            .route("/callback", web::get().to(callback))
            .route("/auth/logout", web::get().to(logout))
            .service(
                web::scope("/api")
                    .wrap(RequireAuth)
                    .route("/", web::get().to(async || "success")),
            )
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
