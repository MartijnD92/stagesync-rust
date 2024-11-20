// @generated automatically by Diesel CLI.

diesel::table! {
    artists (id) {
        id -> Uuid,
        name -> Varchar,
        description -> Nullable<Text>,
        image -> Nullable<Varchar>,
        fee -> Int4,
        currency -> Varchar,
        genre -> Nullable<Varchar>,
        location -> Nullable<Text>,
        user_id -> Uuid,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    auth0_users (user_id) {
        user_id -> Uuid,
        auth0_sub -> Varchar,
    }
}

diesel::table! {
    gigs (id) {
        id -> Uuid,
        title -> Varchar,
        location -> Text,
        date -> Timestamp,
        artist_id -> Uuid,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        first_name -> Text,
        last_name -> Text,
        email -> Text,
        image -> Nullable<Varchar>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::joinable!(auth0_users -> users (user_id));
diesel::joinable!(gigs -> artists (artist_id));

diesel::allow_tables_to_appear_in_same_query!(
    artists,
    auth0_users,
    gigs,
    users,
);
