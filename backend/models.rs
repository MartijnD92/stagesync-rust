use chrono::{DateTime, Local};
use diesel::prelude::*;
use std::fmt;
use std::ops::Deref;

use crate::schema::{artists, gigs};

#[derive(Queryable, Identifiable, Selectable, Debug, PartialEq)]
#[diesel(table_name = artists)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Artist {
    pub id: i32,
    pub name: String,
}

#[derive(Queryable, Identifiable, Selectable, Associations, Debug, PartialEq)]
#[diesel(belongs_to(Artist))]
#[diesel(table_name = gigs)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Gig {
    pub id: i32,
    pub title: String,
    pub location: String,
    pub date: chrono::NaiveDateTime,
    pub artist_id: i32,
}

impl fmt::Display for Gig {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let formatted_date =
            DateTime::<Local>::from_naive_utc_and_offset(self.date, *Local::now().offset())
                .format("%A%v");

        write!(f, "{}, {}, {}", self.title, self.location, formatted_date)
    }
}

// Necessary to display multiple gigs
// See https://github.com/apolitical/impl-display-for-vec for reference
pub struct Gigs(pub Vec<Gig>);

impl Deref for Gigs {
    type Target = Vec<Gig>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl fmt::Display for Gigs {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.iter().fold(Ok(()), |result, gig| {
            result.and_then(|_| writeln!(f, "• {}", gig))
        })
    }
}
