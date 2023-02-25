use crate::{
    error::ServiceError,
    model::{key_pair::generate_key_pair, KeyPair, KeyPairRepository, User, UserRepository},
};
use actix_web::{post, web, Responder};
use rand::Rng;
use serde::Deserialize;
use sqlx::PgPool;

const ID_LEN: usize = 16;

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

    let user = User {
        id: generate_id(ID_LEN),
        name: name.to_string(),
    };
    pool.save_user(user).await?;
    let user = pool.get_user_by_name(&name).await?;

    let (private_key, public_key) = generate_key_pair().unwrap();
    let key_pair = KeyPair {
        user_id: user.id.clone(),
        private_key,
        public_key,
    };
    pool.save_key_pair(key_pair).await.unwrap();
    Ok(format!("Successfully created user {}", name))
}

fn generate_id(len: usize) -> String {
    rand::thread_rng()
        .sample_iter(rand::distributions::Alphanumeric)
        .take(len)
        .map(char::from)
        .collect()
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
