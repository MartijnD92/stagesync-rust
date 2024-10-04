use crate::db;
use crate::models;
use actix_web::{get, post, web, Error, HttpResponse};
use anyhow::Result;
use diesel::{
    prelude::*,
    r2d2::{self, ConnectionManager},
};
use serde_json::json;

type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

#[post("/artists/create")]
pub async fn create_artist(
    pool: web::Data<DbPool>,
    form: web::Json<models::Artist>,
) -> Result<HttpResponse, Error> {
    let user = web::block(move || {
        let mut conn = pool.get()?;
        db::insert_new_artist(&mut conn, &form.name)
    })
    .await?
    .map_err(actix_web::error::ErrorUnprocessableEntity)?;

    Ok(HttpResponse::Ok().json(user))
}

// #[get("/users/{user_id}")]
// pub async fn get_user_by_id(
//     pool: web::Data<DbPool>,
//     id: web::Path<Uuid>,
// ) -> Result<HttpResponse, Error> {
//     let user_id = id.to_owned();
//     let user = web::block(move || {
//         let mut conn = pool.get()?;
//         db::find_user_by_uid(&mut conn, user_id)
//     })
//     .await?
//     .map_err(actix_web::error::ErrorInternalServerError)?;

//     if let Some(user) = user {
//         Ok(HttpResponse::Ok().json(user))
//     } else {
//         let res = HttpResponse::NotFound().body(
//             json!({
//                 "error": 404,
//                 "message": format!("No user found with phone: {id}")
//             })
//             .to_string(),
//         );
//         Ok(res)
//     }
// }
