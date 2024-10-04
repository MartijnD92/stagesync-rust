use anyhow::{Context, Result};
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

pub mod db;
pub mod models;
pub mod routes;
pub mod schema;
use models::{Gig, Gigs};

fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("'DATABASE_URL' must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

fn main() -> Result<()> {
    // use schema::gigs::dsl::*;

    // let conn = &mut establish_connection();
    // let artist = "The Hillbilly Moonshiners";
    // let gig = "Zwarte Cross";

    // let _create_artist = new_artist(conn, artist)?;
    // let _create_gig = new_gig(conn, artist)?;

    Ok(())
}
