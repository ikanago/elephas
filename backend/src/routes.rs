use actix_web::{web, Responder};
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

pub fn route(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("/api")
            .route("/signup", web::post().to(self::signup::signup))
            .route("/login", web::post().to(self::login::login))
            .route("/me", web::get().to(self::me::me))
            .route("/me", web::post().to(self::me::update_me))
            .route(
                "/users/{user_name}",
                web::get().to(self::users::user_profile),
            )
            .route("/posts", web::post().to(self::post::create_post))
            .route(
                "/posts",
                web::get().to(self::post::get_posts_by_user_name),
            )
            .route("/follow", web::post().to(self::follow::create_follow))
            .route("/follow", web::delete().to(self::follow::delete_follow))
            .route(
                "/followees/{user_name}",
                web::get().to(self::follow::get_followees_by_user_name),
            )
            .route(
                "/followers/{user_name}",
                web::get().to(self::follow::get_followers_by_user_name),
            )
            .route(
                "/users/{user_name}/inbox",
                web::get().to(self::inbox::inbox),
            )
            .route("/ping", web::get().to(ping))
            .route("/reset-db", web::delete().to(self::reset_db::reset_db)),
    );
}

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
