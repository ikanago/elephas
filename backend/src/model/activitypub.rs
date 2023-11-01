use crate::model::key_pair::KeyPair;
use async_trait::async_trait;
use reqwest::header::ACCEPT;
use serde::{de::DeserializeOwned, Deserialize};

#[allow(unused)]
#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Person {
    pub id: String,
    pub r#type: String,
    pub preferred_username: String,
    pub name: Option<String>,
    pub summary: Option<String>,
    pub inbox: String,
}

#[mockall::automock]
#[async_trait]
pub trait ActivityPubRequestRepository {
    async fn get<T: DeserializeOwned + 'static>(&self, url: &str) -> crate::Result<T>;
    async fn post(
        &self,
        url: &str,
        payload: &serde_json::Value,
        keypair: KeyPair,
    ) -> crate::Result<()>;
}

pub struct ActivityPubRequestRepositoryImpl;

#[async_trait]
impl ActivityPubRequestRepository for ActivityPubRequestRepositoryImpl {
    async fn get<T: DeserializeOwned>(&self, url: &str) -> crate::Result<T> {
        let accept = r#"application/ld+json; profile="https://www.w3.org/ns/activitystreams"; charset=utf-8"#;
        let object = reqwest::Client::new()
            .get(url)
            .header(ACCEPT, accept)
            .send()
            .await?
            .json::<T>()
            .await?;
        Ok(object)
    }

    async fn post(
        &self,
        url: &str,
        payload: &serde_json::Value,
        keypair: KeyPair,
    ) -> crate::Result<()> {
        let headers = crate::model::key_pair::sign_headers(payload, url, &keypair.private_key)?;
        reqwest::Client::new()
            .post(url)
            .headers(headers)
            .body(payload.to_string())
            .send()
            .await?;
        Ok(())
    }
}
