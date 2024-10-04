// @generated automatically by Diesel CLI.

diesel::table! {
    artists (id) {
        id -> Uuid,
        name -> Varchar,
        created_at -> Varchar,
    }
}

diesel::table! {
    gigs (id) {
        id -> Uuid,
        title -> Varchar,
        location -> Text,
        date -> Timestamp,
        artist_id -> Uuid,
    }
}

diesel::joinable!(gigs -> artists (artist_id));

diesel::allow_tables_to_appear_in_same_query!(
    artists,
    gigs,
);
