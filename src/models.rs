use crate::schema::movies;
use diesel::prelude::*;

#[derive(Queryable)]
pub struct Movie {
    pub id: i32,
    pub title: String,
}

#[derive(Insertable)]
#[diesel(table_name = movies)]
pub struct NewMovie<'a> {
    pub title: &'a str,
}
