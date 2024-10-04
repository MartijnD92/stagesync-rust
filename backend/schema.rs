// @generated automatically by Diesel CLI.

diesel::table! {
    artists (id) {
        id -> Text,
        name -> Text,
        created_at -> Text,
    }
}

diesel::table! {
    gigs (id) {
        id -> Text,
        title -> Text,
        location -> Text,
        date -> Timestamp,
        artist_id -> Text,
    }
}

diesel::joinable!(gigs -> artists (artist_id));

diesel::allow_tables_to_appear_in_same_query!(
    artists,
    gigs,
);
