use actix_web::{get, web, Responder, Scope};

mod home;
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
        .service(self::home::home)
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
