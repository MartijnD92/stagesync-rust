use actix_web::{middleware::Logger, App, HttpServer};
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

    let app = HttpServer::new(move || {
        App::new()
            .app_data(auth0_config.clone())
            .wrap(middlewares::cors(&config.client_origin_url))
            .wrap(middlewares::security_headers())
            .wrap(Logger::default())
            .service(routes::routes())
    })
    .bind(("127.0.0.1", config.port))?
    .run();

    println!("Server running at http://127.0.0.1:{}/", config.port);
    app.await
}
