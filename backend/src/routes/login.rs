use crate::{error::ServiceError, model::UserRepository};
use actix_session::Session;
use actix_web::{post, web, HttpResponse, Responder};
use argon2::{password_hash::Encoding, Argon2, PasswordHash, PasswordVerifier};
use serde::Deserialize;
use sqlx::PgPool;

#[derive(Clone, Deserialize)]
pub struct LoginCredential {
    pub name: String,
    pub password: String,
}

#[post("/login")]
pub async fn login(
    pool: web::Data<PgPool>,
    body: web::Json<LoginCredential>,
    session: Session,
) -> crate::Result<impl Responder> {
    login_service(pool.as_ref(), body.into_inner(), session).await
}

async fn login_service(
    pool: &PgPool,
    LoginCredential { name, password }: LoginCredential,
    session: Session,
) -> crate::Result<impl Responder> {
    let user = pool.get_user_by_name(&name).await.unwrap();
    verify_password(&password, &user.password_hash)?;

    session.renew();
    session
        .insert("user_id", user.id)
        .expect("user ID is must be serializable");
    Ok(HttpResponse::Ok().finish())
}

fn verify_password(password: &str, password_hash: &str) -> crate::Result<()> {
    let hash = PasswordHash::parse(password_hash, Encoding::default())
        .map_err(|_| ServiceError::InternalServerError)?;
    Argon2::default()
        .verify_password(password.as_bytes(), &hash)
        .map_err(|_| ServiceError::Unauthorized)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[sqlx::test]
    async fn invalid_password(pool: PgPool) {}
}
