use actix_files::Files;
use actix_web::{middleware::Logger, web, App, HttpServer};
use diesel::{
    prelude::*,
    r2d2::{self, ConnectionManager},
};
use dotenvy::dotenv;

mod db;
mod errors;
mod extractors;
mod handlers;
mod middlewares;
mod models;
mod routes;
mod schema;
mod types;

#[allow(dead_code)]
struct AppState {
    app_name: String,
    api_version: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let config = types::Config::default();
    let auth0_config = extractors::Auth0Config::default();
    // env::set_var("RUST_LOG", "info");
    // env::set_var("RUST_BACKTRACE", "1");
    // env_logger::init();

    let manager = ConnectionManager::<PgConnection>::new(config.database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    let app = HttpServer::new(move || {
        App::new()
            .app_data(auth0_config.clone())
            .app_data(web::Data::new(pool.clone()))
            .wrap(middlewares::cors(&config.client_origin_url))
            .wrap(middlewares::security_headers())
            .wrap(Logger::default())
            .service(routes::routes())
            .service(web::resource("/").to(handlers::index))
            .service(Files::new("/", "./static"))
    })
    .bind(("127.0.0.1", config.port))?
    .run();

    println!("Server running at http://127.0.0.1:{}/", config.port);
    app.await
}
