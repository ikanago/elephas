use serde::{Deserialize, Serialize};
use tracing::info;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Webfinger {
    pub subject: String,
    pub links: Vec<WebfingerLink>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WebfingerLink {
    pub href: Option<String>,
    pub rel: String,
    pub r#type: Option<String>,
}

pub async fn fetch(user_name: &str, host_name: &str) -> crate::Result<String> {
    let url = format!(
        "https://{}/.well-known/webfinger?resource=acct:{}@{}",
        host_name, user_name, host_name
    );
    let webfinger = reqwest::get(url).await?.json::<Webfinger>().await?;;
    info!(webfinger = ?webfinger.clone());
    Ok(webfinger
        .links
        .iter()
        .find(|link| link.rel == "self")
        .ok_or_else(|| crate::error::ServiceError::InternalServerError)?
        .href
        .clone()
        .ok_or_else(|| crate::error::ServiceError::InternalServerError)?)
}
