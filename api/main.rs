use actix_cors::Cors;
use actix_files::Files;
use actix_web::{http, middleware::Logger, web, App, HttpServer};
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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("'DATABASE_URL' must be set");
    // env::set_var("RUST_LOG", "info");
    // env::set_var("RUST_BACKTRACE", "1");
    // env_logger::init();

    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    let server_addr = "127.0.0.1";
    let server_port = 8080;

    let app = HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:3000")
            .allowed_origin("http://localhost:8080")
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600);

        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(cors)
            .wrap(Logger::default())
            .service(web::resource("/").to(routes::index))
            .service(routes::create_artist)
            .service(routes::get_artist_by_id)
            .service(routes::get_gigs)
            .service(Files::new("/", "./static"))
    })
    .workers(2) // TODO: How many do I need and do I actually have to specify this?
    .bind((server_addr, server_port))?
    .run();

    println!("Server running at http://{server_addr}:{server_port}/");
    app.await
}
