use crate::db;
use crate::errors::ServiceError;
use crate::extractors::Claims;
use crate::models::artist::ArtistRequest;
use crate::models::gig::GigRequest;
use crate::models::user::UserRequest;
use actix_files::NamedFile;
use actix_web::{delete, get, post, web, HttpResponse, Responder};
use diesel::{
    prelude::*,
    r2d2::{self, ConnectionManager},
};
use serde_json::json;
use std::collections::HashSet;
use uuid::Uuid;

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub async fn index() -> impl Responder {
    NamedFile::open_async("./static/index.html").await.unwrap()
}

#[get("/users/{id}")]
pub async fn get_user_by_id(
    claims: Claims,
    pool: web::Data<DbPool>,
    id: web::Path<Uuid>,
) -> Result<HttpResponse, ServiceError> {
    if claims.validate_permissions(&HashSet::from(["read:users".to_string()])) {
        let user_id = id.to_owned();
        let user = web::block(move || {
            let mut conn = pool.get()?;
            db::get_user_by_id(&mut conn, user_id)
        })
        .await?
        .map_err(|_| ServiceError::InternalServerError)?;

        if let Some(user) = user {
            Ok(HttpResponse::Ok().json(user))
        } else {
            let res = HttpResponse::NotFound().body(
                json!({
                    "error": 404,
                    "message": format!("No user found with id '{id}'")
                })
                .to_string(),
            );
            Ok(res)
        }
    } else {
        Err(ServiceError::Unauthorized)
    }
}

#[get("/users")]
pub async fn get_users(pool: web::Data<DbPool>) -> Result<HttpResponse, ServiceError> {
    let users = web::block(move || {
        let mut conn = pool.get()?;
        db::get_all_users(&mut conn)
    })
    .await?
    .map_err(|_| ServiceError::InternalServerError)?;

    if !users.is_empty() {
        Ok(HttpResponse::Ok().json(users))
    } else {
        let res = HttpResponse::NotFound().body(
            json!({
                "error": 404,
                "message": "No users were found.",
            })
            .to_string(),
        );
        Ok(res)
    }
}

#[post("/users")]
pub async fn add_user(
    pool: web::Data<DbPool>,
    form: web::Json<UserRequest>,
) -> Result<HttpResponse, ServiceError> {
    let user = web::block(move || {
        let mut conn = pool.get()?;
        db::insert_new_user(&mut conn, form)
    })
    .await?
    .map_err(|_| ServiceError::InternalServerError)?;

    Ok(HttpResponse::Created().json(user))
}

#[delete("/users/{id}")]
async fn delete_user(
    pool: web::Data<DbPool>,
    id: web::Path<Uuid>,
) -> Result<HttpResponse, ServiceError> {
    let user_id = id.to_owned();
    let delete_count = web::block(move || {
        let mut conn = pool.get()?;
        db::delete_user_by_id(&mut conn, user_id)
    })
    .await?
    .map_err(|_| ServiceError::InternalServerError)?;

    if delete_count > 0 {
        Ok(HttpResponse::NoContent().finish())
    } else {
        let res = HttpResponse::NotFound().body(
            json!({
                "error": 404,
                "message": format!("Failed to delete user with id '{id}'. No user found.")
            })
            .to_string(),
        );
        Ok(res)
    }
}

#[get("/artists")]
pub async fn get_artists(pool: web::Data<DbPool>) -> Result<HttpResponse, ServiceError> {
    let artists = web::block(move || {
        let mut conn = pool.get()?;
        db::get_all_artists(&mut conn)
    })
    .await?
    .map_err(|_| ServiceError::InternalServerError)?;

    if !artists.is_empty() {
        Ok(HttpResponse::Ok().json(artists))
    } else {
        let res = HttpResponse::NotFound().body(
            json!({
                "error": 404,
                "message": "No users were found.",
            })
            .to_string(),
        );
        Ok(res)
    }
}

