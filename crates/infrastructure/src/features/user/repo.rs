use app::user::UserRepo;
use async_trait::async_trait;
use domain::user::{User, UserId};
use sqlx::{FromRow, PgPool};

#[derive(FromRow)]
struct PgUserRow {
    id: UserId,
    email: String,
    name: String,
    created_at: chrono::DateTime<chrono::Utc>,
    updated_at: chrono::DateTime<chrono::Utc>,
    last_login_at: Option<chrono::DateTime<chrono::Utc>>,
}

impl From<PgUserRow> for User {
    fn from(row: PgUserRow) -> Self {
        Self {
            id: row.id,
            email: row.email,
            name: row.name,
            created_at: row.created_at,
            updated_at: row.updated_at,
            last_login_at: row.last_login_at,
        }
    }
}

pub struct PgUserRepo {
    pool: PgPool,
}

impl PgUserRepo {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserRepo for PgUserRepo {
    async fn get_user_by_id(&self, user_id: UserId) -> anyhow::Result<Option<User>> {
        let row = sqlx::query_as::<_, PgUserRow>(
            r#"
            SELECT id, email, name, created_at, updated_at, last_login_at
            FROM users
            WHERE id = $1
            "#,
        )
        .bind(user_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(|r| r.into()))
    }

    async fn upsert_user(&self, email: &str, name: &str) -> anyhow::Result<User> {
        let row = sqlx::query_as::<_, PgUserRow>(
            r#"
            INSERT INTO users (email, name)
            VALUES ($1, $2)
            ON CONFLICT (email)
            DO UPDATE SET name = EXCLUDED.name, updated_at = NOW()
            RETURNING *
            "#,
        )
        .bind(email)
        .bind(name)
        .fetch_one(&self.pool)
        .await?;

        Ok(row.into())
    }
}
