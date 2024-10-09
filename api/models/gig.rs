use crate::schema::gig;
use chrono::{DateTime, Local};
use diesel::prelude::*;

use itertools::Itertools;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::ops::Deref;

#[derive(Serialize, Deserialize, Debug, Identifiable, Queryable)]
#[diesel(belongs_to(Artist, foreign_key = "id"))]
#[diesel(table_name = gig)]
pub struct Gig {
    pub id: uuid::Uuid,
    pub title: String,
    pub location: String,
    pub date: chrono::NaiveDateTime,
    pub artist_id: uuid::Uuid,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

impl fmt::Display for Gig {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let formatted_date =
            DateTime::<Local>::from_naive_utc_and_offset(self.date, *Local::now().offset())
                .format("%A%v");

        write!(f, "{}, {}, {}", self.title, self.location, formatted_date)
    }
}

// This is necessary to display multiple gigs
// See https://github.com/apolitical/impl-display-for-vec for reference
#[derive(Debug, Serialize, Deserialize)]
pub struct Gigs(pub Vec<Gig>);

impl Deref for Gigs {
    type Target = Vec<Gig>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl fmt::Display for Gigs {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // self.0.iter().fold(Ok(()), |result, gig| {
        //     result.and_then(|_| writeln!(f, "• {}", gig))
        // })
        let data_formatter = self
            .0
            .iter()
            .format_with("\n", |gig, f| f(&format_args!("• {}", gig)));
        write!(f, "{}", data_formatter)
    }
}