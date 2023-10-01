use actix_web::{get, web, HttpResponse};
use serde::Deserialize;
use sqlx::PgPool;

use crate::{
    model::UserRepository,
    service::webfinger::{Webfinger, WebfingerLink},
};

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
) -> HttpResponse {
    let user_and_host_name = query.resource.replace("acct:", "");
    // TODO: assume valid email address.
    let user_name = user_and_host_name.split('@').next().unwrap();
    let host_name = host_name.as_ref();

    if let Err(_) = pool.get_user_by_name(user_name).await {
        return HttpResponse::NotFound().finish();
    }

    let webfinger = Webfinger {
        subject: format!("acct:{user_name}@{host_name}"),
        links: vec![{
            WebfingerLink {
                href: Some(format!("https://{host_name}/api/users/{user_name}")),
                rel: "self".to_string(),
                r#type: Some("application/activity+json".to_string()),
            }
        }],
    };
    HttpResponse::Ok().json(webfinger)
}
