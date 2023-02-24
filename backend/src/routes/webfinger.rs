use actix_web::{get, web, Responder};
use serde::Deserialize;
use serde_json::json;

#[derive(Deserialize)]
pub struct WebfingerQuery {
    resource: String,
}

#[get("/.well-known/webfinger")]
pub async fn webfinger(
    web::Query(query): web::Query<WebfingerQuery>,
    host_name: web::Data<String>,
) -> impl Responder {
    let email = query.resource.replace("acct:", "");
    // TODO: assume valid email address.
    let user_name = email.split('@').next().unwrap();
    let host_name = host_name.as_ref();
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
