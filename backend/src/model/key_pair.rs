use async_trait::async_trait;
use base64::{engine::general_purpose, Engine as _};
use chrono::Local;
use rand::thread_rng;
use reqwest::{
    header::{HeaderMap, CONTENT_TYPE, DATE, HOST},
    Url,
};
use rsa::{
    pkcs1v15::SigningKey,
    pkcs8::{spki::EncodePublicKey, DecodePrivateKey},
    pkcs8::{EncodePrivateKey, LineEnding},
    sha2::Sha256,
    signature::{SignatureEncoding, Signer},
    RsaPrivateKey, RsaPublicKey,
};
use sqlx::PgPool;

use crate::error::ServiceError;

#[derive(Clone, Debug)]
pub struct KeyPair {
    pub user_name: String,
    pub private_key: String,
    pub public_key: String,
}

#[async_trait]
pub trait KeyPairRepository {
    async fn save_key_pair(&self, key_pair: KeyPair) -> crate::Result<()>;
    async fn get_key_pair_by_user_name(&self, user_name: String) -> crate::Result<KeyPair>;
}

#[async_trait]
impl KeyPairRepository for PgPool {
    async fn save_key_pair(&self, key_pair: KeyPair) -> crate::Result<()> {
        sqlx::query!(
            r#"
            INSERT INTO user_key_pair ("user_name", "private_key", "public_key")
            VALUES (
                $1,
                $2,
                $3
            )
        "#,
            key_pair.user_name,
            key_pair.private_key,
            key_pair.public_key
        )
        .execute(self)
        .await?;
        Ok(())
    }

    async fn get_key_pair_by_user_name(&self, user_name: String) -> crate::Result<KeyPair> {
        let key_pair = sqlx::query_as!(
            KeyPair,
            r#"
            SELECT * FROM user_key_pair WHERE user_name = $1
        "#,
            user_name
        )
        .fetch_one(self)
        .await?;
        Ok(key_pair)
    }
}

const BITS: usize = 2048;

pub fn generate_key_pair() -> crate::Result<(String, String)> {
    if let Ok(env) = std::env::var("ENV") {
        if env == "test" {
            return Ok((String::new(), String::new()));
        }
    }

    let private_key = RsaPrivateKey::new(&mut thread_rng(), BITS).expect("private key is created successfully; error is generated when nprimes < 2, but nprimes is 2.");
    let public_key = RsaPublicKey::from(&private_key);
    let private_key_pem = private_key
        .to_pkcs8_pem(LineEnding::LF)
        .unwrap()
        .to_string();
    let public_key_pem = public_key.to_public_key_pem(LineEnding::LF).unwrap();
    Ok((private_key_pem, public_key_pem))
}

pub fn sign_headers(
    payload: &serde_json::Value,
    url: &str,
    private_key: &str,
) -> crate::Result<HeaderMap> {
    let url = Url::parse(url)
        .map_err(|_| ServiceError::InvalidActivityPubRequest(format!("Invalid URL: {}", url)))?;
    let now = Local::now().to_rfc2822();
    let digest = sha256::digest(payload.to_string());
    let digest = general_purpose::STANDARD.encode(digest);

    let host_name = url.host_str().unwrap();
    let signed_string = [
        format!("(request-target): post {}", url.path()),
        format!("host: {}", host_name),
        format!("date: {}", now),
        format!("digest: SHA-256={}", digest),
    ]
    .join("\n");
    let private_key =
        RsaPrivateKey::from_pkcs8_pem(&private_key).expect("Only valid private key must be stored");
    let signing_key = SigningKey::<Sha256>::new(private_key);
    let signature: rsa::pkcs1v15::Signature = signing_key.sign(signed_string.as_bytes());
    let signature = general_purpose::STANDARD.encode(signature.to_bytes());

    let headers = [
        (HOST, host_name),
        (DATE, &now),
        (CONTENT_TYPE, "application/activity+json"),
        ("Digest".parse().unwrap(), &format!("SHA-256={}", digest)),
        (
            "Signature".parse().unwrap(),
            &[
                r#"keyId="https://ikanago.dev/users/test""#,
                r#"algorithm="rsa-sha256""#,
                r#"headers="(request-target) host date digest""#,
                &format!(r#"signature="{}""#, signature),
            ]
            .join(","),
        ),
    ]
    .into_iter()
    .map(|(k, v)| (k, v.parse().unwrap()))
    .collect::<HeaderMap>();
    Ok(headers)
}
