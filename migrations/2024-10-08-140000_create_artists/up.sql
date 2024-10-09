CREATE TABLE IF NOT EXISTS artist (
    id    UUID PRIMARY KEY NOT NULL,
    name VARCHAR NOT NULL,
    description TEXT,
    image VARCHAR,
    fee DECIMAL NOT NULL,
    currency VARCHAR NOT NULL,
    genre VARCHAR,
    location TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
)