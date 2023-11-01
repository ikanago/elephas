use serde::Deserialize;
use sqlx::PgPool;
use tracing::info;
use utoipa::ToSchema;

use crate::{
    model::{
        activitypub::ActivityPubRequestRepositoryImpl,
        user::{parse_user_and_host_name, User, UserRepository},
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
        let person = remote_user::resolve(
            &user_name,
            &host_name,
            &RemoteWebfingerRepositoryImpl,
            &ActivityPubRequestRepositoryImpl,
        )
        .await?;
        let user_profile = person.into();
        info!(user = ?user_profile);
        return Ok(user_profile);
    }

    let user = pool.get_user_by_name(user_name).await?;
    let user_profile = UserProfile::from(user);
    info!(user = ?user_profile);
    Ok(user_profile)
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
    // TODO: use UserProfile here
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
