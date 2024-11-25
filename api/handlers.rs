use crate::db;
use crate::errors::ServiceError;
use crate::extractors::Claims;
use crate::models::artist::ArtistRequest;
use crate::models::gig::GigRequest;
use crate::models::user::UserRequest;
use actix_web::{delete, get, post, web, HttpResponse};
use serde_json::json;
use std::collections::HashSet;
use uuid::Uuid;

#[get("/users/me")]
pub async fn get_current_user(claims: Claims) -> Result<HttpResponse, ServiceError> {
    let user = web::block(move || db::get_user_from_claims(&claims))
        .await?
        .map_err(|_| ServiceError::InternalServerError)?;

    if let Some(user) = user {
        Ok(HttpResponse::Ok().json(user))
    } else {
        Ok(HttpResponse::NotFound().body(
            json!({
                "error": 404,
                "message": format!("Could not find current user")
            })
            .to_string(),
        ))
    }
}

#[get("/users/{id}")]
pub async fn get_user_by_id(
    claims: Claims,
    id: web::Path<Uuid>,
) -> Result<HttpResponse, ServiceError> {
    claims.validate_permissions(&HashSet::from(["read:users_all".to_string()]))?;

    let user_id = id.to_owned();
    let user = web::block(move || db::get_user_by_id(user_id))
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
}

#[get("/users")]
pub async fn get_users(claims: Claims) -> Result<HttpResponse, ServiceError> {
    claims.validate_permissions(&HashSet::from(["read:users_all".to_string()]))?;

    let users = web::block(db::get_all_users)
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
    claims: Claims,
    form: web::Json<UserRequest>,
) -> Result<HttpResponse, ServiceError> {
    claims.validate_permissions(&HashSet::from(["update:users_all".to_string()]))?;

    let user = web::block(move || db::insert_new_user(form))
        .await?
        .map_err(|_| ServiceError::InternalServerError)?;

    Ok(HttpResponse::Created().json(user))
}

#[delete("/users/{id}")]
async fn delete_user(claims: Claims, id: web::Path<Uuid>) -> Result<HttpResponse, ServiceError> {
    claims.validate_permissions(&HashSet::from(["delete:user_any".to_string()]))?;
    // claims.validate_auth0_id(&id)?;

    let user_id = id.to_owned();
    let delete_count = web::block(move || db::delete_user_by_id(user_id))
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

#[get("/artists/me")]
pub async fn get_artists_for_user(claims: Claims) -> Result<HttpResponse, ServiceError> {
    claims.validate_permissions(&HashSet::from(["read:artists".to_string()]))?;

    let current_user = web::block(move || db::get_user_from_claims(&claims))
        .await?
        .map_err(|_| ServiceError::InternalServerError)?;

    if let Some(user) = current_user {
        let artists = web::block(move || db::get_artists_for_user(user.id))
            .await?
            .map_err(|_| ServiceError::InternalServerError)?;

        if !artists.is_empty() {
            return Ok(HttpResponse::Ok().json(artists));
        }
        Ok(HttpResponse::NotFound().body(
            json!({
                "error": 404,
                "message": "No artists were found.",
            })
            .to_string(),
        ))
    } else {
        Ok(HttpResponse::Unauthorized().json(json!({
            "error": 401,
            "message": "Unauthorized user."
        })))
    }
}

#[get("/artists")]
pub async fn get_artists(claims: Claims) -> Result<HttpResponse, ServiceError> {
    claims.validate_permissions(&HashSet::from(["read:artists".to_string()]))?;

    let artists = web::block(db::get_all_artists)
        .await?
        .map_err(|_| ServiceError::InternalServerError)?;

    if !artists.is_empty() {
        Ok(HttpResponse::Ok().json(artists))
    } else {
        let res = HttpResponse::NotFound().body(
            json!({
                "error": 404,
                "message": "No artists were found.",
            })
            .to_string(),
        );
        Ok(res)
    }
}

#[get("/artists/{id}")]
pub async fn get_artist_by_id(
    claims: Claims,
    id: web::Path<Uuid>,
) -> Result<HttpResponse, ServiceError> {
    claims.validate_permissions(&HashSet::from(["read:artists".to_string()]))?;

    let artist_id = id.to_owned();
    let artist = web::block(move || db::get_artist_by_id(artist_id))
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
    claims: Claims,
    form: web::Json<ArtistRequest>,
) -> Result<HttpResponse, ServiceError> {
    claims.validate_permissions(&HashSet::from(["update:artists".to_string()]))?;

    let artist = web::block(move || db::insert_new_artist(form))
        .await?
        .map_err(|_| ServiceError::InternalServerError)?;

    Ok(HttpResponse::Created().json(artist))
}

#[delete("/artists/{id}")]
async fn delete_artist(claims: Claims, id: web::Path<Uuid>) -> Result<HttpResponse, ServiceError> {
    claims.validate_permissions(&HashSet::from(["delete:artists".to_string()]))?;

    let artist_id = id.to_owned();
    let delete_count = web::block(move || db::delete_artist_by_id(artist_id))
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
pub async fn get_gigs(claims: Claims) -> Result<HttpResponse, ServiceError> {
    claims.validate_permissions(&HashSet::from(["read:gigs".to_string()]))?;
    println!("{:?}", claims);

    let gigs = web::block(db::get_all_gigs)
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
async fn get_gig_by_id(claims: Claims, id: web::Path<Uuid>) -> Result<HttpResponse, ServiceError> {
    claims.validate_permissions(&HashSet::from(["read:gigs".to_string()]))?;

    let gig_id = id.to_owned();
    let gig = web::block(move || db::get_gig_by_id(gig_id))
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
    claims: Claims,
    form: web::Json<GigRequest>,
) -> Result<HttpResponse, ServiceError> {
    claims.validate_permissions(&HashSet::from(["update:gigs".to_string()]))?;

    let gig = web::block(move || db::insert_new_gig(form))
        .await?
        .map_err(|_| ServiceError::InternalServerError)?;

    Ok(HttpResponse::Ok().json(gig))
}

#[delete("/gigs/{id}")]
async fn delete_gig(claims: Claims, id: web::Path<Uuid>) -> Result<HttpResponse, ServiceError> {
    claims.validate_permissions(&HashSet::from(["delete:gigs".to_string()]))?;

    let gig_id = id.to_owned();
    let delete_count = web::block(move || db::delete_gig_by_id(gig_id))
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
