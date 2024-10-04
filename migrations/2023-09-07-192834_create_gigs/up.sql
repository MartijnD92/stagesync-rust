CREATE TABLE IF NOT EXISTS gigs (
    id    UUID PRIMARY KEY NOT NULL,
    title VARCHAR NOT NULL,
    location  TEXT NOT NULL,
    date  TIMESTAMP NOT NULL,
    artist_id UUID NOT NULL REFERENCES artists(id)
)
