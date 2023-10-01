use serde::Deserialize;
use sqlx::PgPool;
use tracing::info;
use utoipa::ToSchema;

use crate::{
    model::{
        ap_person::ApPersonRepositoryImpl,
        user::{User, UserRepository},
        user_profile::UserProfile,
        webfinger::RemoteWebfingerRepositoryImpl,
    },
    service::remote_user,
};

pub async fn get_user_profile_service(
    pool: &PgPool,
    user_name: &str,
) -> crate::Result<UserProfile> {
    info!(user_name);
    if let Some((user_name, host_name)) = parse_user_and_host_name(user_name) {
        let user_profile = remote_user::resolve(
            &user_name,
            &host_name,
            RemoteWebfingerRepositoryImpl,
            ApPersonRepositoryImpl,
        )
        .await?;
        info!(user = ?user_profile);
        return Ok(user_profile);
    }

    let user = pool.get_user_by_name(user_name).await?;
    let user_profile = UserProfile::from(user);
    info!(user = ?user_profile);
    Ok(user_profile)
}

fn parse_user_and_host_name(user_and_host_name: &str) -> Option<(String, String)> {
    let mut iter = user_and_host_name.split('@');
    let user_name = iter.next()?;
    let host_name = iter.next()?;
    Some((user_name.to_string(), host_name.to_string()))
}

#[derive(Clone, Debug, Deserialize, ToSchema)]
pub struct UserProfileUpdate {
    pub display_name: String,
    pub summary: String,
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
        summary: user_profile.summary,
        avatar_url: user_profile.avatar_url,
        ..user
    };
    pool.save_user(user).await?;
    Ok(())
}
