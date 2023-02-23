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
    sha2::{Digest, Sha256},
    signature::RandomizedSigner,
    RsaPrivateKey, RsaPublicKey,
};

#[derive(Clone, Debug)]
pub struct KeyPair {
    pub user_id: i32,
    pub private_key: String,
    pub public_key: String,
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
    payload: serde_json::Value,
    url: impl Into<Url>,
    private_key: &str,
) -> anyhow::Result<HeaderMap> {
    let url: Url = url.into();
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
    let signature = signing_key.sign_with_rng(&mut thread_rng(), signed_string.as_bytes());

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
