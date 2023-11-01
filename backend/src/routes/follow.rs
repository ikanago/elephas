use actix_session::Session;
use actix_web::{web, HttpResponse, Responder};
use sqlx::PgPool;

use crate::{
    error::ServiceError,
    model::{
        follow::{Follow, FollowRepository},
        user::{parse_user_and_host_name, UserRepository},
        user_profile::UserProfile,
    },
    SESSION_KEY,
};

#[utoipa::path(
    post,
    path = "/follow",
    request_body = Follow,
    responses(
        (status = 204, description = "Successfully follow the user"),
        (status = 401, body = ErrorMessage, description = "Unauthorized"),
        (status = 500, body = ErrorMessage, description = "InternalServerError"),
    )
)]
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
    if pool.get_user_by_name(&follow_to_name).await.is_err() {
        if is_follow_to_remote {
            crate::service::remote_user::create_remote_user(
                &pool,
                &follow_to_username,
                &follow_to_host_name,
            )
            .await?;
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
        crate::service::activitypub::follow_remote_person(
            &pool,
            &host_name,
            &follow_from_name,
            &follow_to_username,
            &follow_to_host_name,
        )
        .await?;
    }

    Ok(HttpResponse::NoContent().finish())
}

#[utoipa::path(
    delete,
    path = "/follow",
    request_body = Follow,
    responses(
        (status = 204, description = "Successfully remove the user"),
        (status = 401, body = ErrorMessage, description = "Unauthorized"),
        (status = 500, body = ErrorMessage, description = "InternalServerError"),
    )
)]
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
    get,
    path = "/followees/{user_name}",
    responses(
        (status = 200, body = Vec<UserProfile>, description = "Successfully get followees for a user"),
        (status = 401, body = ErrorMessage, description = "Unauthorized"),
        (status = 500, body = ErrorMessage, description = "InternalServerError"),
    )
)]
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
    Ok(HttpResponse::Ok().json(folloees))
}

#[utoipa::path(
    get,
    path = "/followers/{user_name}",
    responses(
        (status = 200, body = Vec<UserProfile>, description = "Successfully get followers for a user"),
        (status = 401, body = ErrorMessage, description = "Unauthorized"),
        (status = 500, body = ErrorMessage, description = "InternalServerError"),
    )
)]
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
    Ok(HttpResponse::Ok().json(folloers))
}
