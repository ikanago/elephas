use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};

#[derive(Clone, Debug, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: i32,
    pub name: String,
}

#[async_trait]
pub trait UserRepository {
    async fn create_user(&self, name: &str) -> crate::Result<()>;
    async fn get_user_by_name(&self, name: &str) -> crate::Result<User>;
}

#[async_trait]
impl UserRepository for PgPool {
    async fn create_user(&self, name: &str) -> crate::Result<()> {
        sqlx::query!(
            r#"
            INSERT INTO users ("name")
            VALUES (
                $1
            )
            "#,
            name
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

#[cfg(test)]
mod tests {
    use super::*;

    #[sqlx::test]
    async fn get_registered_user(pool: PgPool) {
        let email = "ikanago@example.com";
        let name = "ikanago";
        pool.create_user(name).await.unwrap();

        let user = pool.get_user_by_name("ikanago").await.unwrap();
        assert_eq!(name, user.name);
    }
}
