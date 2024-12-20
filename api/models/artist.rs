use crate::models::gig::Gigs;
use crate::models::user::User;
use crate::schema::artists;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Insertable, Queryable, Debug, Clone, Serialize, Deserialize)]
pub struct Artist {
    pub id: uuid::Uuid,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    pub fee: i32,
    pub currency: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub genre: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    pub fee: i32,
    pub currency: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub genre: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct ArtistResponse {
    #[serde(flatten)]
    pub artist: Artist,
    pub owner: User,
    pub gigs: Gigs,
}
