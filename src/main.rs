mod args;
mod config;
mod game;
mod tui;

use std::{io, process};

use args::CaruArgs;
use clap::Parser;
use config::Config;
use game::IdleEntityType;

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

            let game_state = game.update();
            tui::display_status(&game_state, game.get_delta_time());

            config.save(game)
        }
        args::BasicCommand::Upgrade(entity_args) => {
            let mut game = config.load()?;
            let game_state = game.update();

            match entity_args.entity {
                args::IdleEntityArg::All => panic!("Upgrade all is not implemented"),
                args::IdleEntityArg::Lumberjack(amendment_arg) => {
                    if let Some(entity_count_arg) = amendment_arg.count {
                        let successful_upgrades = game.upgrade(
                            IdleEntityType::Lumberjack,
                            get_amount_from_entity_count_arg(&entity_count_arg),
                        );
                        tui::confirm_upgrade_info(
                            successful_upgrades,
                            get_amount_from_entity_count_arg(&entity_count_arg),
                            &game_state.lumberjack_info.name,
                        );
                    };
                    if amendment_arg.info {
                        tui::display_upgrade_info(&game_state.lumberjack_info, &game_state)
                    };
                }
                args::IdleEntityArg::Stonemason(amendment_arg) => {
                    if let Some(entity_count_arg) = amendment_arg.count {
                        game.upgrade(
                            IdleEntityType::Stonemason,
                            get_amount_from_entity_count_arg(&entity_count_arg),
                        );
                    };
                    if amendment_arg.info {
                        tui::display_upgrade_info(&game_state.stonemason_info, &game_state)
                    };
                }
                args::IdleEntityArg::Bowmaker(amendment_arg) => {
                    if let Some(entity_count_arg) = amendment_arg.count {
                        game.upgrade(
                            IdleEntityType::Bowmaker,
                            get_amount_from_entity_count_arg(&entity_count_arg),
                        );
                    };
                    if amendment_arg.info {
                        tui::display_upgrade_info(&game_state.bowmaker_info, &game_state)
                    };
                }
                args::IdleEntityArg::Weaponsmith(amendment_arg) => {
                    if let Some(entity_count_arg) = amendment_arg.count {
                        game.upgrade(
                            IdleEntityType::Weaponsmith,
                            get_amount_from_entity_count_arg(&entity_count_arg),
                        );
                    };
                    if amendment_arg.info {
                        tui::display_upgrade_info(&game_state.weaponsmith_info, &game_state)
                    };
                }
                args::IdleEntityArg::Academic(amendment_arg) => {
                    if let Some(entity_count_arg) = amendment_arg.count {
                        game.upgrade(
                            IdleEntityType::Academic,
                            get_amount_from_entity_count_arg(&entity_count_arg),
                        );
                    };
                    if amendment_arg.info {
                        tui::display_upgrade_info(&game_state.academic_info, &game_state)
                    };
                }
                args::IdleEntityArg::Catapult(amendment_arg) => {
                    if let Some(entity_count_arg) = amendment_arg.count {
                        game.upgrade(
                            IdleEntityType::Catapult,
                            get_amount_from_entity_count_arg(&entity_count_arg),
                        );
                    };
                    if amendment_arg.info {
                        tui::display_upgrade_info(&game_state.catapult_info, &game_state)
                    };
                }
                args::IdleEntityArg::King(amendment_arg) => {
                    if let Some(entity_count_arg) = amendment_arg.count {
                        game.upgrade(
                            IdleEntityType::King,
                            get_amount_from_entity_count_arg(&entity_count_arg),
                        );
                    };
                    if amendment_arg.info {
                        tui::display_upgrade_info(&game_state.king_info, &game_state)
                    };
                }
            }
            config.save(game)
        }
    }
}

fn get_amount_from_entity_count_arg(arg: &args::EntityCountArg) -> u32 {
    match arg {
        args::EntityCountArg::One => 1,
        args::EntityCountArg::All => u32::MAX,
        args::EntityCountArg::Amount(n) => *n,
    }
}
