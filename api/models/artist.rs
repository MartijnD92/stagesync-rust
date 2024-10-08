use crate::schema::artist;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Queryable, AsChangeset)]
#[diesel(table_name = artist)]
pub struct Artist {
    pub id: uuid::Uuid,
    pub name: String,
    pub description: Option<String>,
    pub image: String,
    pub fee: BigDecimal,
    pub currency: String,
    pub genre: String,
    pub location: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Debug, Clone, AsChangeset)]
#[diesel(table_name = artist)]
pub struct UpdateArtist {
    pub name: String,
    pub description: Option<String>,
    pub image: String,
    pub fee: BigDecimal,
    pub currency: String,
    pub genre: Option<String>,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = artist)]
pub struct NewArtist {
    pub name: String,
    pub description: Option<String>,
    pub image: String,
    pub fee: BigDecimal,
    pub currency: String,
    pub genre: Option<String>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Serialize, Deserialize)]
pub struct ArtistResponse {
    pub artist: Artist,
    pub gigs: Gigs,
    pub total: i64,
    pub offset: i64,
    pub limit: i64,
}