use api::ApiConfig;
use dotenv::dotenv;
use infrastructure::bootstrap_db;

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
  dotenv().ok();

  let api_config = ApiConfig::from_env().expect("Failed to load config");

  let db = bootstrap_db(&api_config.database_url)
    .await
    .expect("Failed to connect to database");
  println!("Connected to the database");

  api::run(api_config, db).await
}
