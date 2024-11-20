CREATE TABLE artists (
    id UUID PRIMARY KEY NOT NULL,
    name VARCHAR NOT NULL,
    description TEXT,
    image VARCHAR,
    fee INTEGER NOT NULL,
    currency VARCHAR NOT NULL,
    genre VARCHAR,
    location TEXT,
    user_id UUID NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
)