use crate::idle_entity::IdleEntity;
use serde::{Deserialize, Serialize};
use std::{
    fmt,
    time::{Duration, SystemTime},
    vec,
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
    pub fn update(&mut self) {
        self.current_delta_time = SystemTime::now()
            .duration_since(self.last_time_stamp)
            .unwrap();

        for entity in &self.idle_entities {
            self.current_gold += entity.get_gold(self.current_delta_time.as_millis())
        }

        self.last_time_stamp = SystemTime::now();
    }

    pub fn upgrade(&mut self, entity_type: IdleEntityType, amount: u32) {
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
        self.display_upgrade_status(successful_upgrades, amount, &entity_type);
    }
}

// Display methods
impl Game {
    pub fn display_upgrade_info(&self, entity_type: &IdleEntityType) {
        let entity = match entity_type {
            IdleEntityType::Lumberjack => &self.idle_entities[0],
            IdleEntityType::Stonemason => &self.idle_entities[1],
            IdleEntityType::Bowmaker => &self.idle_entities[2],
            IdleEntityType::Weaponsmith => &self.idle_entities[3],
            IdleEntityType::Academic => &self.idle_entities[4],
            IdleEntityType::Catapult => &self.idle_entities[5],
            IdleEntityType::King => &self.idle_entities[6],
        };

        let quanity = entity.quanity_of_possible_upgrades(&self.current_gold);
        let total_cost = entity.cost_for_next_upgrades(quanity);

        let indent = " ";
        println!("Information to upgrade {}:", entity.get_name());
        println!(
            "{:>4}{} upgrades [{}] -> [{}]: {:.2} Gold [{:.2} Gold]",
            indent,
            entity.get_name(),
            entity.get_level(),
            entity.get_level() + quanity,
            total_cost,
            self.current_gold
        );
    }

    fn display_upgrade_status(
        &self,
        successful_upgrades: u32,
        amount_to_upgrade: u32,
        entity_type: &IdleEntityType,
    ) {
        let entity_name = match entity_type {
            IdleEntityType::Lumberjack => self.idle_entities[0].get_name(),
            IdleEntityType::Stonemason => self.idle_entities[1].get_name(),
            IdleEntityType::Bowmaker => self.idle_entities[2].get_name(),
            IdleEntityType::Weaponsmith => self.idle_entities[3].get_name(),
            IdleEntityType::Academic => self.idle_entities[4].get_name(),
            IdleEntityType::Catapult => self.idle_entities[5].get_name(),
            IdleEntityType::King => self.idle_entities[6].get_name(),
        };

        println!(
            "Successfully upgraded {}/{} {}",
            successful_upgrades, amount_to_upgrade, entity_name
        )
    }

    pub fn display_status(&self) {
        let indent = " ";
        println!(
            "[Game Status] - Time passed since last check: [{:?}]",
            self.current_delta_time
        );
        println!("{:>4}Current Gold: {:.2} Gold", indent, self.current_gold);
        for entity in &self.idle_entities {
            println!("{:>8}{}", indent, entity);
        }
    }
}
