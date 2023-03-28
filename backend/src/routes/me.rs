use actix_session::Session;
use actix_web::{get, web, Responder};
use serde::Serialize;
use serde_json::json;
use sqlx::PgPool;
use utoipa::ToSchema;

use crate::{error::ServiceError, service::user_profile::user_profile_service, SESSION_KEY};

#[derive(Clone, Serialize, ToSchema)]
pub struct UserInfoResponse {
    #[schema(example = "alice")]
    pub name: String,
}

#[utoipa::path(
    responses(
        (status = 200, body = UserProfile, description = "Successfully fetched user info"),
        (status = 401, body = ErrorMessage, description = "Unauthorized"),
        (status = 500, body = ErrorMessage, description = "InternalServerError"),
    )
)]
#[get("/me")]
#[tracing::instrument(skip(pool, session))]
pub async fn me(pool: web::Data<PgPool>, session: Session) -> crate::Result<impl Responder> {
    // TODO: extract session validation
    let user_name = session
        .get::<String>(SESSION_KEY)
        .map_err(|_| ServiceError::InternalServerError)?
        .ok_or(ServiceError::WrongCredential)?;
    let user = user_profile_service(pool.as_ref(), &user_name).await?;
    Ok(web::Json(json!(user)))
}
