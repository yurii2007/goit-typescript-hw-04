use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, prelude::FromRow};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub last_login_at: Option<DateTime<Utc>>,
}

pub async fn insert_user(pool: &PgPool, email: String, name: String) -> anyhow::Result<User> {
    let user = sqlx::query_as::<_, User>(
        r#"
      INSERT INTO users (email, name)
      VALUES ($1, $2)
      ON CONFLICT (email)
      DO UPDATE
      SET name = EXCLUDED.name, updated_at = NOW()
      RETURNING *
    "#,
    )
    .bind(email)
    .bind(name)
    .fetch_one(pool)
    .await?;

    Ok(user)
}
