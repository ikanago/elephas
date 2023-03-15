use actix_session::{storage::RedisActorSessionStore, SessionMiddleware};
use actix_web::{
    cookie::{Key, SameSite},
    web, App, HttpServer,
};
use base64::engine::{general_purpose, Engine};
use routes::routing;
use sqlx::postgres::PgPoolOptions;

mod error;
mod model;
mod routes;

pub type Result<T> = std::result::Result<T, error::ServiceError>;

#[actix_web::main]
async fn main() {
    dotenvy::dotenv().unwrap();

    let db_url = std::env::var("DATABASE_URL").unwrap();
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .unwrap();

    let host_name = std::env::var("HOST_NAME").unwrap();
    let cookie_key = {
        let key = std::env::var("COOKIE_KEY").unwrap();
        let key = general_purpose::STANDARD.decode(key.as_bytes()).unwrap();
        Key::from(&key)
    };
    let redis_url = std::env::var("REDIS_URL").unwrap();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(host_name.clone()))
            .wrap(
                SessionMiddleware::builder(
                    RedisActorSessionStore::new(&redis_url),
                    cookie_key.clone(),
                )
                .cookie_same_site(SameSite::None)
                .build(),
            )
            .service(routing())
    })
    .bind(("0.0.0.0", 3000))
    .unwrap()
    .bind(("::", 3000))
    .unwrap()
    .run()
    .await
    .unwrap();
}
