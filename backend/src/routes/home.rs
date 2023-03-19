use actix_session::Session;
use actix_web::{get, web, HttpResponse, Responder};
use serde_json::json;
use sqlx::PgPool;

use crate::{error::ServiceError, model::UserRepository};

#[get("/")]
pub async fn home(pool: web::Data<PgPool>, session: Session) -> impl Responder {
    home_service(pool.as_ref(), session).await
}

async fn home_service(pool: &PgPool, session: Session) -> crate::Result<impl Responder> {
    // TODO: extract session validation
    let stored_user_id = session
        .get::<String>("user_id")
        .map_err(|_| ServiceError::InternalServerError)?
        .ok_or(ServiceError::WrongCredential)?;
    let user = pool.get_user_by_id(&stored_user_id).await.unwrap();

    Ok(HttpResponse::Ok().json(json!({
        "name": user.name,
    })))
}
