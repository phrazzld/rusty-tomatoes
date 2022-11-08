use crate::schema::movies;
use diesel::prelude::*;

#[derive(Queryable)]
pub struct Movie {
    pub id: i32,
    pub title: String,
    pub watched_at: String,
    pub created_at: String,
}

#[derive(Insertable)]
#[diesel(table_name = movies)]
pub struct NewMovie<'a> {
    pub title: &'a str,
    pub watched_at: &'a chrono::NaiveDateTime,
}

impl std::fmt::Display for Movie {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{{ id: {:?}, title: {:?}, watched_at: {:?}, created_at: {:?} }}",
            self.id, self.title, self.watched_at, self.created_at
        )
    }
}
