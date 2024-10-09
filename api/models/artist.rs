use crate::models::gig::Gigs;
use crate::schema::artist;
use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};

use diesel::prelude::*;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Insertable, Queryable)]
#[diesel(table_name = artist)]
pub struct Artist {
    pub id: uuid::Uuid,
    pub name: String,
    pub description: Option<String>,
    pub image: Option<String>,
    pub fee: BigDecimal,
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
    pub fee: f64,
    pub currency: String,
    pub genre: Option<String>,
    pub location: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct ArtistResponse {
    pub artist: Artist,
    pub gigs: Gigs,
    pub total: i64,
    pub offset: i64,
    pub limit: i64,
}
