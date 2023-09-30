use reqwest::{header::ACCEPT, Client};
use serde::Deserialize;

use super::{user_profile::UserProfile, webfinger};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ApPerson {
    id: String,
    r#type: String,
    preferred_username: String,
    name: Option<String>,
    summary: Option<String>,
}

pub async fn resolve(user_name: &str, host_name: &str) -> crate::Result<UserProfile> {
    let href = webfinger::fetch(&user_name, &host_name).await?;
    let client = Client::new();
    let accept =
        r#"application/ld+json; profile="https://www.w3.org/ns/activitystreams"; charset=utf-8"#;
    let object = client
        .get(href)
        .header(ACCEPT, accept)
        .send()
        .await?
        .json::<ApPerson>()
        .await?;
    let profile = UserProfile {
        name: object.preferred_username,
        display_name: object.name.unwrap_or_default(),
        summary: object.summary.unwrap_or_default(),
        avatar_url: "".to_string(),
    };

    Ok(profile)
}

// async fn save(pool: &PgPool, user_name: &str, host_name: &str) -> crate::Result<UserProfile> {}
