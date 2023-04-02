use actix_web::{get, web, Responder, Scope};
use utoipa::OpenApi;

mod follow;
mod host_meta;
mod inbox;
mod login;
mod me;
mod post;
mod reset_db;
mod signup;
mod users;
mod webfinger;

pub fn routing() -> Scope {
    web::scope("/api")
        .service(self::signup::signup)
        .service(self::login::login)
        .service(self::me::me)
        .service(self::me::update_me)
        .service(self::users::user_profile)
        .service(self::post::create_post)
        .service(self::post::get_posts_by_user_name)
        .service(self::follow::create_follow)
        .service(self::follow::delete_follow)
        .service(self::follow::get_followees_by_user_name)
        .service(self::follow::get_followers_by_user_name)
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
        self::me::update_me,
        self::users::user_profile,
        self::post::create_post,
        self::post::get_posts_by_user_name,
        self::follow::create_follow,
        self::follow::delete_follow,
        self::follow::get_followees_by_user_name,
        self::follow::get_followers_by_user_name,
    ),
    components(schemas(
        self::signup::SignupCredential,
        self::login::LoginCredential,
        crate::service::user_profile::UserProfile,
        crate::service::user_profile::UserProfileUpdate,
        self::post::NewPost,
        crate::model::post::Post,
        crate::model::follow::Follow,
        crate::error::ErrorMessage,
    ))
)]
pub struct ApiDoc;
