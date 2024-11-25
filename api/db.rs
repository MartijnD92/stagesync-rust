use crate::errors::ServiceError;
use crate::extractors::Claims;
use crate::models::artist::{Artist, ArtistRequest, ArtistResponse, NewArtist};
use crate::models::gig::{Gig, GigRequest, Gigs};
use crate::models::user::{NewUser, User, UserRequest};
use crate::types;

use actix_web::web;
use chrono::Utc;
use diesel::pg::PgConnection;
use diesel::{
    prelude::*,
    r2d2::{ConnectionManager, Pool, PoolError, PooledConnection},
};
use std::sync::LazyLock;
use uuid::Uuid;

type DbPool = Pool<ConnectionManager<PgConnection>>;
type DbConn = PooledConnection<ConnectionManager<PgConnection>>;
type DbError = Box<dyn std::error::Error + Send + Sync>;

static GLOBAL_POOL: LazyLock<DbPool> = LazyLock::new(|| {
    let config = types::Config::default();
    let manager = ConnectionManager::<PgConnection>::new(config.database_url);
    Pool::builder()
        .build(manager)
        .expect("Failed to create pool.")
});

fn get_connection() -> Result<DbConn, PoolError> {
    GLOBAL_POOL.get()
}

pub fn insert_new_user(data: web::Json<UserRequest>) -> Result<User, DbError> {
    use super::schema::users::dsl::*;
    let mut conn = get_connection()?;

    let new_user = NewUser {
        id: Uuid::new_v4(),
        first_name: &data.first_name,
        last_name: &data.last_name,
        email: &data.email,
        created_at: Utc::now().naive_utc(),
        updated_at: Utc::now().naive_utc(),
    };

    let user = diesel::insert_into(users)
        .values(&new_user)
        .get_result(&mut conn)?;

    Ok(user)
}

pub fn get_all_users() -> Result<Vec<User>, DbError> {
    use super::schema::users::dsl::users;
    let mut conn = get_connection()?;

    let data = users.load::<User>(&mut conn)?;
    Ok(data)
}

pub fn get_user_by_id(uuid: Uuid) -> Result<Option<User>, DbError> {
    use super::schema::users::dsl::{id, users};
    let mut conn = get_connection()?;

    let user = users
        .filter(id.eq(uuid))
        .first::<User>(&mut conn)
        .optional()?;

    Ok(user)
}

pub fn get_user_from_claims(claims: &Claims) -> Result<Option<User>, DbError> {
    use super::schema::auth0_users;
    use super::schema::users;
    let mut conn = get_connection()?;
    let sub = claims
        .get_auth0_sub()
        .expect("Could not parse sub from claims");

    let user: Option<User> = users::table
        .inner_join(auth0_users::table.on(auth0_users::user_id.eq(users::id)))
        .filter(auth0_users::auth0_sub.eq(sub))
        .select(users::all_columns)
        .first(&mut conn)
        .optional()?;

    Ok(user)
}

pub fn delete_user_by_id(uuid: Uuid) -> Result<usize, DbError> {
    use super::schema::users::dsl::{id, users};
    let mut conn = get_connection()?;

    let count = diesel::delete(users.filter(id.eq(uuid))).execute(&mut conn)?;

    Ok(count)
}

pub fn insert_new_artist(data: web::Json<ArtistRequest>) -> Result<Artist, DbError> {
    use super::schema::artists::dsl::*;
    let mut conn = get_connection()?;

    let new_artist = NewArtist {
        id: Uuid::new_v4(),
        name: &data.name,
        description: data.description.as_deref(),
        fee: data.fee,
        currency: &data.currency,
        image: data.image.as_deref(),
        genre: data.genre.as_deref(),
        location: data.location.as_deref(),
        created_at: Utc::now().naive_utc(),
        updated_at: Utc::now().naive_utc(),
    };

    let artist = diesel::insert_into(artists)
        .values(&new_artist)
        .get_result(&mut conn)?;

    Ok(artist)
}

