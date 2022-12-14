// Display methods

use std::time::Duration;

use crate::game::game_state::{GameState, IdleEntityInformation};

const INDENT: &str = " ";

pub fn display_upgrade_info(entity_info: &IdleEntityInformation, game_state: &GameState) {
    println!("Information to upgrade {}:", entity_info.name);
    println!(
        "{:>4}{} upgrades [{}] -> [{}]: {:.2} Gold [{:.2} Gold]",
        INDENT,
        entity_info.name,
        entity_info.level,
        entity_info.level + entity_info.maximum_upgrade_quantity,
        entity_info.upgrade_cost_max,
        game_state.game_info.gold
    );
}

pub fn confirm_upgrade_info(
    successful_upgrades: u32,
    amount_to_upgrade: u32,
    entity_info_name: &String,
) {
    println!(
        "Successfully upgraded {}/{} {}",
        successful_upgrades, amount_to_upgrade, entity_info_name
    )
}

pub fn display_status(game_state: &GameState, delta_time: &Duration) {
    println!(
        "[Game Status] - Time passed since last check: [{:?}]",
        delta_time
    );
    println!(
        "{:>4}Current Gold: {:.2} Gold",
        INDENT, game_state.game_info.gold
    );
    println!(
        "{:>7}{}",
        INDENT,
        display_idle_entity_info(&game_state.lumberjack_info)
    );
    println!(
        "{:>7}{}",
        INDENT,
        display_idle_entity_info(&game_state.stonemason_info)
    );
    println!(
        "{:>7}{}",
        INDENT,
        display_idle_entity_info(&game_state.bowmaker_info)
    );
    println!(
        "{:>7}{}",
        INDENT,
        display_idle_entity_info(&game_state.weaponsmith_info)
    );
    println!(
        "{:>7}{}",
        INDENT,
        display_idle_entity_info(&game_state.academic_info)
    );
    println!(
        "{:>7}{}",
        INDENT,
        display_idle_entity_info(&game_state.catapult_info)
    );
    println!(
        "{:>7}{}",
        INDENT,
        display_idle_entity_info(&game_state.king_info)
    );
}

fn display_idle_entity_info(entity: &IdleEntityInformation) -> String {
    format!(
        "[{:>4}] {:<16} {:>8.02} g/s.",
        entity.level,
        entity.name.as_str(),
        entity.gold_per_second
    )
}
