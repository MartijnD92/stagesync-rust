use actix_cors::Cors;
use actix_files::Files;
use actix_web::{http, middleware::Logger, web, App, HttpServer};
use actix_web_httpauth::middleware::HttpAuthentication;
use diesel::{
    prelude::*,
    r2d2::{self, ConnectionManager},
};
use dotenvy::dotenv;
use std::env;

mod db;
mod errors;
mod models;
mod routes;
mod schema;
mod services;

#[allow(dead_code)]
struct AppState {
    app_name: String,
    api_version: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("'DATABASE_URL' must be set");
    let client_url = env::var("CLIENT_ORIGIN_URL").expect("'CLIENT_ORIGIN_URL' must be set");
    let server_port = env::var("PORT").expect("'PORT' must be set");
    let server_addr = "127.0.0.1";
    let server_url = format!("{}:{}", server_addr, server_port);
    // env::set_var("RUST_LOG", "info");
    // env::set_var("RUST_BACKTRACE", "1");
    // env_logger::init();

    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    let app = HttpServer::new(move || {
        let auth = HttpAuthentication::bearer(services::auth::validator);
        let cors = Cors::default()
            .allowed_origin(&client_url)
            .allowed_origin(&server_url)
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600);

        App::new()
            .wrap(auth)
            .app_data(web::Data::new(pool.clone()))
            .wrap(cors)
            .wrap(Logger::default())
            .service(
                web::scope("/api/v1")
                    // Users
                    .service(routes::get_users)
                    .service(routes::get_user_by_id)
                    .service(routes::add_user)
                    .service(routes::delete_user)
                    // Artists
                    .service(routes::get_artists)
                    .service(routes::get_artist_by_id)
                    .service(routes::add_artist)
                    .service(routes::delete_artist)
                    // Gigs
                    .service(routes::get_gigs)
                    .service(routes::get_gig_by_id)
                    .service(routes::create_gig)
                    .service(routes::delete_gig),
            )
            .service(web::resource("/").to(routes::index))
            .service(Files::new("/", "./static"))
    })
    .bind((
        server_addr,
        server_port
            .parse()
            .expect("Failed to parse port number from env"),
    ))?
    .run();

    println!("Server running at http://{server_addr}:{server_port}/");
    app.await
}
