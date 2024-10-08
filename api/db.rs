use crate::models::*;
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use std::time::SystemTime;
use uuid::Uuid;

type DbError = Box<dyn std::error::Error + Send + Sync>;

pub fn insert_new_artist(conn: &mut PgConnection, nm: &str) -> Result<Artist, DbError> {
    use crate::schema::artists::dsl::*;

    let new_artist = Artist {
        id: Uuid::new_v4(),
        name: nm.to_owned(),
        created_at: iso_date(),
    };

    diesel::insert_into(artists)
        .values(&new_artist)
        .execute(conn)?;

    Ok(new_artist)
}

pub fn find_artist_by_uuid(conn: &mut PgConnection, uuid: Uuid) -> Result<Option<Artist>, DbError> {
    use crate::schema::artists::dsl::*;

    let artist = artists
        .filter(id.eq(uuid))
        .first::<Artist>(conn)
        .optional()?;

    Ok(artist)
}

fn iso_date() -> String {
    let now = SystemTime::now();
    let now: DateTime<Utc> = now.into();
    now.to_rfc3339()
}

pub fn get_all_gigs(conn: &mut PgConnection) -> Result<Gigs, DbError> {
    use crate::schema::gigs;

    let gigs_data: Vec<Gig> = gigs::table.get_results(conn)?;
    Ok(Gigs(gigs_data))
}
