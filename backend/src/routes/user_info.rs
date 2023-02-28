use crate::{
    error::ServiceError,
    model::{KeyPairRepository, UserRepository},
};
use actix_session::Session;
use actix_web::{get, web, HttpResponse};
use reqwest::header::LOCATION;
use serde_json::json;
use sqlx::PgPool;

#[get("/users/{name}")]
async fn user_info(
    pool: web::Data<PgPool>,
    host_name: web::Data<String>,
    session: Session,
    param: web::Path<String>,
) -> crate::Result<HttpResponse> {
    user_info_service(
        pool.as_ref(),
        host_name.as_ref(),
        session,
        param.into_inner(),
    )
    .await
}

async fn user_info_service(
    pool: &PgPool,
    host_name: &str,
    session: Session,
    name: String,
) -> crate::Result<HttpResponse> {
    let stored_user_id = if let Some(user_id) = session
        .get::<String>("user_id")
        .map_err(|_| ServiceError::InternalServerError)?
    {
        user_id
    } else {
        return Ok(HttpResponse::SeeOther()
            .insert_header((LOCATION, "/login"))
            .finish());
    };
    let user = pool.get_user_by_id(&stored_user_id).await.unwrap();
    if name != user.name {
        return Ok(HttpResponse::SeeOther()
            .insert_header((LOCATION, "/login"))
            .finish());
    }

    let key_pair = pool.get_key_pair_by_user_id(user.id).await.unwrap();

    Ok(HttpResponse::Ok().json(json!({
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
    use crate::routes::signup::{signup_service, Regestration};

    use super::*;

    use actix_session::SessionExt;
    use actix_web::test::TestRequest;
    use reqwest::StatusCode;

    #[sqlx::test]
    async fn error_other_users_session(pool: PgPool) {
        // arrange
        let req = TestRequest::default().to_srv_request();
        let session = req.get_session();
        let first_user_name = "ikanago".to_string();
        let regstration = Regestration {
            name: first_user_name.clone(),
            password: "password".to_string(),
        };
        signup_service(&pool, regstration, session.clone())
            .await
            .unwrap();
        let first_user_id = pool.get_user_by_name(&first_user_name).await.unwrap().id;

        session.insert("user_id", first_user_id).unwrap();

        // act
        let res = user_info_service(&pool, "example.com", session, "mallory".to_string())
            .await
            .unwrap();

        // assert
        assert_eq!(res.status(), StatusCode::SEE_OTHER);
    }
}
