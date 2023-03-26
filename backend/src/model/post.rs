use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use utoipa::ToSchema;

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct Post {
    pub id: String,
    pub user_id: String,
    pub content: String,
    pub published_at: DateTime<Utc>,
}

#[async_trait]
pub trait PostRepository {
    async fn save_post(&self, post: Post) -> crate::Result<()>;
    async fn get_posts_by_user_id(&self, user_id: &str) -> crate::Result<Vec<Post>>;
}

#[async_trait]
impl PostRepository for PgPool {
    async fn save_post(&self, post: Post) -> crate::Result<()> {
        sqlx::query!(
            r#"
            INSERT INTO posts ("id", "user_id", "content", "published_at")
            VALUES (
                $1,
                $2,
                $3,
                $4
            )
            "#,
            post.id,
            post.user_id,
            post.content,
            post.published_at
        )
        .execute(self)
        .await?;
        Ok(())
    }

    async fn get_posts_by_user_id(&self, user_id: &str) -> crate::Result<Vec<Post>> {
        let posts = sqlx::query_as!(
            Post,
            r#"
            SELECT * FROM posts WHERE user_id = $1
            "#,
            user_id
        )
        .fetch_all(self)
        .await?;
        Ok(posts)
    }
}
