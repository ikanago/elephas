use crate::{
    error::ServiceError,
    model::{key_pair::generate_key_pair, KeyPair, KeyPairRepository, User, UserRepository},
};
use actix_session::Session;
use actix_web::{post, web, HttpResponse, Responder};
use serde::Deserialize;
use sqlx::PgPool;
use tracing::info;
use utoipa::ToSchema;

#[derive(Clone, Debug, Deserialize, ToSchema)]
pub struct SignupCredential {
    #[schema(example = "alice")]
    pub name: String,
    #[schema(example = "password")]
    pub password: String,
}

#[utoipa::path(
    request_body = SignupCredential,
    responses(
        (status = 204, description = "Successfully created a new user"),
        (status = 500, body = ErrorMessage, description = "InternalServerError"),
    )
)]
#[post("/signup")]
#[tracing::instrument(skip(pool, session))]
pub async fn signup(
    pool: web::Data<PgPool>,
    body: web::Json<SignupCredential>,
    session: Session,
) -> crate::Result<impl Responder> {
    signup_service(pool.as_ref(), body.into_inner(), session).await
}

pub async fn signup_service(
    pool: &PgPool,
    SignupCredential { name, password }: SignupCredential,
    session: Session,
) -> crate::Result<impl Responder> {
    if pool.get_user_by_name(&name).await.is_ok() {
        return Err(ServiceError::NameAlreadyTaken);
    }

    let user = User {
        name: name.to_string(),
        password_hash: hash_password(&password),
        description: String::new(),
        avatar_url: String::new(),
    };
    info!(user = ?user);
    pool.save_user(user.clone()).await?;

    let (private_key, public_key) = generate_key_pair().unwrap();
    let key_pair = KeyPair {
        user_name: user.name.clone(),
        private_key,
        public_key,
    };
    info!(key_pair = ?key_pair);
    pool.save_key_pair(key_pair).await.unwrap();

    session
        .insert("user_name", user.name.clone())
        .expect("user name must be serializable");
    info!("Create a session for the user {}.", user.name);
    Ok(HttpResponse::NoContent().finish())
}

fn hash_password(password: &str) -> String {
    let cost = match std::env::var("ENV") {
        Ok(env) if env == "dev" => 4,
        _ => bcrypt::DEFAULT_COST,
    };
    bcrypt::hash(password, cost).expect("password successfully hashed")
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
        let regstration = SignupCredential {
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
