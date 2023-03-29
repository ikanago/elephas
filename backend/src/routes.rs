use actix_web::{get, web, Responder, Scope};
use utoipa::OpenApi;

mod host_meta;
mod inbox;
mod login;
mod me;
mod post;
mod reset_db;
mod signup;
mod user_profile;
mod webfinger;

pub fn routing() -> Scope {
    web::scope("/api")
        .service(self::signup::signup)
        .service(self::login::login)
        .service(self::me::me)
        .service(self::user_profile::user_profile)
        .service(self::post::create_post)
        .service(self::post::get_posts_by_user_id)
        .service(self::inbox::inbox)
        .service(self::webfinger::webfinger)
        .service(self::host_meta::host_meta)
        .service(ping)
        // TODO: disable in production
        .service(self::reset_db::reset_db)
}

#[get("/ping")]
#[tracing::instrument]
async fn ping() -> impl Responder {
    "pong"
}

#[derive(OpenApi)]
#[openapi(
    paths(
        self::signup::signup,
        self::login::login,
        self::me::me,
        self::user_profile::user_profile,
        self::post::create_post,
        self::post::get_posts_by_user_id,
    ),
    components(schemas(
        self::signup::SignupCredential,
        self::login::LoginCredential,
        crate::service::user_profile::UserProfile,
        self::post::NewPost,
        crate::model::post::Post,
        crate::error::ErrorMessage
    ))
)]
pub struct ApiDoc;
