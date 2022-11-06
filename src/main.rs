mod args;
mod db;
mod models;
mod schema;

use self::models::*;
use args::MediaJournalArgs;
use clap::Parser;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

fn main() {
    let connection = &mut db::establish_connection();
    let results = schema::movies::table
        .load::<Movie>(connection)
        .expect("Error loading movies");

    // If there are no results, create a new movie
    // Otherwise, print all movies
    if results.is_empty() {
        create_movie(connection, "The Matrix");
    } else {
        println!("Displaying {} movies", results.len());
        for movie in results {
            println!("{}:: {}", movie.id, movie.title);
        }
    }

    let args = MediaJournalArgs::parse();

    println!("{:#?}", args);
}

pub fn create_movie(conn: &mut SqliteConnection, title: &str) -> Movie {
    use schema::movies;

    let new_movie = NewMovie { title };

    diesel::insert_into(movies::table)
        .values(&new_movie)
        .execute(conn)
        .expect("Error saving new movie");

    movies::table.order(movies::id.desc()).first(conn).unwrap()
}