pub fn get_all_artists() -> Result<Vec<ArtistResponse>, DbError> {
    use super::schema::artists;
    use super::schema::gigs;
    use super::schema::users;
    let mut conn = get_connection()?;

    let mut res = Vec::new();
    let data: Vec<Artist> = artists::table.load(&mut conn)?;
    for artist in Some(data).iter().flatten() {
        let artist_gigs: Vec<Gig> = gigs::table
            .filter(gigs::artist_id.eq(artist.id))
            .order(gigs::date.desc())
            .load(&mut conn)?;

        let owner: User = users::table
            .filter(users::id.eq(artist.user_id))
            .first(&mut conn)?;

        res.push(ArtistResponse {
            artist: artist.clone(),
            owner,
            gigs: Gigs(artist_gigs),
        });
    }
    Ok(res)
}

pub fn get_artists_for_user(user_id: Uuid) -> Result<Vec<ArtistResponse>, DbError> {
    use super::schema::artists;
    use super::schema::gigs;
    use super::schema::users;
    let mut conn = get_connection()?;

    let mut res = Vec::new();
    let data: Vec<Artist> = artists::table
        .filter(artists::user_id.eq(user_id))
        .load(&mut conn)?;
    for artist in Some(data).iter().flatten() {
        let artist_gigs: Vec<Gig> = gigs::table
            .filter(gigs::artist_id.eq(artist.id))
            .order(gigs::date.desc())
            .load(&mut conn)?;

        let owner: User = users::table
            .filter(users::id.eq(artist.user_id))
            .first(&mut conn)?;

        res.push(ArtistResponse {
            artist: artist.clone(),
            owner,
            gigs: Gigs(artist_gigs),
        });
    }
    Ok(res)
}

pub fn get_artist_by_id(uuid: Uuid) -> Result<Option<ArtistResponse>, DbError> {
    use super::schema::artists;
    use super::schema::gigs;
    use super::schema::users;
    let mut conn = get_connection()?;

    let artist = artists::table
        .filter(artists::id.eq(uuid))
        .first::<Artist>(&mut conn)
        .optional()?;

    if let Some(artist) = artist {
        let artist_gigs = gigs::table
            .filter(gigs::artist_id.eq(artist.id))
            .load::<Gig>(&mut conn)?;

        let owner = users::table
            .filter(users::id.eq(artist.user_id))
            .first::<User>(&mut conn)?;

        let res = ArtistResponse {
            artist,
            owner,
            gigs: Gigs(artist_gigs),
        };

        Ok(Some(res))
    } else {
        Ok(None)
    }
}

pub fn delete_artist_by_id(uuid: Uuid) -> Result<usize, DbError> {
    use super::schema::artists::dsl::{artists, id};
    let mut conn = get_connection()?;

    let count = diesel::delete(artists.filter(id.eq(uuid))).execute(&mut conn)?;

    Ok(count)
}

pub fn get_all_gigs() -> Result<Gigs, DbError> {
    use super::schema::gigs::dsl::gigs;
    let mut conn = get_connection()?;

    let data: Vec<Gig> = gigs.load::<Gig>(&mut conn)?;
    Ok(Gigs(data))
}

pub fn get_gig_by_id(uuid: Uuid) -> Result<Option<Gig>, DbError> {
    use super::schema::gigs::dsl::{gigs, id};
    let mut conn = get_connection()?;

    let gig = gigs
        .filter(id.eq(uuid))
        .first::<Gig>(&mut conn)
        .optional()?;

    Ok(gig)
}

pub fn insert_new_gig(data: web::Json<GigRequest>) -> Result<Gig, DbError> {
    use super::schema::gigs::dsl::*;
    let mut conn = get_connection()?;

    let new_gig = Gig {
        id: Uuid::new_v4(),
        title: data.title.clone(),
        location: data.location.clone(),
        date: data.date,
        artist_id: data.artist_id,
        created_at: Utc::now().naive_utc(),
        updated_at: Utc::now().naive_utc(),
    };

    diesel::insert_into(gigs)
        .values(&new_gig)
        .execute(&mut conn)?;

    Ok(new_gig)
}

pub fn delete_gig_by_id(uuid: Uuid) -> Result<usize, DbError> {
    use super::schema::gigs::dsl::{gigs, id};
    let mut conn = get_connection()?;

    let count = diesel::delete(gigs.filter(id.eq(uuid))).execute(&mut conn)?;

    Ok(count)
}
