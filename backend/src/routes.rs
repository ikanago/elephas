use actix_web::{web, Scope};

mod host_meta;
mod inbox;
mod signup;
mod user_info;
mod webfinger;

pub fn routing() -> Scope {
    web::scope("/")
        .service(self::signup::signup)
        .service(self::user_info::user_info)
        .service(self::inbox::inbox)
        .service(self::webfinger::webfinger)
        .service(self::host_meta::host_meta)
}
