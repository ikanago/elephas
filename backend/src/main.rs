use crate::model::key_pair::generate_key_pair;
use crate::model::KeyPairRepository;
use crate::{infra::UserRepository, model::key_pair::sign_headers};
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;
use serde_json::json;
use sqlx::{postgres::PgPoolOptions, PgPool};

mod infra;
mod model;

#[derive(Deserialize)]
struct Regestration {
    email: String,
    name: String,
}

#[post("/register")]
async fn register(pool: web::Data<PgPool>, body: web::Json<Regestration>) -> impl Responder {
    // TODO: test creating a new user with the email already used fails.
    pool.create_user(&body.email, &body.name).await.unwrap();
    let user = pool.get_user_by_name(&body.name).await.unwrap();
    let (private_key, public_key) = generate_key_pair().unwrap();
    pool.create_key_pair(user.id, &private_key, &public_key)
        .await
        .unwrap();
    format!("Successfully created user {}", body.name)
}

#[get("/users/{name}")]
async fn user_info(
    pool: web::Data<PgPool>,
    param: web::Path<String>,
    host_name: web::Data<String>,
) -> impl Responder {
    let name = param.into_inner();
    let user = pool.get_user_by_name(&name).await.unwrap();
    let key_pair = pool.get_key_pair_by_user_id(user.id).await.unwrap();

    web::Json(json!({
        "@context": [
            "https://www.w3.org/ns/activitystreams",
            "https://w3id.org/security/v1",
        ],
        "type": "Person",
        "id": format!("https://{}/users/{}", &**host_name, name),
        "inbox": format!("https://{}/users/{}/inbox", &**host_name, name),
        "preferredUsername": name,
        "name": name,
        "icon": {
            "type": "Image",
            "url": "https://blog.ikanago.dev/_next/image?url=%2Fblog_icon.png&w=828&q=75",
            "name": "",
        },
        "publicKey": {
            "id": format!("https://{}/users/{}#main-key", &**host_name, name),
            "type": "Key",
            "owner": format!("https://{}/users/{}", &**host_name, name),
            "publicKeyPem": key_pair.public_key,
        },
    }))
}

#[derive(Deserialize)]
struct InboxBody {
    id: String,
    r#type: String,
    actor: String,
    object: String,
}

#[post("/users/{name}/inbox")]
async fn inbox(
    pool: web::Data<PgPool>,
    param: web::Path<String>,
    body: web::Json<InboxBody>,
    host_name: web::Data<String>,
) -> impl Responder {
    let name = param.into_inner();
    let user = pool.get_user_by_name(&name).await.unwrap();

    if body.r#type == "Follow" {
        let payload = json!({
            "@context": "https://www.w3.org/ns/activitystreams",
            "id": format!("https://{}/users/test/accept/1", &**host_name),
            "type": "Accept",
            "actor": body.object,
            "object": {
                "id": body.id,
                "type": body.r#type,
                "actor": body.actor,
                "object": body.object,
            },
        });
        // TODO: assuming remote inbox URL.
        let target_inbox = format!("{}/inbox", body.actor);

        let key_pair = pool.get_key_pair_by_user_id(user.id).await.unwrap();
        let headers = sign_headers(&payload, &target_inbox, &key_pair.private_key).unwrap();

        let client = reqwest::Client::new();
        client
            .post(target_inbox)
            .headers(headers)
            .json(&payload)
            .send()
            .await
            .unwrap();
    }
    HttpResponse::Ok()
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
    let user_name = email.split('@').next().unwrap();
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
            .service(register)
            .service(user_info)
            .service(inbox)
            .service(webfinger)
            .service(host_meta)
    })
    .bind(("0.0.0.0", 3000))
    .unwrap()
    .run()
    .await
    .unwrap();
}
