mod features;

pub use features::auth;
pub use features::user;
use sqlx::{Pool, Postgres, postgres::PgPoolOptions};

pub async fn bootstrap_db(database_url: &str) -> Result<Pool<Postgres>, anyhow::Error> {
  let db = PgPoolOptions::new()
    .max_connections(5)
    .connect(database_url)
    .await
    .expect("Could not establish database connection");

  sqlx::migrate!("../../migrations")
    .run(&db)
    .await
    .expect("Failed to run migrations");

  Ok(db)
}
