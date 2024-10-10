use crate::models::artist::{Artist, ArtistRequest, ArtistResponse};
use crate::models::gig::{Gig, Gigs};

use actix_web::web;
use chrono::Utc;
use diesel::prelude::*;
use uuid::Uuid;

type DbError = Box<dyn std::error::Error + Send + Sync>;

pub fn insert_new_artist(
    connection: &mut PgConnection,
    data: web::Json<ArtistRequest>,
) -> Result<Artist, DbError> {
    use super::schema::artists::dsl::*;

    let new_artist = Artist {
        id: Uuid::new_v4(),
        name: data.name.clone(),
        description: data.description.clone(),
        fee: data.fee,
        currency: data.currency.clone(),
        image: data.image.clone(),
        genre: data.genre.clone(),
        location: data.location.clone(),
        created_at: Utc::now().naive_utc(),
        updated_at: Utc::now().naive_utc(),
    };

    diesel::insert_into(artists)
        .values(&new_artist)
        .execute(connection)?;

    Ok(new_artist)
}

pub fn find_artist_by_uuid(
    connection: &mut PgConnection,
    uuid: Uuid,
) -> Result<Option<ArtistResponse>, DbError> {
    use super::schema::artists::dsl::{artists, id};
    use super::schema::gigs::dsl::{artist_id, gigs};

    let artist = artists
        .filter(id.eq(uuid))
        .first::<Artist>(connection)
        .optional()?;

    if let Some(artist) = artist {
        let artist_gigs = gigs
            .filter(artist_id.eq(artist.id))
            .load::<Gig>(connection)?;

        let res = ArtistResponse {
            artist,
            gigs: Gigs(artist_gigs),
        };

        Ok(Some(res))
    } else {
        Ok(None)
    }
}

pub fn find_all_gigs(connection: &mut PgConnection) -> Result<Gigs, DbError> {
    use super::schema::gigs;

    let gigs_data: Vec<Gig> = gigs::table.get_results(connection)?;
    Ok(Gigs(gigs_data))
}
