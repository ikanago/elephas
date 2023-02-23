use async_trait::async_trait;
use sqlx::PgPool;

use crate::model::User;

#[async_trait]
pub trait UserRepository {
    async fn create_user(&self, email: &str, name: &str) -> anyhow::Result<()>;
    async fn get_user_by_name(&self, name: &str) -> anyhow::Result<User>;
}

#[async_trait]
impl UserRepository for PgPool {
    async fn create_user(&self, email: &str, name: &str) -> anyhow::Result<()> {
        sqlx::query!(
            r#"
            INSERT INTO users ("email", "name")
            VALUES (
                $1,
                $2
            )
            "#,
            email,
            name
        )
        .execute(self)
        .await?;
        Ok(())
    }

    async fn get_user_by_name(&self, name: &str) -> anyhow::Result<User> {
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
        pool.create_user(email, name).await.unwrap();

        let user = pool.get_user_by_name("ikanago").await.unwrap();
        assert_eq!(email, user.email);
        assert_eq!(name, user.name);
    }
}
