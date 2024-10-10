use crate::db;
use crate::models::artist::ArtistRequest;
use actix_files::NamedFile;
use actix_web::{get, post, web, Error, HttpResponse, Responder};
use diesel::{
    prelude::*,
    r2d2::{self, ConnectionManager},
};
use serde_json::json;
use uuid::Uuid;

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub async fn index() -> impl Responder {
    NamedFile::open_async("./static/index.html").await.unwrap()
}

#[post("/artists/create")]
pub async fn create_artist(
    pool: web::Data<DbPool>,
    form: web::Json<ArtistRequest>,
) -> Result<HttpResponse, Error> {
    let artist = web::block(move || {
        let mut conn = pool.get()?;
        db::insert_new_artist(&mut conn, form)
    })
    .await?
    .map_err(actix_web::error::ErrorUnprocessableEntity)?;

    Ok(HttpResponse::Ok().json(artist))
}

#[get("/artists/{artist_id}")]
pub async fn get_artist_by_id(
    pool: web::Data<DbPool>,
    id: web::Path<Uuid>,
) -> Result<HttpResponse, Error> {
    let artist_id = id.to_owned();
    let artist = web::block(move || {
        let mut conn = pool.get()?;
        db::find_artist_by_uuid(&mut conn, artist_id)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    if let Some(artist) = artist {
        Ok(HttpResponse::Ok().json(artist))
    } else {
        let res = HttpResponse::NotFound().body(
            json!({
                "error": 404,
                "message": format!("No artist found with id: {id}")
            })
            .to_string(),
        );
        Ok(res)
    }
}

#[get("/gigs")]
pub async fn get_gigs(pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    let gigs = web::block(move || {
        let mut conn = pool.get()?;
        db::find_all_gigs(&mut conn)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    if !gigs.is_empty() {
        Ok(HttpResponse::Ok().json(gigs))
    } else {
        let res = HttpResponse::NotFound().body(
            json!({
                "error": 404,
                "message": "No gigs found.",
            })
            .to_string(),
        );
        Ok(res)
    }
}
