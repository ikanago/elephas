use crate::model::{key_pair::generate_key_pair, KeyPairRepository, UserRepository};
use actix_web::{post, web, Responder};
use serde::Deserialize;
use sqlx::PgPool;

#[derive(Deserialize)]
pub struct Regestration {
    email: String,
    name: String,
}

#[post("/signup")]
pub async fn signup(pool: web::Data<PgPool>, body: web::Json<Regestration>) -> impl Responder {
    // TODO: test creating a new user with the email already used fails.
    signup_service(pool.as_ref(), &body.email, &body.name).await
}

async fn signup_service(pool: &PgPool, email: &str, name: &str) -> impl Responder {
    pool.create_user(&email, &name).await.unwrap();
    let user = pool.get_user_by_name(&name).await.unwrap();
    let (private_key, public_key) = generate_key_pair().unwrap();
    pool.create_key_pair(user.id, &private_key, &public_key)
        .await
        .unwrap();
    format!("Successfully created user {}", name)
}
