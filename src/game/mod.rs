pub mod game_state;
pub mod idle_entity;

use serde::{Deserialize, Serialize};
use std::{
    fmt,
    time::{Duration, SystemTime},
    vec,
};

use self::{
    game_state::{GameInformation, GameState, IdleEntityInformation},
    idle_entity::IdleEntity,
};

pub type Gold = f64;
pub enum IdleEntityType {
    Lumberjack = 0,
    Stonemason = 1,
    Bowmaker = 2,
    Weaponsmith = 3,
    Academic = 4,
    Catapult = 5,
    King = 6,
}

impl fmt::Display for IdleEntityType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IdleEntityType::Lumberjack => write!(f, "Lumberjack"),
            IdleEntityType::Stonemason => write!(f, "Stonemason"),
            IdleEntityType::Bowmaker => write!(f, "Bowmaker"),
            IdleEntityType::Weaponsmith => write!(f, "Weaponsmith"),
            IdleEntityType::Academic => write!(f, "Academic"),
            IdleEntityType::Catapult => write!(f, "Catapult"),
            IdleEntityType::King => write!(f, "King"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Game {
    init_time_stamp: SystemTime,
    last_time_stamp: SystemTime,
    current_delta_time: Duration,
    current_gold: Gold,
    idle_entities: Vec<IdleEntity>,
}

// Game associated functions
impl Game {
    pub fn new() -> Self {
        Game {
            init_time_stamp: SystemTime::now(),
            last_time_stamp: SystemTime::now(),
            current_gold: 0.0,
            idle_entities: vec![
                IdleEntity::build(IdleEntityType::Lumberjack.to_string(), 0.1, 1.00, 1.15),
                IdleEntity::build(IdleEntityType::Stonemason.to_string(), 1.0, 10.00, 1.35),
                IdleEntity::build(IdleEntityType::Bowmaker.to_string(), 20.0, 1_000.00, 1.55),
                IdleEntity::build(
                    IdleEntityType::Weaponsmith.to_string(),
                    350.0,
                    100_000.00,
                    1.8,
                ),
                IdleEntity::build(
                    IdleEntityType::Academic.to_string(),
                    1350.0,
                    10_000_000.00,
                    2.15,
                ),
                IdleEntity::build(
                    IdleEntityType::Catapult.to_string(),
                    5000.0,
                    1_000_000_000.00,
                    3.15,
                ),
                IdleEntity::build(
                    IdleEntityType::King.to_string(),
                    20_000.0,
                    1_000_000_000_000.00,
                    4.0,
                ),
            ],
            current_delta_time: Duration::new(0, 0),
        }
    }
}

// Game methods
impl Game {
    pub fn update(&mut self) -> GameState {
        self.current_delta_time = SystemTime::now()
            .duration_since(self.last_time_stamp)
            .unwrap();

        self.update_gold();
        self.create_game_state();

        self.last_time_stamp = SystemTime::now();

        self.create_game_state()
    }

    fn update_gold(&mut self) {
        for entity in &self.idle_entities {
            self.current_gold += entity.get_gold(self.current_delta_time.as_millis())
        }
    }

    pub fn create_game_state(&self) -> GameState {
        let total_gold_per_second = self
            .idle_entities
            .iter()
            .map(|entity| entity.gold_per_second())
            .sum();

        let game_information = GameInformation {
            gold: self.current_gold,
            gold_per_second: total_gold_per_second,
        };

        GameState {
            game_info: game_information,
            lumberjack_info: self.create_idle_entity_info(0, total_gold_per_second),
            stonemason_info: self.create_idle_entity_info(1, total_gold_per_second),
            bowmaker_info: self.create_idle_entity_info(2, total_gold_per_second),
            weaponsmith_info: self.create_idle_entity_info(3, total_gold_per_second),
            academic_info: self.create_idle_entity_info(4, total_gold_per_second),
            catapult_info: self.create_idle_entity_info(5, total_gold_per_second),
            king_info: self.create_idle_entity_info(6, total_gold_per_second),
        }
    }

    fn create_idle_entity_info(
        &self,
        idx: usize,
        total_gold_per_second: f64,
    ) -> IdleEntityInformation {
        let upgrade_quanity =
            self.idle_entities[idx].quanity_of_possible_upgrades(&self.current_gold);

        IdleEntityInformation {
            name: self.idle_entities[idx].get_name().to_string(),
            level: self.idle_entities[idx].get_level(),
            gold_per_second: self.idle_entities[idx].gold_per_second(),
            gold_per_second_percent: self.idle_entities[idx].gold_per_second()
                / total_gold_per_second,
            maximum_upgrade_quantity: upgrade_quanity,
            upgrade_cost_next: self.idle_entities[idx].cost_for_next_upgrade(),
            upgrade_cost_max: self.idle_entities[idx].cost_for_next_upgrades(upgrade_quanity),
        }
    }

    pub fn upgrade(&mut self, entity_type: IdleEntityType, amount: u32) -> u32 {
        let mut successful_upgrades = 0;
        while successful_upgrades < amount {
            let result = match entity_type {
                IdleEntityType::Lumberjack => self.idle_entities[0].upgrade(&mut self.current_gold),
                IdleEntityType::Stonemason => self.idle_entities[1].upgrade(&mut self.current_gold),
                IdleEntityType::Bowmaker => self.idle_entities[2].upgrade(&mut self.current_gold),
                IdleEntityType::Weaponsmith => {
                    self.idle_entities[3].upgrade(&mut self.current_gold)
                }
                IdleEntityType::Academic => self.idle_entities[4].upgrade(&mut self.current_gold),
                IdleEntityType::Catapult => self.idle_entities[5].upgrade(&mut self.current_gold),
                IdleEntityType::King => self.idle_entities[6].upgrade(&mut self.current_gold),
            };

            if let Err(()) = result {
                break;
            }

            successful_upgrades += 1;
        }
        successful_upgrades
    }

    pub fn get_delta_time(&self) -> &Duration {
        &self.current_delta_time
    }
}
