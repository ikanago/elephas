use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use tracing::info;
use utoipa::ToSchema;

use crate::model::{User, UserRepository};

#[derive(Clone, Debug, Serialize, ToSchema)]
pub struct UserProfile {
    pub name: String,
    pub display_name: String,
    pub description: String,
    pub avatar_url: String,
}

impl From<User> for UserProfile {
    fn from(user: User) -> Self {
        Self {
            name: user.name,
            display_name: user.display_name,
            description: user.description,
            avatar_url: user.avatar_url,
        }
    }
}

pub async fn get_user_profile_service(
    pool: &PgPool,
    user_name: &str,
) -> crate::Result<UserProfile> {
    let user = pool.get_user_by_name(user_name).await?;
    let user_profile = UserProfile::from(user);
    info!(user = ?user_profile);
    Ok(user_profile)
}

#[derive(Clone, Debug, Deserialize, ToSchema)]
pub struct UserProfileUpdate {
    pub display_name: String,
    pub description: String,
    pub avatar_url: String,
}

pub async fn update_user_profile_service(
    pool: &PgPool,
    user_name: &str,
    user_profile: UserProfileUpdate,
) -> crate::Result<()> {
    let user = pool.get_user_by_name(&user_name).await?;
    let user = User {
        display_name: user_profile.display_name,
        description: user_profile.description,
        avatar_url: user_profile.avatar_url,
        ..user
    };
    pool.save_user(user).await?;
    Ok(())
}
