use std::{
    fs,
    fs::File,
    io::{self, BufReader},
    path::PathBuf,
};

use crate::game::Game;

const GAME_FOLDER_NAME: &str = ".caru";
const GAME_FILE_NAME: &str = "game.json";

pub struct Config {
    pub game_directory: PathBuf,
    pub game_file_path: PathBuf,
}

impl Config {
    pub fn build(home_dir: &PathBuf) -> Config {
        let mut game_dir = PathBuf::new();
        game_dir.push(home_dir);
        game_dir.push(GAME_FOLDER_NAME);

        let mut game_file_path = PathBuf::from(&game_dir);
        game_file_path.push(GAME_FILE_NAME);
        Config {
            game_directory: game_dir,
            game_file_path: game_file_path,
        }
    }

    pub fn load(&self) -> Result<Game, io::Error> {
        let file = File::open(&self.game_file_path)?;
        let reader = BufReader::new(file);

        // Read the JSON contents of the file as an instance of `Game`.
        let game = serde_json::from_reader(reader)?;

        Ok(game)
    }

    pub fn save(&self, game: Game) -> io::Result<()> {
        let serialized_game = serde_json::to_string(&game).unwrap();
        fs::write(&self.game_file_path, &serialized_game.as_bytes())?;

        Ok(())
    }
}

pub fn init_game(config: &Config) -> io::Result<()> {
    fs::create_dir(&config.game_directory)?;

    println!(
        "Successfully created a new game directory at {}",
        config.game_directory.display()
    );
    config.save(Game {
        ..Default::default()
    })?;
    println!(
        "Successfully created a new game file at {}",
        config.game_file_path.display()
    );
    Ok(())
}

pub fn delete_game_directory(path: &mut PathBuf) -> io::Result<()> {
    path.push(GAME_FOLDER_NAME);
    fs::remove_dir_all(&path)?;

    println!("Successfully removed all files at {}", path.display());
    Ok(())
}
