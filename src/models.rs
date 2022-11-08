use crate::schema::movies;
use diesel::prelude::*;

#[derive(Queryable)]
pub struct Movie {
    pub id: i32,
    pub title: String,
    pub created_at: String,
}

#[derive(Insertable)]
#[diesel(table_name = movies)]
pub struct NewMovie<'a> {
    pub title: &'a str,
}

impl std::fmt::Display for Movie {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{{ id: {:?}, title: {:?}, created_at: {:?} }}",
            self.id, self.title, self.created_at
        )
    }
}
