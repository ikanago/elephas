use crate::model::{
    ap_person::{ApPerson, ApPersonRepository},
    webfinger::RemoteWebfingerRepository,
};

pub async fn resolve(
    user_name: &str,
    host_name: &str,
    remote_webfinger_repository: impl RemoteWebfingerRepository,
    ap_person_repository: impl ApPersonRepository,
) -> crate::Result<ApPerson> {
    let webfinger = remote_webfinger_repository
        .fetch_webfinger(&user_name, &host_name)
        .await?;
    let href = webfinger
        .links
        .iter()
        .find(|link| link.rel == "self")
        .ok_or_else(|| crate::error::ServiceError::InternalServerError)?
        .href
        .clone()
        .ok_or_else(|| crate::error::ServiceError::InternalServerError)?;

    let ap_person = ap_person_repository.fetch_ap_person(href.as_str()).await?;
    Ok(ap_person)
}

#[cfg(test)]
mod tests {
    use crate::model::{
        ap_person::{ApPerson, MockApPersonRepository},
        webfinger::{MockRemoteWebfingerRepository, Webfinger, WebfingerLink},
    };

    use super::*;
    use mockall::predicate::*;

    #[actix_web::test]
    async fn resolve_remote_user() {
        let mut mock_remote_webfinger_repository = MockRemoteWebfingerRepository::new();
        mock_remote_webfinger_repository
            .expect_fetch_webfinger()
            .with(eq("test"), eq("test.ikanago.dev"))
            .returning(|_, _| {
                Ok(Webfinger {
                    subject: "acct:test@test.ikanago.dev".to_string(),
                    links: vec![WebfingerLink {
                        href: Some("https://test.ikanago.dev/users/test".to_string()),
                        rel: "self".to_string(),
                        r#type: None,
                    }],
                })
            });

        let mut mock_ap_person_repository = MockApPersonRepository::new();
        mock_ap_person_repository
            .expect_fetch_ap_person()
            .with(eq("https://test.ikanago.dev/users/test"))
            .returning(|_| {
                Ok(ApPerson {
                    id: "https://test.ikanago.dev/users/test".to_string(),
                    r#type: "Person".to_string(),
                    preferred_username: "test".to_string(),
                    name: Some("test".to_string()),
                    summary: Some("test".to_string()),
                    inbox: "https://test.ikanago.dev/users/test/inbox".to_string(),
                })
            });

        let person = resolve(
            "test",
            "test.ikanago.dev",
            mock_remote_webfinger_repository,
            mock_ap_person_repository,
        )
        .await
        .unwrap();
        assert_eq!(
            person,
            ApPerson {
                id: "https://test.ikanago.dev/users/test".to_string(),
                r#type: "Person".to_string(),
                preferred_username: "test".to_string(),
                name: Some("test".to_string()),
                summary: Some("test".to_string()),
                inbox: "https://test.ikanago.dev/users/test/inbox".to_string(),
            }
        )
    }
}
