use async_trait::async_trait;
use reqwest::{header::ACCEPT, Client};
use serde::Deserialize;

#[allow(unused)]
#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ApPerson {
    pub id: String,
    pub r#type: String,
    pub preferred_username: String,
    pub name: Option<String>,
    pub summary: Option<String>,
    pub inbox: String,
}

#[mockall::automock]
#[async_trait]
pub trait ApPersonRepository {
    async fn fetch_ap_person(&self, href: &str) -> crate::Result<ApPerson>;
}

#[derive(Clone)]
pub struct ApPersonRepositoryImpl;

#[async_trait]
impl ApPersonRepository for ApPersonRepositoryImpl {
    async fn fetch_ap_person(&self, href: &str) -> crate::Result<ApPerson> {
        let accept = r#"application/ld+json; profile="https://www.w3.org/ns/activitystreams"; charset=utf-8"#;
        let object = Client::new()
            .get(href)
            .header(ACCEPT, accept)
            .send()
            .await?
            .json::<ApPerson>()
            .await?;
        Ok(object)
    }
}
