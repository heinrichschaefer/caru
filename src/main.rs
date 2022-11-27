mod args;
mod config;
mod game;
mod idle_entity;

use std::{io, process};

use args::CaruArgs;
use clap::Parser;
use config::Config;

fn main() -> io::Result<()> {
    let home_dir = dirs::home_dir().unwrap_or_else(|| {
        eprintln!("Cannot find home directory.");
        process::exit(1)
    });

    let config = Config::build(&home_dir);
    let cli = CaruArgs::parse();

    match cli.command {
        args::BasicCommand::Init => config::init_game(&config),
        args::BasicCommand::Delete => config::delete_game_directory(&config),
        args::BasicCommand::Status => {
            let mut game = config.load()?;

            game.update();
            game.display_status();

            config.save(game)
        }
    }
}
