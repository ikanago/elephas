use async_trait::async_trait;
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

#[mockall::automock]
#[async_trait]
pub trait RemoteWebfingerRepository {
    async fn fetch_webfinger(&self, user_name: &str, host_name: &str) -> crate::Result<Webfinger>;
}

#[derive(Clone)]
pub struct RemoteWebfingerRepositoryImpl;

#[async_trait]
impl RemoteWebfingerRepository for RemoteWebfingerRepositoryImpl {
    async fn fetch_webfinger(&self, user_name: &str, host_name: &str) -> crate::Result<Webfinger> {
        let url = format!(
            "https://{}/.well-known/webfinger?resource=acct:{}@{}",
            host_name, user_name, host_name
        );
        let webfinger = reqwest::get(url).await?.json::<Webfinger>().await?;
        info!(webfinger = ?webfinger.clone());
        Ok(webfinger)
    }
}