#[get("/artists/{id}")]
pub async fn get_artist_by_id(
    pool: web::Data<DbPool>,
    id: web::Path<Uuid>,
) -> Result<HttpResponse, ServiceError> {
    let artist_id = id.to_owned();
    let artist = web::block(move || {
        let mut conn = pool.get()?;
        db::get_artist_by_id(&mut conn, artist_id)
    })
    .await?
    .map_err(|_| ServiceError::InternalServerError)?;

    if let Some(artist) = artist {
        Ok(HttpResponse::Ok().json(artist))
    } else {
        let res = HttpResponse::NotFound().body(
            json!({
                "error": 404,
                "message": format!("No artist found with id '{id}'")
            })
            .to_string(),
        );
        Ok(res)
    }
}

#[post("/artists")]
pub async fn add_artist(
    pool: web::Data<DbPool>,
    form: web::Json<ArtistRequest>,
) -> Result<HttpResponse, ServiceError> {
    let artist = web::block(move || {
        let mut conn = pool.get()?;
        db::insert_new_artist(&mut conn, form)
    })
    .await?
    .map_err(|_| ServiceError::InternalServerError)?;

    Ok(HttpResponse::Created().json(artist))
}

#[delete("/artists/{id}")]
async fn delete_artist(
    pool: web::Data<DbPool>,
    id: web::Path<Uuid>,
) -> Result<HttpResponse, ServiceError> {
    let artist_id = id.to_owned();
    let delete_count = web::block(move || {
        let mut conn = pool.get()?;
        db::delete_artist_by_id(&mut conn, artist_id)
    })
    .await?
    .map_err(|_| ServiceError::InternalServerError)?;

    if delete_count > 0 {
        Ok(HttpResponse::NoContent().finish())
    } else {
        let res = HttpResponse::NotFound().body(
            json!({
                "error": 404,
                "message": format!("Failed to delete artist with id '{id}'. No artist found.")
            })
            .to_string(),
        );
        Ok(res)
    }
}

#[get("/gigs")]
pub async fn get_gigs(pool: web::Data<DbPool>) -> Result<HttpResponse, ServiceError> {
    let gigs = web::block(move || {
        let mut conn = pool.get()?;
        db::get_all_gigs(&mut conn)
    })
    .await?
    .map_err(|_| ServiceError::InternalServerError)?;

    if !gigs.is_empty() {
        Ok(HttpResponse::Ok().json(gigs))
    } else {
        let res = HttpResponse::NotFound().body(
            json!({
                "error": 404,
                "message": "No gigs were found.",
            })
            .to_string(),
        );
        Ok(res)
    }
}

#[get("/gigs/{id}")]
async fn get_gig_by_id(
    pool: web::Data<DbPool>,
    id: web::Path<Uuid>,
) -> Result<HttpResponse, ServiceError> {
    let gig_id = id.to_owned();
    let gig = web::block(move || {
        let mut conn = pool.get()?;
        db::get_gig_by_id(&mut conn, gig_id)
    })
    .await?
    .map_err(|_| ServiceError::InternalServerError)?;

    if let Some(gig) = gig {
        Ok(HttpResponse::Ok().json(gig))
    } else {
        let res = HttpResponse::NotFound().body(
            json!({
                "error": 404,
                "message": format!("No gig found with id '{id}'")
            })
            .to_string(),
        );
        Ok(res)
    }
}

#[post("/gigs")]
pub async fn create_gig(
    pool: web::Data<DbPool>,
    form: web::Json<GigRequest>,
) -> Result<HttpResponse, ServiceError> {
    let gig = web::block(move || {
        let mut conn = pool.get()?;
        db::insert_new_gig(&mut conn, form)
    })
    .await?
    .map_err(|_| ServiceError::InternalServerError)?;

    Ok(HttpResponse::Ok().json(gig))
}

#[delete("/gigs/{id}")]
async fn delete_gig(
    pool: web::Data<DbPool>,
    id: web::Path<Uuid>,
) -> Result<HttpResponse, ServiceError> {
    let gig_id = id.to_owned();
    let delete_count = web::block(move || {
        let mut conn = pool.get()?;
        db::delete_gig_by_id(&mut conn, gig_id)
    })
    .await?
    .map_err(|_| ServiceError::InternalServerError)?;

    if delete_count > 0 {
        Ok(HttpResponse::NoContent().finish())
    } else {
        let res = HttpResponse::NotFound().body(
            json!({
                "error": 404,
                "message": format!("Failed to delete gig with id '{id}'. No gig found.")
            })
            .to_string(),
        );
        Ok(res)
    }
}
