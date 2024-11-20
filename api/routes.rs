use super::handlers;

use actix_web::{web, Scope};

pub fn routes() -> Scope {
    web::scope("/api/v1")
        // Users
        .service(handlers::get_current_user)
        .service(handlers::get_user_by_id)
        .service(handlers::get_users)
        .service(handlers::add_user)
        .service(handlers::delete_user)
        // Artists
        .service(handlers::get_artists)
        .service(handlers::get_artist_by_id)
        .service(handlers::add_artist)
        .service(handlers::delete_artist)
        // Gigs
        .service(handlers::get_gigs)
        .service(handlers::get_gig_by_id)
        .service(handlers::create_gig)
        .service(handlers::delete_gig)
}
