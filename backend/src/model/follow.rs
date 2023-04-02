use super::User;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use utoipa::ToSchema;

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct Follow {
    pub follow_from_name: String,
    pub follow_to_name: String,
}

#[async_trait]
pub trait FollowRepository {
    async fn save_follow(&self, follow: Follow) -> crate::Result<()>;
    async fn delete_follow(&self, follow: Follow) -> crate::Result<()>;
    async fn get_followers_by_name(&self, follow_from_name: &str) -> crate::Result<Vec<User>>;
    async fn get_followees_by_name(&self, follow_to_name: &str) -> crate::Result<Vec<User>>;
}

#[async_trait]
impl FollowRepository for PgPool {
    async fn save_follow(&self, follow: Follow) -> crate::Result<()> {
        sqlx::query!(
            r#"
            INSERT INTO follows ("follow_from_name", "follow_to_name")
            VALUES (
                $1,
                $2
            )
            "#,
            follow.follow_from_name,
            follow.follow_to_name
        )
        .execute(self)
        .await?;
        Ok(())
    }

    async fn delete_follow(&self, follow: Follow) -> crate::Result<()> {
        sqlx::query!(
            r#"
            DELETE FROM follows
            WHERE follow_from_name = $1 AND follow_to_name = $2
            "#,
            follow.follow_from_name,
            follow.follow_to_name
        )
        .execute(self)
        .await?;
        Ok(())
    }

    async fn get_followers_by_name(&self, name: &str) -> crate::Result<Vec<User>> {
        let followers = sqlx::query_as!(
            User,
            r#"
            SELECT users.*
            FROM users INNER JOIN follows ON users.name = follows.follow_from_name
            WHERE follows.follow_to_name = $1
            "#,
            name
        )
        .fetch_all(self)
        .await?;
        Ok(followers)
    }

    async fn get_followees_by_name(&self, name: &str) -> crate::Result<Vec<User>> {
        let followees = sqlx::query_as!(
            User,
            r#"
            SELECT users.*
            FROM users INNER JOIN follows ON users.name = follows.follow_to_name
            WHERE follows.follow_from_name = $1
            "#,
            name
        )
        .fetch_all(self)
        .await?;
        Ok(followees)
    }
}

#[cfg(test)]
mod tests {
    use crate::model::UserRepository;

    use super::*;

    async fn prepare_users(pool: &PgPool) {
        for (name, hash) in [
            ("u1", "p1"),
            ("u2", "p2"),
            ("u3", "p3"),
            ("u4", "p4"),
            ("u5", "p5"),
        ] {
            pool.save_user(User {
                name: name.to_string(),
                password_hash: hash.to_string(),
                description: String::new(),
                avatar_url: String::new(),
            })
            .await
            .unwrap();
        }

        for (from, to) in [("u1", "u2"), ("u1", "u3"), ("u4", "u1"), ("u5", "u1")] {
            pool.save_follow(Follow {
                follow_from_name: from.to_string(),
                follow_to_name: to.to_string(),
            })
            .await
            .unwrap();
        }
    }

    #[sqlx::test]
    async fn get_followers(pool: PgPool) {
        prepare_users(&pool).await;
        let followers = pool.get_followers_by_name("u1").await.unwrap();
        assert_eq!(followers.len(), 2);
        assert_eq!(followers[0].name, "u4");
        assert_eq!(followers[1].name, "u5");
    }

    #[sqlx::test]
    async fn get_followees(pool: PgPool) {
        prepare_users(&pool).await;
        let followees = pool.get_followees_by_name("u1").await.unwrap();
        assert_eq!(followees.len(), 2);
        assert_eq!(followees[0].name, "u2");
        assert_eq!(followees[1].name, "u3");
    }
}
