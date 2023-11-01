use serde_json::json;
use sqlx::PgPool;

use crate::model::{key_pair::sign_headers, KeyPairRepository};

pub async fn follow_remote_person(
    pool: &PgPool,
    host_name: &str,
    follow_from_user_name: &str,
    follow_to_user_name: &str,
    follow_to_host_name: &str,
) -> crate::Result<()> {
    let person = crate::service::remote_user::resolve(
        follow_to_user_name,
        follow_to_host_name,
        crate::model::webfinger::RemoteWebfingerRepositoryImpl,
        crate::model::ap_person::ApPersonRepositoryImpl,
    )
    .await?;
    let payload = json!({
        "@context": [
            "https://www.w3.org/ns/activitystreams",
            "https://w3id.org/security/v1",
        ],
        "type": "Follow",
        "actor": format!("https://{}/api/users/{}", host_name, follow_from_user_name),
        "object": person.id,
    });
    let keypair = pool
        .get_key_pair_by_user_name(follow_from_user_name)
        .await?;
    let headers = sign_headers(&payload, &person.inbox, &keypair.private_key)?;

    reqwest::Client::new()
        .post(&person.inbox)
        .headers(headers)
        .json(&payload)
        .send()
        .await?;
    Ok(())
}
