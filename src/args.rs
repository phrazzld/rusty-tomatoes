use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct RustyTomatoesArgs {
    #[clap(subcommand)]
    pub entity_type: EntityType,
}

#[derive(Debug, Subcommand)]
pub enum EntityType {
    /// Create, update, delete, or show movies
    Movie(MovieCommand),
}

#[derive(Debug, Args)]
pub struct MovieCommand {
    #[clap(subcommand)]
    pub command: MovieSubcommand,
}

#[derive(Debug, Subcommand)]
pub enum MovieSubcommand {
    /// Create a new movie
    Create(CreateMovie),

    /// Delete an existing movie
    Delete(DeleteMovie),

    /// Show all movies
    Show,
}

#[derive(Debug, Args)]
pub struct CreateMovie {
    /// The title of the movie
    pub title: String,
    /// The date the movie was watched (YYYY-MM-DD)
    pub watched_at: String,
}

#[derive(Debug, Args)]
pub struct DeleteMovie {
    /// The id of the movie to delete
    pub id: i32,
}
