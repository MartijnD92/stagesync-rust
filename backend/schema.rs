// @generated automatically by Diesel CLI.

diesel::table! {
    artists (id) {
        id -> Integer,
        name -> Text,
    }
}

diesel::table! {
    gigs (id) {
        id -> Integer,
        title -> Text,
        location -> Text,
        date -> Timestamp,
        artist_id -> Integer,
    }
}

diesel::joinable!(gigs -> artists (artist_id));

diesel::allow_tables_to_appear_in_same_query!(
    artists,
    gigs,
);
