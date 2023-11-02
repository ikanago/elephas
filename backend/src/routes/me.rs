use actix_session::Session;
use actix_web::{web, HttpResponse, Responder};
use serde_json::json;
use sqlx::PgPool;

use crate::{
    error::ServiceError,
    service::user_profile::{
        get_user_profile_service, update_user_profile_service, UserProfileUpdate,
    },
    SESSION_KEY,
};

#[utoipa::path(
    get,
    path = "/me",
    responses(
        (status = 200, body = UserProfile, description = "Successfully fetched user info"),
        (status = 401, body = ErrorMessage, description = "Unauthorized"),
        (status = 500, body = ErrorMessage, description = "InternalServerError"),
    )
)]
#[tracing::instrument(skip(pool, session))]
pub async fn me(pool: web::Data<PgPool>, session: Session) -> crate::Result<impl Responder> {
    // TODO: extract session validation
    let user_name = session
        .get::<String>(SESSION_KEY)
        .map_err(|_| ServiceError::InternalServerError)?
        .ok_or(ServiceError::WrongCredential)?;
    let user_profile = get_user_profile_service(pool.as_ref(), &user_name).await?;
    Ok(web::Json(json!(user_profile)))
}

#[utoipa::path(
    post,
    path = "/me",
    request_body = UserProfileUpdate,
    responses(
        (status = 204, description = "Successfully update user profile"),
        (status = 401, body = ErrorMessage, description = "Unauthorized"),
        (status = 500, body = ErrorMessage, description = "InternalServerError"),
    )
)]
#[tracing::instrument(skip(pool, session))]
pub async fn update_me(
    pool: web::Data<PgPool>,
    session: Session,
    body: web::Json<UserProfileUpdate>,
) -> crate::Result<impl Responder> {
    let user_name = session
        .get::<String>(SESSION_KEY)
        .map_err(|_| ServiceError::InternalServerError)?
        .ok_or(ServiceError::WrongCredential)?;
    update_user_profile_service(pool.as_ref(), &user_name, body.into_inner()).await?;
    Ok(HttpResponse::NoContent().finish())
}
