use serde::Serialize;
use sqlx::PgPool;
use tracing::info;
use utoipa::ToSchema;

use crate::model::UserRepository;

#[derive(Clone, Serialize, ToSchema)]
pub struct UserProfile {
    #[schema(example = "alice")]
    pub name: String,
}

pub async fn user_profile_service(pool: &PgPool, user_name: &str) -> crate::Result<UserProfile> {
    let user = pool.get_user_by_name(user_name).await?;
    info!(user = ?user);
    Ok(UserProfile { name: user.name })
}
