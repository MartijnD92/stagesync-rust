use crate::models::gig::Gigs;
use crate::schema::artists;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Insertable, Queryable, Debug, Clone, Serialize, Deserialize)]
pub struct Artist {
    pub id: uuid::Uuid,
    pub name: String,
    pub description: Option<String>,
    pub image: Option<String>,
    pub fee: i32,
    pub currency: String,
    pub genre: Option<String>,
    pub location: Option<String>,
    pub user_id: uuid::Uuid,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = artists)]
pub struct NewArtist<'a> {
    pub id: uuid::Uuid,
    pub name: &'a str,
    pub description: Option<&'a str>,
    pub image: Option<&'a str>,
    pub fee: i32,
    pub currency: &'a str,
    pub genre: Option<&'a str>,
    pub location: Option<&'a str>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ArtistRequest {
    pub name: String,
    pub description: Option<String>,
    pub image: Option<String>,
    pub fee: i32,
    pub currency: String,
    pub genre: Option<String>,
    pub location: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct ArtistResponse {
    pub artist: Artist,
    pub gigs: Gigs,
}
