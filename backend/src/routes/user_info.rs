use crate::{
    error::ServiceError,
    model::{KeyPairRepository, UserRepository},
};
use actix_session::Session;
use actix_web::{get, http::header::Accept, web, Responder};
use serde::Serialize;
use serde_json::{json, Value};
use sqlx::PgPool;
use utoipa::ToSchema;

#[derive(Clone, Serialize, ToSchema)]
pub struct UserInfoResponse {
    #[schema(example = "alice")]
    pub name: String,
}

#[utoipa::path(
    responses(
        (status = 200, body = UserInfoResponse, description = "Successfully fetched user info"),
        (status = 400, body = ErrorMessage, description = "BadRequest"),
        (status = 500, body = ErrorMessage, description = "InternalServerError"),
    )
)]
#[get("/users/{name}")]
async fn user_info(
    pool: web::Data<PgPool>,
    host_name: web::Data<String>,
    session: Session,
    accept: web::Header<Accept>,
    param: web::Path<String>,
) -> crate::Result<impl Responder> {
    user_info_service(
        pool.as_ref(),
        host_name.as_ref(),
        session,
        accept.to_string(),
        param.as_ref(),
    )
    .await
}

async fn user_info_service(
    pool: &PgPool,
    host_name: &str,
    session: Session,
    accept: String,
    name: &str,
) -> crate::Result<web::Json<Value>> {
    if accept.contains("application/ld+json") {
        return user_info_activity_json(pool, host_name, name).await;
    }

    let stored_user_id = session
        .get::<String>("user_id")
        .map_err(|_| ServiceError::InternalServerError)?
        .ok_or(ServiceError::WrongCredential)?;
    let user = pool.get_user_by_id(&stored_user_id).await.unwrap();
    if name != user.name {
        return Err(ServiceError::WrongCredential);
    }

    let res = UserInfoResponse { name: user.name };
    Ok(web::Json(json!(res)))
}

async fn user_info_activity_json(
    pool: &PgPool,
    host_name: &str,
    name: &str,
) -> crate::Result<web::Json<Value>> {
    let user = pool.get_user_by_name(&name).await?;
    let key_pair = pool.get_key_pair_by_user_id(user.id).await?;

    Ok(web::Json(json!({
        "@context": [
            "https://www.w3.org/ns/activitystreams",
            "https://w3id.org/security/v1",
        ],
        "type": "Person",
        "id": format!("https://{}/users/{}", host_name, name),
        "inbox": format!("https://{}/users/{}/inbox", host_name, name),
        "preferredUsername": name,
        "name": name,
        "icon": {
            "type": "Image",
            "url": "https://blog.ikanago.dev/_next/image?url=%2Fblog_icon.png&w=828&q=75",
            "name": "",
        },
        "publicKey": {
            "id": format!("https://{}/users/{}#main-key", host_name, name),
            "type": "Key",
            "owner": format!("https://{}/users/{}", host_name, name),
            "publicKeyPem": key_pair.public_key,
        },
    })))
}

#[cfg(test)]
mod tests {
    use crate::routes::signup::{signup_service, SignupCredential};

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
        let user_id = pool.get_user_by_name(&name).await.unwrap().id;

        session.insert("user_id", user_id).unwrap();

        // act
        let res = user_info_service(
            &pool,
            "example.com",
            session,
            "application/ld+json".to_string(),
            &name,
        )
        .await
        .unwrap();

        // assert
        let body = res.0;
        assert_eq!("Person", body["type"]);
        assert_eq!(name, body["name"]);
    }

    #[sqlx::test]
    async fn error_other_users_session(pool: PgPool) {
        // arrange
        let req = TestRequest::default().to_srv_request();
        let session = req.get_session();
        let first_user_name = "ikanago".to_string();
        let regstration = SignupCredential {
            name: first_user_name.clone(),
            password: "password".to_string(),
        };
        signup_service(&pool, regstration, session.clone())
            .await
            .unwrap();
        let first_user_id = pool.get_user_by_name(&first_user_name).await.unwrap().id;

        session.insert("user_id", first_user_id).unwrap();

        // act
        let res =
            user_info_service(&pool, "example.com", session, "*/*".to_string(), "mallory").await;

        // assert
        assert!(res.is_err());
    }
}
