use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};

#[derive(Clone, Debug, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: String,
    pub name: String,
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
            INSERT INTO users ("id", "name")
            VALUES (
                $1,
                $2
            )
            "#,
            user.id,
            user.name
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
        let user = User {
            id: "xxxx".to_string(),
            name: "ikanago".to_string(),
        };
        pool.save_user(user).await.unwrap();

        let user = pool.get_user_by_name("ikanago").await.unwrap();
        assert_eq!(user.id, "xxxx");
        assert_eq!(user.name, "ikanago");
    }
}
