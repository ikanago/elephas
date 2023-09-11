use actix_files::{Files, NamedFile};
use actix_session::{storage::RedisActorSessionStore, SessionMiddleware};
use actix_web::{cookie::Key, web, App, HttpServer};
use backend::routes::routing;
use base64::engine::{general_purpose, Engine};
use sqlx::postgres::PgPoolOptions;
use tracing::info;
use tracing_actix_web::TracingLogger;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

#[actix_web::main]
async fn main() {
    tracing_subscriber::registry()
        .with(EnvFilter::from_default_env())
        .with(tracing_subscriber::fmt::layer())
        .init();

    // In CI, we don't have a .env file, so we just ignore it
    if let Err(_) = dotenvy::dotenv() {
        info!("No .env file found, using environment variables instead")
    }

    let db_url = std::env::var("DATABASE_URL").unwrap();
    info!(db_url);
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .unwrap();

    let host_name = if std::env::var("ENV").unwrap_or_default() == "dev" {
        "localhost:3000".to_string()
    } else {
        std::env::var("HOST_NAME").unwrap()
    };
    info!(host_name);
    let cookie_key = {
        let key = std::env::var("COOKIE_KEY").unwrap();
        let key = general_purpose::STANDARD.decode(key.as_bytes()).unwrap();
        Key::from(&key)
    };
    let redis_url = std::env::var("REDIS_URL").unwrap();
    info!(redis_url);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(host_name.clone()))
            .wrap(
                SessionMiddleware::builder(
                    RedisActorSessionStore::new(&redis_url),
                    cookie_key.clone(),
                )
                .build(),
            )
            .wrap(TracingLogger::default())
            .service(routing())
            .service(backend::routes::webfinger::webfinger)
            .service(backend::routes::host_meta::host_meta)
            .service(Files::new("/assets", "../frontend/dist/assets"))
            .service(
                web::resource("/{_:.*}")
                    .to(|| async { NamedFile::open("../frontend/dist/index.html") }),
            )
    })
    .bind(("0.0.0.0", 3001))
    .unwrap()
    .bind(("::", 3000))
    .unwrap()
    .run()
    .await
    .unwrap();
}
