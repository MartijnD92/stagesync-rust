use crate::models::gig::Gigs;
use crate::schema::artists;
use serde::{Deserialize, Serialize};

use diesel::prelude::*;

#[derive(Insertable, Queryable, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Artist {
    pub id: uuid::Uuid,
    pub name: String,
    pub description: Option<String>,
    pub image: Option<String>,
    pub fee: i32,
    pub currency: String,
    pub genre: Option<String>,
    pub location: Option<String>,
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
