extern crate dashboard;
extern crate diesel;

use diesel::prelude::*;
use std::error::Error;

use self::dashboard::*;
use crate::models::{Gig, Gigs};

fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    use self::schema::gigs::dsl::*;

    let conn = &mut establish_connection();
    let artist = "The Hillbilly Moonshiners";
    // let gig = "Zwarte Cross";

    // let _create_artist = new_artist(conn, artist)?;
    // let _create_gig = new_artist(conn, artist)?;

    let artist_gigs = gigs
        .filter(artist_id.eq(1))
        .select(Gig::as_select())
        .load(conn)
        .expect("Error loading gigs");

    // let artist = create_post(&connection, name, gigs);
    println!("Gigs for '{}':\n{}", artist , Gigs(artist_gigs));

    Ok(())
}
