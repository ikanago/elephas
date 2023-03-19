use actix_web::{get, web, Responder, Scope};
use utoipa::OpenApi;

mod me;
mod host_meta;
mod inbox;
mod login;
mod reset_db;
mod signup;
mod user_info;
mod webfinger;

pub fn routing() -> Scope {
    web::scope("/api")
        .service(self::signup::signup)
        .service(self::login::login)
        .service(self::me::me)
        .service(self::user_info::user_info)
        .service(self::inbox::inbox)
        .service(self::webfinger::webfinger)
        .service(self::host_meta::host_meta)
        .service(ping)
        // TODO: disable in production
        .service(self::reset_db::reset_db)
}

#[get("/ping")]
async fn ping() -> impl Responder {
    "pong"
}

#[derive(OpenApi)]
#[openapi(
    paths(
        self::signup::signup,
        self::login::login,
        self::me::me,
        self::user_info::user_info
    ),
    components(schemas(
        self::signup::SignupCredential,
        self::login::LoginCredential,
        self::user_info::UserInfoResponse,
        crate::error::ErrorMessage
    ))
)]
pub struct ApiDoc;
