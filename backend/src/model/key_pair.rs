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
    signature::Signer,
    RsaPrivateKey, RsaPublicKey,
};
use sqlx::PgPool;

#[derive(Clone, Debug)]
pub struct KeyPair {
    pub user_id: i32,
    pub private_key: String,
    pub public_key: String,
}

#[async_trait]
pub trait KeyPairRepository {
    async fn create_key_pair(
        &self,
        user_id: i32,
        private_key: &str,
        public_key: &str,
    ) -> anyhow::Result<()>;
    async fn get_key_pair_by_user_id(&self, user_id: i32) -> anyhow::Result<KeyPair>;
}

#[async_trait]
impl KeyPairRepository for PgPool {
    async fn create_key_pair(
        &self,
        user_id: i32,
        private_key: &str,
        public_key: &str,
    ) -> anyhow::Result<()> {
        sqlx::query!(
            r#"
            INSERT INTO user_key_pair ("user_id", "private_key", "public_key")
            VALUES (
                $1,
                $2,
                $3
            )
        "#,
            user_id,
            private_key,
            public_key
        )
        .execute(self)
        .await?;
        Ok(())
    }

    async fn get_key_pair_by_user_id(&self, user_id: i32) -> anyhow::Result<KeyPair> {
        let key_pair = sqlx::query_as!(
            KeyPair,
            r#"
            SELECT * FROM user_key_pair WHERE user_id = $1
        "#,
            user_id
        )
        .fetch_one(self)
        .await?;
        Ok(key_pair)
    }
}

const BITS: usize = 2048;

pub fn generate_key_pair() -> anyhow::Result<(String, String)> {
    let private_key = RsaPrivateKey::new(&mut thread_rng(), BITS)?;
    let public_key = RsaPublicKey::from(&private_key);
    let private_key_pem = private_key.to_pkcs8_pem(LineEnding::LF)?.to_string();
    let public_key_pem = public_key.to_public_key_pem(LineEnding::LF)?;
    Ok((private_key_pem, public_key_pem))
}

pub fn sign_headers(
    payload: &serde_json::Value,
    url: &str,
    private_key: &str,
) -> anyhow::Result<HeaderMap> {
    let url = Url::parse(url)?;
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
    let private_key = RsaPrivateKey::from_pkcs8_pem(&private_key)?;
    let signing_key = SigningKey::<Sha256>::new_with_prefix(private_key);
    let signature = signing_key.sign(signed_string.as_bytes());
    let signature = general_purpose::STANDARD.encode(signature);

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
