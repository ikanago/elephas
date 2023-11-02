use serde_json::json;
use sqlx::PgPool;

use crate::model::{
    activitypub::ActivityPubRequestRepository, webfinger::RemoteWebfingerRepository,
    KeyPairRepository,
};

pub async fn follow_remote_person(
    pool: &PgPool,
    host_name: &str,
    follow_from_user_name: &str,
    follow_to_user_name: &str,
    follow_to_host_name: &str,
    activitypub_request_repository: &impl ActivityPubRequestRepository,
    remote_webfinger_repository: &impl RemoteWebfingerRepository,
) -> crate::Result<()> {
    let person = crate::service::remote_user::resolve(
        follow_to_user_name,
        follow_to_host_name,
        remote_webfinger_repository,
        activitypub_request_repository,
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

    activitypub_request_repository
        .post(&person.inbox, &payload, keypair)
        .await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use mockall::predicate::eq;

    use crate::model::{
        activitypub::{MockActivityPubRequestRepository, Person},
        webfinger::{MockRemoteWebfingerRepository, Webfinger, WebfingerLink},
        KeyPair, User, UserRepository,
    };

    use super::*;

    async fn prepare_remote_user(pool: &PgPool, user_name: &str, key_pair: &KeyPair) {
        pool.save_user(User {
            name: user_name.to_string(),
            ..Default::default()
        })
        .await
        .unwrap();
        pool.save_key_pair(key_pair.clone()).await.unwrap();
    }

    #[sqlx::test]
    async fn follow_remote_person_test(pool: PgPool) {
        let mut remote_webfinger_repository = MockRemoteWebfingerRepository::new();
        remote_webfinger_repository
            .expect_fetch_webfinger()
            .with(eq("test_remote"), eq("remote.ikanago.dev"))
            .returning(|_, _| {
                Ok(Webfinger {
                    subject: "acct:test_remote@remote.ikanago.dev".to_string(),
                    links: vec![WebfingerLink {
                        href: Some("https://remote.ikanago.dev/users/test_remote".to_string()),
                        rel: "self".to_string(),
                        r#type: None,
                    }],
                })
            });

        let mut activitypub_request_repository = MockActivityPubRequestRepository::new();
        activitypub_request_repository
            .expect_get()
            .with(eq("https://remote.ikanago.dev/users/test_remote"))
            .returning(|_| {
                Ok(Person {
                    id: "https://remote.ikanago.dev/users/test_remote".to_string(),
                    r#type: "Person".to_string(),
                    preferred_username: "test_remote".to_string(),
                    name: Some("test_remote".to_string()),
                    summary: Some("test_remote".to_string()),
                    inbox: "https://remote.ikanago.dev/users/test_remote/inbox".to_string(),
                })
            });

        let key_pair = KeyPair {
            user_name: "test_local".to_string(),
            private_key: "test_private_key".to_string(),
            public_key: "test_public_key".to_string(),
        };
        activitypub_request_repository
            .expect_post()
            .with(
                eq("https://remote.ikanago.dev/users/test_remote/inbox"),
                eq(json!({
                    "@context": [
                        "https://www.w3.org/ns/activitystreams",
                        "https://w3id.org/security/v1",
                    ],
                    "type": "Follow",
                    "actor": "https://ikanago.dev/api/users/test_local",
                    "object": "https://remote.ikanago.dev/users/test_remote",
                })),
                eq(key_pair.clone()),
            )
            .returning(|_, _, _| Ok(()));

        prepare_remote_user(&pool, "test_local", &key_pair).await;

        follow_remote_person(
            &pool,
            "ikanago.dev",
            "test_local",
            "test_remote",
            "remote.ikanago.dev",
            &activitypub_request_repository,
            &remote_webfinger_repository,
        )
        .await
        .unwrap();
    }
}
