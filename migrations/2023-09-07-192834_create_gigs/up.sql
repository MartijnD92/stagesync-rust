CREATE TABLE IF NOT EXISTS "gigs" (
    "id"    INTEGER PRIMARY KEY NOT NULL,
    "title" VARCHAR NOT NULL,
    "location"  TEXT NOT NULL,
    "date"  TIMESTAMP NOT NULL,
    "artist_id" INTEGER NOT NULL REFERENCES artists(id)
)
