use actix_web::{get, web, Either, HttpResponse, Responder};
use serde::Deserialize;
use serde_json::json;
use sqlx::PgPool;

use crate::model::UserRepository;

#[derive(Debug, Deserialize)]
pub struct WebfingerQuery {
    resource: String,
}

#[get("/.well-known/webfinger")]
#[tracing::instrument(skip(pool))]
pub async fn webfinger(
    pool: web::Data<PgPool>,
    host_name: web::Data<String>,
    web::Query(query): web::Query<WebfingerQuery>,
) -> Either<HttpResponse, impl Responder> {
    let user_and_host_name = query.resource.replace("acct:", "");
    // TODO: assume valid email address.
    let user_name = user_and_host_name.split('@').next().unwrap();
    let host_name = host_name.as_ref();

    if let Err(_) = pool.get_user_by_name(user_name).await {
        return Either::Left(HttpResponse::NotFound().finish());
    }

    Either::Right(web::Json(json!({
        "subject": format!("acct:{user_name}@{host_name}"),
        "links": [
            {
                "rel": "self",
                "type": "application/activity+json",
                "href": format!("https://{host_name}/api/users/{user_name}"),
            },
        ],
    })))
}
