CREATE TABLE auth0_users (
    user_id    UUID PRIMARY KEY NOT NULL REFERENCES users(id),
    auth0_sub  VARCHAR NOT NULL
)
