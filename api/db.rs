use super::models::artist::{Artist, ArtistRequest};
use super::models::gig::{Gig, Gigs};

use actix_web::web;
use chrono::Utc;
use diesel::prelude::*;
use uuid::Uuid;

type DbError = Box<dyn std::error::Error + Send + Sync>;

pub fn insert_new_artist(
    conn: &mut PgConnection,
    data: web::Json<ArtistRequest>,
) -> Result<Artist, DbError> {
    use super::schema::artist::dsl::*;

    let new_artist = Artist {
        id: Uuid::new_v4(),
        name: data.name,
        description: data.description.clone(),
        fee: data.fee,
        currency: data.currency,
        image: data.image.clone(),
        genre: data.genre.clone(),
        location: data.location,
        created_at: Utc::now().naive_utc(),
        updated_at: Utc::now().naive_utc(),
    };

    diesel::insert_into(artist)
        .values(&new_artist)
        .execute(conn)?;

    Ok(new_artist)
}

pub fn find_artist_by_uuid(conn: &mut PgConnection, uuid: Uuid) -> Result<Option<Artist>, DbError> {
    use super::schema::artist::dsl::{artist, id};

    let _artist = artist
        .filter(id.eq(uuid))
        .first::<Artist>(conn)
        .optional()?;

    let gigs = Gig::belonging_to(&_artist).select(Gig::as_select());

    Ok(_artist)
}

pub fn get_all_gigs(conn: &mut PgConnection) -> Result<Gigs, DbError> {
    use super::schema::gig;

    let gigs_data: Vec<Gig> = gig::table.get_results(conn)?;
    Ok(Gigs(gigs_data))
}
