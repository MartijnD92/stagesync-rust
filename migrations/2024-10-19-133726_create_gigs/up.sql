CREATE TABLE IF NOT EXISTS gigs (
    id    UUID PRIMARY KEY NOT NULL,
    title VARCHAR NOT NULL,
    location  TEXT NOT NULL,
    date  TIMESTAMP NOT NULL DEFAULT NOW(),
    artist_id UUID NOT NULL REFERENCES artists(id),
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
)
