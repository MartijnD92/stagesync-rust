// @generated automatically by Diesel CLI.

diesel::table! {
    artist (id) {
        id -> Uuid,
        name -> Varchar,
        description -> Nullable<Text>,
        image -> Nullable<Varchar>,
        fee -> Numeric,
        currency -> Varchar,
        genre -> Nullable<Varchar>,
        location -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    gig (id) {
        id -> Uuid,
        title -> Varchar,
        location -> Text,
        date -> Timestamp,
        artist_id -> Uuid,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::joinable!(gig -> artist (artist_id));

diesel::allow_tables_to_appear_in_same_query!(artist, gig,);
