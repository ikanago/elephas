use crate::model::User;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;
use serde_json::json;
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

#[derive(Deserialize)]
struct WebfingerQuery {
    resource: String,
}

#[get("/.well-known/webfinger")]
async fn webfinger(
    web::Query(query): web::Query<WebfingerQuery>,
    host_name: web::Data<String>,
) -> impl Responder {
    let email = query.resource.replace("acct:", "");
    // TODO: assume valid email address.
    let user_name = email.split("@").next().unwrap();
    let host_name = &**host_name;
    // TODO: check if the user exists
    web::Json(json!({
        "subject": format!("acct:{user_name}@{host_name}"),
        "links": [
            {
                "rel": "self",
                "type": "application/activity+json",
                "href": format!("https://{host_name}/users/{user_name}"),
            },
        ],
    }))
}

#[get("/.well-known/host-meta")]
async fn host_meta(host_name: web::Data<String>) -> impl Responder {
    let host_name = &**host_name;
    HttpResponse::Ok()
        .insert_header(("Content-Type", "application/xrd+xml; charset=utf-8"))
        .body(format!(
r#"<?xml version="1.0" encoding="UTF-8"?>
<XRD xmlns="http://docs.oasis-open.org/ns/xri/xrd-1.0">
    <Link rel="lrdd" type="application/xrd+xml" template="https://${host_name}/.well-known/webfinger?resource={{uri}}"/>
</XRD>"#))
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

    let host_name = std::env::var("HOST_NAME").unwrap();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(host_name.clone()))
            .service(ping)
            .service(register)
            .service(user)
            .service(webfinger)
            .service(host_meta)
    })
    .bind(("0.0.0.0", 3000))
    .unwrap()
    .run()
    .await
    .unwrap();
}
