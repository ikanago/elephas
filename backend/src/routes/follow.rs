use actix_session::Session;
use actix_web::{get, post, web, HttpResponse, Responder};
use serde::Deserialize;
use sqlx::PgPool;
use tracing::debug;
use utoipa::ToSchema;

use crate::{
    error::ServiceError,
    model::{
        follow::{Follow, FollowRepository},
        UserRepository,
    },
    service::user_profile::UserProfile,
    SESSION_KEY,
};

#[derive(Clone, Debug, Deserialize, ToSchema)]
pub struct NewFollow {
    pub follow_to_name: String,
}

#[utoipa::path(
    request_body = NewFollow,
    responses(
        (status = 204, description = "Successfully follow the user"),
        (status = 401, body = ErrorMessage, description = "Unauthorized"),
        (status = 500, body = ErrorMessage, description = "InternalServerError"),
    )
)]
#[post("/follow")]
#[tracing::instrument(skip(pool, session))]
pub async fn create_follow(
    pool: web::Data<PgPool>,
    body: web::Json<NewFollow>,
    session: Session,
) -> crate::Result<impl Responder> {
    create_follow_service(pool.as_ref(), body.into_inner(), session).await
}

async fn create_follow_service(
    pool: &PgPool,
    NewFollow { follow_to_name }: NewFollow,
    session: Session,
) -> crate::Result<impl Responder> {
    let user_name = session
        .get::<String>(SESSION_KEY)
        .map_err(|_| ServiceError::InternalServerError)?
        .ok_or(ServiceError::WrongCredential)?;

    if pool.get_user_by_name(&follow_to_name).await.is_err() {
        return Err(ServiceError::UserNotFound);
    }

    let follow = Follow {
        follow_from_name: user_name,
        follow_to_name,
    };
    pool.save_follow(follow).await?;
    Ok(HttpResponse::NoContent().finish())
}

#[utoipa::path(
    responses(
        (status = 200, body = Vec<UserProfile>, description = "Successfully get followees for a user"),
        (status = 401, body = ErrorMessage, description = "Unauthorized"),
        (status = 500, body = ErrorMessage, description = "InternalServerError"),
    )
)]
#[get("/followees/{name}")]
#[tracing::instrument(skip(pool))]
pub async fn get_followees_by_user_name(
    pool: web::Data<PgPool>,
    user_name: web::Path<String>,
) -> crate::Result<impl Responder> {
    let folloees = pool
        .get_followees_by_name(&user_name)
        .await?
        .into_iter()
        .map(|user| UserProfile { name: user.name })
        .collect::<Vec<_>>();
    debug!(folloees = ?folloees);
    Ok(HttpResponse::Ok().json(folloees))
}

#[utoipa::path(
    responses(
        (status = 200, body = Vec<UserProfile>, description = "Successfully get followers for a user"),
        (status = 401, body = ErrorMessage, description = "Unauthorized"),
        (status = 500, body = ErrorMessage, description = "InternalServerError"),
    )
)]
#[get("/followers/{name}")]
#[tracing::instrument(skip(pool))]
pub async fn get_followers_by_user_name(
    pool: web::Data<PgPool>,
    user_name: web::Path<String>,
) -> crate::Result<impl Responder> {
    let folloers = pool
        .get_followers_by_name(&user_name)
        .await?
        .into_iter()
        .map(|user| UserProfile { name: user.name })
        .collect::<Vec<_>>();
    debug!(folloers = ?folloers);
    Ok(HttpResponse::Ok().json(folloers))
}
