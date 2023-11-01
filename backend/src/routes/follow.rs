use actix_session::Session;
use actix_web::{delete, get, post, web, HttpResponse, Responder};
use serde_json::json;
use sqlx::PgPool;
use tracing::debug;

use crate::{
    error::ServiceError,
    model::{
        follow::{Follow, FollowRepository},
        key_pair::sign_headers,
        user::{parse_user_and_host_name, UserRepository},
        user_profile::UserProfile,
        KeyPairRepository,
    },
    SESSION_KEY,
};

#[utoipa::path(
    request_body = Follow,
    responses(
        (status = 204, description = "Successfully follow the user"),
        (status = 401, body = ErrorMessage, description = "Unauthorized"),
        (status = 500, body = ErrorMessage, description = "InternalServerError"),
    )
)]
#[post("/follow")]
#[tracing::instrument(skip(pool, session))]
pub async fn create_follow(
    pool: web::Data<PgPool>,
    body: web::Json<Follow>,
    host_name: web::Data<String>,
    session: Session,
) -> crate::Result<impl Responder> {
    let user_name = session
        .get::<String>(SESSION_KEY)
        .map_err(|_| ServiceError::InternalServerError)?
        .ok_or(ServiceError::WrongCredential)?;

    let Follow {
        follow_from_name,
        follow_to_name,
    } = body.into_inner();
    if user_name != follow_from_name {
        return Err(ServiceError::WrongCredential);
    }

    let (follow_to_username, follow_to_host_name) = parse_user_and_host_name(&follow_to_name)
        .unwrap_or((follow_to_name.clone(), String::new()));
    let is_follow_to_remote = !follow_to_host_name.is_empty();
    debug!(follow_to_username = ?follow_to_username, host_name = ?follow_to_host_name, is_follow_to_remote);
    if pool.get_user_by_name(&follow_to_name).await.is_err() {
        if is_follow_to_remote {
            let person = crate::service::remote_user::resolve(
                &follow_to_username,
                &follow_to_host_name,
                crate::model::webfinger::RemoteWebfingerRepositoryImpl,
                crate::model::ap_person::ApPersonRepositoryImpl,
            )
            .await?;
            debug!(person = ?person);
            let user_profile: UserProfile = person.into();
            let user = crate::model::user::User {
                // TODO: use ID
                name: user_profile.name.clone(),
                display_name: user_profile.display_name.clone(),
                summary: user_profile.summary.clone(),
                avatar_url: user_profile.avatar_url.clone(),
                ..Default::default()
            };
            pool.save_user(user).await?;
        } else {
            return Err(ServiceError::UserNotFound);
        }
    }

    // TODO: check if the user is already followed

    let follow = Follow {
        follow_from_name: follow_from_name.clone(),
        follow_to_name: follow_to_username.clone(),
    };
    pool.save_follow(follow).await?;

    if is_follow_to_remote {
        let person = crate::service::remote_user::resolve(
            &follow_to_username,
            &follow_to_host_name,
            crate::model::webfinger::RemoteWebfingerRepositoryImpl,
            crate::model::ap_person::ApPersonRepositoryImpl,
        )
        .await?;
        let payload = json!({
            "@context": [
                "https://www.w3.org/ns/activitystreams",
                "https://w3id.org/security/v1",
            ],
            "type": "Follow",
            "actor": format!("https://{}/api/users/{}", host_name.into_inner(), follow_from_name),
            "object": person.id,
        });
        let keypair = pool.get_key_pair_by_user_name(follow_from_name).await?;
        let headers = sign_headers(&payload, &person.inbox, &keypair.private_key)?;
        debug!(payload = ?payload, keypair = ?keypair, headers = ?headers);
        reqwest::Client::new()
            .post(&person.inbox)
            .headers(headers)
            .json(&payload)
            .send()
            .await?;
    }

    Ok(HttpResponse::NoContent().finish())
}

#[utoipa::path(
    request_body = Follow,
    responses(
        (status = 204, description = "Successfully remove the user"),
        (status = 401, body = ErrorMessage, description = "Unauthorized"),
        (status = 500, body = ErrorMessage, description = "InternalServerError"),
    )
)]
#[delete("/follow")]
#[tracing::instrument(skip(pool, session))]
pub async fn delete_follow(
    pool: web::Data<PgPool>,
    body: web::Json<Follow>,
    session: Session,
) -> crate::Result<impl Responder> {
    let user_name = session
        .get::<String>(SESSION_KEY)
        .map_err(|_| ServiceError::InternalServerError)?
        .ok_or(ServiceError::WrongCredential)?;

    if user_name != body.follow_from_name {
        return Err(ServiceError::WrongCredential);
    }

    pool.delete_follow(body.into_inner()).await?;
    Ok(HttpResponse::NoContent().finish())
}

#[utoipa::path(
    responses(
        (status = 200, body = Vec<UserProfile>, description = "Successfully get followees for a user"),
        (status = 401, body = ErrorMessage, description = "Unauthorized"),
        (status = 500, body = ErrorMessage, description = "InternalServerError"),
    )
)]
#[get("/followees/{name}")]
#[tracing::instrument(skip(pool))]
pub async fn get_followees_by_user_name(
    pool: web::Data<PgPool>,
    user_name: web::Path<String>,
) -> crate::Result<impl Responder> {
    let folloees = pool
        .get_followees_by_name(&user_name)
        .await?
        .into_iter()
        .map(|user| UserProfile::from(user))
        .collect::<Vec<_>>();
    debug!(folloees = ?folloees);
    Ok(HttpResponse::Ok().json(folloees))
}

#[utoipa::path(
    responses(
        (status = 200, body = Vec<UserProfile>, description = "Successfully get followers for a user"),
        (status = 401, body = ErrorMessage, description = "Unauthorized"),
        (status = 500, body = ErrorMessage, description = "InternalServerError"),
    )
)]
#[get("/followers/{name}")]
#[tracing::instrument(skip(pool))]
pub async fn get_followers_by_user_name(
    pool: web::Data<PgPool>,
    user_name: web::Path<String>,
) -> crate::Result<impl Responder> {
    let folloers = pool
        .get_followers_by_name(&user_name)
        .await?
        .into_iter()
        .map(|user| UserProfile::from(user))
        .collect::<Vec<_>>();
    debug!(folloers = ?folloers);
    Ok(HttpResponse::Ok().json(folloers))
}
