use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(author, version, about)]
pub struct CaruArgs {
    #[command(subcommand)]
    pub command: BasicCommand,
}

#[derive(Debug, Subcommand)]
pub enum BasicCommand {
    /// Initializes a new game at <User>/.caru
    Init,
    /// Delete all data relating to the current game instance
    Delete,
    /// Gets the current status of the game
    Status,
}
