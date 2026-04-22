use actix_cors::Cors;
use actix_web::{App, HttpServer, middleware::Logger, web};

mod config;

pub use config::ApiConfig;

pub async fn run(config: ApiConfig) -> Result<(), std::io::Error> {
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
            .service(web::scope("/auth/google"))
            .service(web::scope("/api").route("/status", web::get().to(async || "success")))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
