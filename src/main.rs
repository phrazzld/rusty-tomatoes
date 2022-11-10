mod args;
mod db;
mod models;
mod schema;

use self::models::*;
use args::{
    CreateMovie, DeleteMovie, EntityType, MovieCommand, MovieSubcommand, RustyTomatoesArgs,
};
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
        MovieSubcommand::Delete(movie) => handle_delete_movie(movie),
        MovieSubcommand::Show => handle_show_movies(),
    }
}

pub fn handle_show_movies() {
    let connection = &mut db::establish_connection();
    let results = schema::movies::table
        .load::<Movie>(connection)
        .expect("Error loading movies");
    results.iter().for_each(|movie| println!("{}", movie));
}

pub fn handle_delete_movie(movie: DeleteMovie) {
    let connection = &mut db::establish_connection();

    let movie = schema::movies::table
        .find(movie.id)
        .first::<Movie>(connection)
        .expect("Error loading movie");
    println!("Deleting movie: {}", movie);

    diesel::delete(schema::movies::table.find(movie.id))
        .execute(connection)
        .expect("Error deleting movie");
}

pub fn handle_create_movie(movie: CreateMovie) {
    let connection = &mut db::establish_connection();
    create_movie(connection, &movie.title, &movie.watched_at);
}

pub fn create_movie(conn: &mut SqliteConnection, title: &str, watched_at: &str) {
    use schema::movies;

    let new_movie = NewMovie {
        title,
        watched_at: &chrono::NaiveDateTime::parse_from_str(
            &(watched_at.to_owned() + " 00:00:00".clone()),
            "%Y-%m-%d %H:%M:%S",
        )
        .unwrap(),
    };

    diesel::insert_into(movies::table)
        .values(&new_movie)
        .execute(conn)
        .expect("Error saving new movie");

    println!("Successfully created movie: \"{}\"", title);
}
