use actix_web::{get, web, Responder, Scope};

mod home;
mod host_meta;
mod inbox;
mod login;
mod signup;
mod user_info;
mod webfinger;

pub fn routing() -> Scope {
    web::scope("")
        .service(self::signup::signup)
        .service(self::login::login)
        .service(self::home::home)
        .service(self::user_info::user_info)
        .service(self::inbox::inbox)
        .service(self::webfinger::webfinger)
        .service(self::host_meta::host_meta)
        .service(ping)
}

#[get("/ping")]
async fn ping() -> impl Responder {
    "pong"
}
