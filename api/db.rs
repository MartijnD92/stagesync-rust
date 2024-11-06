use crate::models::artist::{Artist, ArtistRequest, ArtistResponse, NewArtist};
use crate::models::gig::{Gig, GigRequest, Gigs};
use crate::models::user::{NewUser, User, UserRequest};

use actix_web::web;
use chrono::Utc;
use diesel::prelude::*;
use uuid::Uuid;

type DbError = Box<dyn std::error::Error + Send + Sync>;

pub fn insert_new_user(
    connection: &mut PgConnection,
    data: web::Json<UserRequest>,
) -> Result<User, DbError> {
    use super::schema::users::dsl::*;

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
        .get_result(connection)?;

    Ok(user)
}

pub fn get_all_users(connection: &mut PgConnection) -> Result<Vec<User>, DbError> {
    use super::schema::users::dsl::users;

    let data = users.load::<User>(connection)?;
    Ok(data)
}

pub fn get_user_by_id(connection: &mut PgConnection, uuid: Uuid) -> Result<Option<User>, DbError> {
    use super::schema::users::dsl::{id, users};

    let user = users
        .filter(id.eq(uuid))
        .first::<User>(connection)
        .optional()?;

    Ok(user)
}

pub fn delete_user_by_id(connection: &mut PgConnection, uuid: Uuid) -> Result<usize, DbError> {
    use super::schema::users::dsl::{id, users};

    let count = diesel::delete(users.filter(id.eq(uuid))).execute(connection)?;

    Ok(count)
}

pub fn insert_new_artist(
    connection: &mut PgConnection,
    data: web::Json<ArtistRequest>,
) -> Result<Artist, DbError> {
    use super::schema::artists::dsl::*;

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
        .get_result(connection)?;

    Ok(artist)
}

pub fn get_all_artists(connection: &mut PgConnection) -> Result<Vec<ArtistResponse>, DbError> {
    use super::schema::artists::dsl::artists;
    use super::schema::gigs::dsl::{artist_id, gigs};
    use super::schema::users::dsl::{id as user_id, users};

    let mut res = Vec::new();
    let data = artists.load::<Artist>(connection)?;
    for a in Some(data).iter().flatten() {
        let artist_gigs = gigs.filter(artist_id.eq(a.id)).load::<Gig>(connection)?;
        let owner = users
            .filter(user_id.eq(a.user_id))
            .first::<User>(connection)?;

        res.push(ArtistResponse {
            artist: a.clone(),
            owner,
            gigs: Gigs(artist_gigs),
        });
    }
    Ok(res)
}

pub fn get_artist_by_id(
    connection: &mut PgConnection,
    uuid: Uuid,
) -> Result<Option<ArtistResponse>, DbError> {
    use super::schema::artists::dsl::{artists, id};
    use super::schema::gigs::dsl::{artist_id, gigs};
    use super::schema::users::dsl::{id as user_id, users};

    let artist = artists
        .filter(id.eq(uuid))
        .first::<Artist>(connection)
        .optional()?;

    if let Some(artist) = artist {
        let artist_gigs = gigs
            .filter(artist_id.eq(artist.id))
            .load::<Gig>(connection)?;

        let owner = users
            .filter(user_id.eq(artist.user_id))
            .first::<User>(connection)?;

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

pub fn delete_artist_by_id(connection: &mut PgConnection, uuid: Uuid) -> Result<usize, DbError> {
    use super::schema::artists::dsl::{artists, id};

    let count = diesel::delete(artists.filter(id.eq(uuid))).execute(connection)?;

    Ok(count)
}

pub fn get_all_gigs(connection: &mut PgConnection) -> Result<Gigs, DbError> {
    use super::schema::gigs::dsl::gigs;

    let data: Vec<Gig> = gigs.load::<Gig>(connection)?;
    Ok(Gigs(data))
}

pub fn get_gig_by_id(connection: &mut PgConnection, uuid: Uuid) -> Result<Option<Gig>, DbError> {
    use super::schema::gigs::dsl::{gigs, id};

    let gig = gigs
        .filter(id.eq(uuid))
        .first::<Gig>(connection)
        .optional()?;

    Ok(gig)
}

pub fn insert_new_gig(
    connection: &mut PgConnection,
    data: web::Json<GigRequest>,
) -> Result<Gig, DbError> {
    use super::schema::gigs::dsl::*;

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
        .execute(connection)?;

    Ok(new_gig)
}

pub fn delete_gig_by_id(connection: &mut PgConnection, uuid: Uuid) -> Result<usize, DbError> {
    use super::schema::gigs::dsl::{gigs, id};

    let count = diesel::delete(gigs.filter(id.eq(uuid))).execute(connection)?;

    Ok(count)
}
