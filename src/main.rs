mod args;
mod config;
mod game;

use std::{io, process};

use args::CaruArgs;
use clap::Parser;
use config::Config;

fn main() -> io::Result<()> {
    let mut home_dir = dirs::home_dir().unwrap_or_else(|| {
        eprintln!("Cannot find home directory.");
        process::exit(1)
    });

    let config = Config::build(&home_dir);
    let cli = CaruArgs::parse();

    match cli.command {
        args::BasicCommand::Init => config::init_game(&config),
        args::BasicCommand::Delete => config::delete_game_directory(&mut home_dir),
        args::BasicCommand::Status => {
            let mut game = config.load()?;

            println!("Current number of lumberjacks: {}", game.lumberjack);
            println!("Current number of stonemasons: {}", game.stonemason);
            game.update();

            config.save(game)
        }
    }
}
