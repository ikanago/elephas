use actix_session::Session;
use actix_web::{get, web, HttpResponse, Responder};
use serde::Serialize;
use serde_json::json;
use sqlx::PgPool;
use tracing::info;
use utoipa::ToSchema;

use crate::{error::ServiceError, model::UserRepository};

#[derive(Clone, Serialize, ToSchema)]
pub struct UserInfoResponse {
    #[schema(example = "alice")]
    pub name: String,
}

#[utoipa::path(
    responses(
        (status = 200, body = UserInfoResponse, description = "Successfully fetched user info"),
        (status = 401, body = ErrorMessage, description = "Unauthorized"),
        (status = 500, body = ErrorMessage, description = "InternalServerError"),
    )
)]
#[get("/me")]
#[tracing::instrument(skip(pool, session))]
pub async fn me(pool: web::Data<PgPool>, session: Session) -> impl Responder {
    me_service(pool.as_ref(), session).await
}

async fn me_service(pool: &PgPool, session: Session) -> crate::Result<impl Responder> {
    // TODO: extract session validation
    let stored_user_name = session
        .get::<String>("user_name")
        .map_err(|_| ServiceError::InternalServerError)?
        .ok_or(ServiceError::WrongCredential)?;
    let user = pool.get_user_by_name(&stored_user_name).await.unwrap();
    info!(user = ?user);

    let res = UserInfoResponse { name: user.name };
    Ok(HttpResponse::Ok().json(json!(res)))
}
