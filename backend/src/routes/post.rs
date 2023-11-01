use actix_session::Session;
use actix_web::{web, HttpResponse, Responder};
use chrono::Utc;
use serde::Deserialize;
use sqlx::PgPool;
use tracing::debug;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::{
    error::ServiceError,
    model::post::{Post, PostRepository},
    SESSION_KEY,
};

#[derive(Clone, Debug, Deserialize, ToSchema)]
pub struct NewPost {
    pub content: String,
}

#[utoipa::path(
    post,
    path = "/posts",
    request_body = NewPost,
    responses(
        (status = 204, description = "Successfully created a post"),
        (status = 401, body = ErrorMessage, description = "Unauthorized"),
        (status = 500, body = ErrorMessage, description = "InternalServerError"),
    )
)]
#[tracing::instrument(skip(pool, session))]
pub async fn create_post(
    pool: web::Data<PgPool>,
    body: web::Json<NewPost>,
    session: Session,
) -> crate::Result<impl Responder> {
    create_post_service(pool.as_ref(), body.into_inner(), session).await
}

async fn create_post_service(
    pool: &PgPool,
    NewPost { content }: NewPost,
    session: Session,
) -> crate::Result<impl Responder> {
    let user_name = session
        .get::<String>(SESSION_KEY)
        .map_err(|_| ServiceError::InternalServerError)?
        .ok_or(ServiceError::WrongCredential)?;
    let post = Post {
        id: Uuid::new_v4().to_string(),
        user_name,
        content,
        published_at: Utc::now(),
    };
    pool.save_post(post).await?;
    Ok(HttpResponse::NoContent().finish())
}

#[utoipa::path(
    get,
    path = "/posts",
    responses(
        (status = 200, body = Vec<Post>, description = "Successfully get posts for a user"),
        (status = 401, body = ErrorMessage, description = "Unauthorized"),
        (status = 500, body = ErrorMessage, description = "InternalServerError"),
    )
)]
#[tracing::instrument(skip(pool, session))]
pub async fn get_posts_by_user_name(
    pool: web::Data<PgPool>,
    session: Session,
) -> crate::Result<impl Responder> {
    // TODO: this retrieves posts from user logging in, but it should be able to retrieve posts from any user.
    let user_name = session
        .get::<String>(SESSION_KEY)
        .map_err(|_| ServiceError::InternalServerError)?
        .ok_or(ServiceError::WrongCredential)?;
    let posts = pool.get_posts_by_user_name(&user_name).await.unwrap();
    debug!(posts = ?posts);
    Ok(HttpResponse::Ok().json(posts))
}
