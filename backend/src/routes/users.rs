use crate::{
    model::{KeyPairRepository, UserRepository},
    service::user_profile::get_user_profile_service,
};
use actix_web::{get, http::header::Accept, web, Responder};
use serde_json::{json, Value};
use sqlx::PgPool;

#[utoipa::path(
    responses(
        (status = 200, body = UserProfile, description = "Successfully fetched user info"),
        (status = 400, body = ErrorMessage, description = "BadRequest"),
        (status = 500, body = ErrorMessage, description = "InternalServerError"),
    )
)]
#[get("/users/{name}")]
#[tracing::instrument(skip(pool, host_name))]
async fn user_profile(
    pool: web::Data<PgPool>,
    host_name: web::Data<String>,
    accept: web::Header<Accept>,
    user_name: web::Path<String>,
) -> crate::Result<impl Responder> {
    if accept.to_string().contains("application/ld+json") {
        return user_info_activity_json(&pool, &host_name, &user_name).await;
    }

    let user = get_user_profile_service(&pool, &user_name).await?;
    Ok(web::Json(json!(user)))
}

async fn user_info_activity_json(
    pool: &PgPool,
    host_name: &str,
    name: &str,
) -> crate::Result<web::Json<Value>> {
    let user = pool.get_user_by_name(&name).await?;
    let key_pair = pool.get_key_pair_by_user_name(user.name).await?;

    let base_url = format!("https://{}/api/users/{}", host_name, name);
    Ok(web::Json(json!({
        "@context": [
            "https://www.w3.org/ns/activitystreams",
            "https://w3id.org/security/v1",
        ],
        "type": "Person",
        // TODO: use immutable ID
        "id": base_url.clone(),
        "url": base_url.clone(),
        "inbox": format!("{}/inbox", base_url),
        "preferredUsername": name,
        "name": name,
        "discoverable": true,
        "icon": {
            "type": "Image",
            "url": "https://blog.ikanago.dev/_next/image?url=%2Fblog_icon.png&w=828&q=75",
            "name": "",
        },
        "publicKey": {
            "id": format!("{}#main-key", base_url),
            "type": "Key",
            "owner": base_url,
            "publicKeyPem": key_pair.public_key,
        },
    })))
}

#[cfg(test)]
mod tests {
    use crate::{
        routes::signup::{signup_service, SignupCredential},
        SESSION_KEY,
    };

    use super::*;

    use actix_session::SessionExt;
    use actix_web::test::TestRequest;

    #[sqlx::test]
    async fn get_user_info_activity_json(pool: PgPool) {
        // arrange
        let req = TestRequest::default().to_srv_request();
        let session = req.get_session();
        let name = "ikanago".to_string();
        let regstration = SignupCredential {
            name: name.clone(),
            password: "password".to_string(),
        };
        signup_service(&pool, regstration, session.clone())
            .await
            .unwrap();
        let user_name = pool.get_user_by_name(&name).await.unwrap().name;

        session.insert(SESSION_KEY, user_name).unwrap();

        // act
        let res = user_info_activity_json(&pool, "example.com", &name)
            .await
            .unwrap();

        // assert
        let body = res.0;
        assert_eq!("Person", body["type"]);
        assert_eq!(name, body["name"]);
    }
}
