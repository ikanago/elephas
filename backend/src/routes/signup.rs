use crate::{
    error::ServiceError,
    model::{key_pair::generate_key_pair, KeyPair, KeyPairRepository, User, UserRepository},
};
use actix_session::Session;
use actix_web::{post, web, HttpResponse, Responder};
use argon2::{password_hash::SaltString, Argon2, PasswordHasher};
use rand::Rng;
use rand_core::OsRng;
use serde::Deserialize;
use sqlx::PgPool;

const ID_LEN: usize = 16;

#[derive(Clone, Deserialize)]
pub struct Regestration {
    pub name: String,
    pub password: String,
}

#[post("/signup")]
pub async fn signup(
    pool: web::Data<PgPool>,
    body: web::Json<Regestration>,
    session: Session,
) -> crate::Result<impl Responder> {
    signup_service(pool.as_ref(), body.into_inner(), session).await
}

pub async fn signup_service(
    pool: &PgPool,
    Regestration { name, password }: Regestration,
    session: Session,
) -> crate::Result<impl Responder> {
    if pool.get_user_by_name(&name).await.is_ok() {
        return Err(ServiceError::NameAlreadyTaken);
    }

    let user = User {
        id: generate_id(ID_LEN),
        name: name.to_string(),
        password_hash: hash_password(&password),
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

    session
        .insert("user_id", user.id)
        .expect("user ID is must be serializable");
    Ok(HttpResponse::Ok())
}

fn generate_id(len: usize) -> String {
    rand::thread_rng()
        .sample_iter(rand::distributions::Alphanumeric)
        .take(len)
        .map(char::from)
        .collect()
}

fn hash_password(password: &str) -> String {
    let salt = SaltString::generate(&mut OsRng);
    let hash = Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .expect("password successfully hashed");
    hash.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    use actix_session::SessionExt;
    use actix_web::test::TestRequest;

    #[sqlx::test]
    async fn error_already_used_name(pool: PgPool) {
        // arrange
        let req = TestRequest::default().to_srv_request();
        let session = req.get_session();
        let regstration = Regestration {
            name: "ikanago".to_string(),
            password: "password".to_string(),
        };

        // act
        signup_service(&pool, regstration.clone(), session.clone())
            .await
            .unwrap();

        // assert
        assert!(signup_service(&pool, regstration, session).await.is_err());
    }
}
