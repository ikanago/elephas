use actix_web::{get, web, Responder, Scope};
use utoipa::OpenApi;

mod follow;
// Make public to use directly in app.rs.
// If this is served in `routing()` below, we have to prepend `web::scope("")` to the path.
// This captures all requests other than those starting with `/api`.
// Then we cannot serve all static files under `/`.
pub mod host_meta;
mod inbox;
mod login;
mod me;
pub mod nodeinfo;
mod post;
mod reset_db;
mod signup;
pub mod users;
pub mod webfinger;

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
        .service(ping)
        // TODO: disable in production
        .service(self::reset_db::reset_db)
}

#[get("/ping")]
#[tracing::instrument]
async fn ping() -> impl Responder {
    "pong\n"
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
        self::login::LoginCredential,
        self::post::NewPost,
        self::signup::SignupCredential,
        crate::service::user_profile::UserProfileUpdate,
        crate::model::follow::Follow,
        crate::model::post::Post,
        crate::model::user_profile::UserProfile,
        crate::error::ErrorMessage,
    )),
    servers(
        (url = "http://localhost:3000/api", description = "Localhost"),
    )
)]
pub struct ApiDoc;
