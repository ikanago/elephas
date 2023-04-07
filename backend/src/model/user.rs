use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct User {
    pub name: String,
    pub password_hash: String,
    pub display_name: String,
    pub summary: String,
    pub avatar_url: String,
}

#[async_trait]
pub trait UserRepository {
    async fn save_user(&self, user: User) -> crate::Result<()>;
    async fn get_user_by_name(&self, name: &str) -> crate::Result<User>;
}

#[async_trait]
impl UserRepository for PgPool {
    async fn save_user(&self, user: User) -> crate::Result<()> {
        sqlx::query!(
            r#"
            INSERT INTO users ("name", "password_hash", "display_name", "summary", "avatar_url")
            VALUES (
                $1,
                $2,
                $3,
                $4,
                $5
            )
            ON CONFLICT ("name") DO UPDATE
            SET
                "password_hash" = EXCLUDED."password_hash",
                "display_name" = EXCLUDED."display_name",
                "summary" = EXCLUDED."summary",
                "avatar_url" = EXCLUDED."avatar_url"
            "#,
            user.name,
            user.password_hash,
            user.display_name,
            user.summary,
            user.avatar_url
        )
        .execute(self)
        .await?;
        Ok(())
    }

    async fn get_user_by_name(&self, name: &str) -> crate::Result<User> {
        let user = sqlx::query_as!(
            User,
            r#"
            SELECT * FROM users WHERE name = $1
            "#,
            name
        )
        .fetch_one(self)
        .await?;
        Ok(user)
    }
}
