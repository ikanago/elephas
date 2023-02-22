use crate::model::User;
use actix_web::{get, post, web, App, HttpServer, Responder};
use serde::Deserialize;
use sqlx::{postgres::PgPoolOptions, PgPool};

mod model;

#[get("/ping")]
async fn ping() -> impl Responder {
    "pong\n"
}

#[derive(Deserialize)]
struct Regestration {
    email: String,
    name: String,
}

#[get("/users/{name}")]
async fn user(pool: web::Data<PgPool>, param: web::Path<String>) -> impl Responder {
    let name = param.into_inner();
    let user = sqlx::query_as!(
        User,
        r#"
        SELECT * FROM users WHERE name = $1
        "#,
        name
    )
    .fetch_one(&**pool)
    .await
    .unwrap();

    format!("{:?}", user)
}

#[post("/register")]
async fn register(pool: web::Data<PgPool>, body: web::Json<Regestration>) -> impl Responder {
    sqlx::query!(
        r#"
        INSERT INTO users ("email", "name")
        VALUES (
            $1,
            $2
        )
        "#,
        body.email,
        body.name
    )
    .execute(&**pool)
    .await
    .unwrap();

    format!("Successfully created user {}", body.name)
}

#[actix_web::main]
async fn main() {
    dotenvy::dotenv().unwrap();

    let db_url = std::env::var("DATABASE_URL").unwrap();
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .unwrap();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(ping)
            .service(register)
            .service(user)
    })
    .bind(("0.0.0.0", 3000))
    .unwrap()
    .run()
    .await
    .unwrap();
}
