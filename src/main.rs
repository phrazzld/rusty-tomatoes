mod args;
mod db;
mod models;
mod schema;

use self::models::*;
use args::{CreateMovie, EntityType, MovieCommand, MovieSubcommand, RustyTomatoesArgs};
use clap::Parser;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

fn main() {
    let args = RustyTomatoesArgs::parse();

    match args.entity_type {
        EntityType::Movie(movie) => handle_movie_command(movie),
    }
}

pub fn handle_movie_command(movie: MovieCommand) {
    match movie.command {
        MovieSubcommand::Create(movie) => handle_create_movie(movie),
        MovieSubcommand::Show => handle_show_movies(),
    }
}

pub fn handle_show_movies() {
    let connection = &mut db::establish_connection();
    let results = schema::movies::table
        .load::<Movie>(connection)
        .expect("Error loading movies");
    results
        .iter()
        .for_each(|movie| println!("id: {:?} || title: {:?}", movie.id, movie.title));
}

pub fn handle_create_movie(movie: CreateMovie) {
    let connection = &mut db::establish_connection();
    create_movie(connection, &movie.title);
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
