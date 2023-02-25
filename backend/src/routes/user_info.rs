use crate::model::{KeyPairRepository, UserRepository};
use actix_web::{get, web, Responder};
use serde_json::json;
use sqlx::PgPool;

#[get("/users/{name}")]
async fn user_info(
    pool: web::Data<PgPool>,
    host_name: web::Data<String>,
    param: web::Path<String>,
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
