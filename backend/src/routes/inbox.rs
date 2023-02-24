use crate::model::{key_pair::sign_headers, KeyPairRepository, UserRepository};
use actix_web::{post, web, Responder, HttpResponse};
use serde::Deserialize;
use serde_json::json;
use sqlx::PgPool;

#[derive(Deserialize)]
pub struct InboxBody {
    id: String,
    r#type: String,
    actor: String,
    object: String,
}

#[post("/users/{name}/inbox")]
async fn inbox(
    pool: web::Data<PgPool>,
    host_name: web::Data<String>,
    param: web::Path<String>,
    body: web::Json<InboxBody>,
) -> impl Responder {
    let name = param.into_inner();
    inbox_service(pool.as_ref(), host_name.as_ref(), name, body.into_inner()).await
}

async fn inbox_service(pool: &PgPool, host_name: &str, name: String, body: InboxBody) -> impl Responder {
    let user = pool.get_user_by_name(&name).await.unwrap();

    if body.r#type == "Follow" {
        let payload = json!({
            "@context": "https://www.w3.org/ns/activitystreams",
            "id": format!("https://{}/users/test/accept/1", host_name),
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
