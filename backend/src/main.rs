use actix_web::{web, App, HttpServer};
use routes::routing;
use sqlx::postgres::PgPoolOptions;

mod model;
mod routes;

#[actix_web::main]
async fn main() {
    dotenvy::dotenv().unwrap();

    let db_url = std::env::var("DATABASE_URL").unwrap();
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .unwrap();

    let host_name = std::env::var("HOST_NAME").unwrap();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(host_name.clone()))
            .service(routing())
    })
    .bind(("0.0.0.0", 3000))
    .unwrap()
    .run()
    .await
    .unwrap();
}
