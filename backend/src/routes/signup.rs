use crate::{
    error::ServiceError,
    model::{key_pair::generate_key_pair, KeyPairRepository, UserRepository},
};
use actix_web::{post, web, Responder};
use serde::Deserialize;
use sqlx::PgPool;

#[derive(Deserialize)]
pub struct Regestration {
    name: String,
}

#[post("/signup")]
pub async fn signup(
    pool: web::Data<PgPool>,
    body: web::Json<Regestration>,
) -> crate::Result<impl Responder> {
    signup_service(pool.as_ref(), &body.name).await
}

async fn signup_service(pool: &PgPool, name: &str) -> crate::Result<impl Responder> {
    if pool.get_user_by_name(&name).await.is_ok() {
        return Err(ServiceError::NameAlreadyTaken);
    }

    pool.create_user(&name).await?;
    let user = pool.get_user_by_name(&name).await?;
    let (private_key, public_key) = generate_key_pair().unwrap();
    pool.create_key_pair(user.id, &private_key, &public_key)
        .await
        .unwrap();
    Ok(format!("Successfully created user {}", name))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[sqlx::test]
    async fn error_already_used_name(pool: PgPool) {
        let name = "ikanago";
        signup_service(&pool, name).await.unwrap();
        assert!(signup_service(&pool, name).await.is_err());
    }
}
