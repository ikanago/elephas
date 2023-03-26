use actix_session::Session;
use actix_web::{post, web, Responder, HttpResponse, get};
use chrono::Utc;
use serde::Deserialize;
use sqlx::PgPool;
use tracing::debug;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::{model::post::{Post, PostRepository}, error::ServiceError};


#[derive(Clone, Debug, Deserialize, ToSchema)]
pub struct NewPost {
    pub content: String
}

#[utoipa::path(
    request_body = NewPost,
    responses(
        (status = 200, description = "Successfully logged in"),
        (status = 401, body = ErrorMessage, description = "Unauthorized"),
        (status = 500, body = ErrorMessage, description = "InternalServerError"),
    )
)]
#[post("/posts")]
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
    let user_id = session
        .get::<String>("user_id")
        .map_err(|_| ServiceError::InternalServerError)?
        .ok_or(ServiceError::WrongCredential)?;
    let post = Post {
        id: Uuid::new_v4().to_string(),
        user_id,
        content,
        published_at: Utc::now(),
    };
    pool.save_post(post).await?;
    Ok(HttpResponse::Ok().finish())
}

#[utoipa::path(
    responses(
        (status = 200, body = Vec<Post>, description = "Successfully get posts for a user"),
        (status = 401, body = ErrorMessage, description = "Unauthorized"),
        (status = 500, body = ErrorMessage, description = "InternalServerError"),
    )
)]
#[get("/posts")]
#[tracing::instrument(skip(pool, session))]
pub async fn get_posts_by_user_id(pool: web::Data<PgPool>, session: Session) -> crate::Result<impl Responder> {
    let user_id = session
        .get::<String>("user_id")
        .map_err(|_| ServiceError::InternalServerError)?
        .ok_or(ServiceError::WrongCredential)?;
    let posts = pool.get_posts_by_user_id(&user_id).await.unwrap();
    debug!(posts = ?posts);
    Ok(HttpResponse::Ok().json(posts))
}
