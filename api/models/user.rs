use crate::schema::users;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: uuid::Uuid,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = users)]
pub struct NewUser<'a> {
    pub id: uuid::Uuid,
    pub first_name: &'a str,
    pub last_name: &'a str,
    pub email: &'a str,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserRequest {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
}
