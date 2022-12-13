use crate::idle_entity::IdleEntity;
use serde::{Deserialize, Serialize};
use std::{
    time::{Duration, SystemTime},
    vec,
};

pub type Gold = f64;
pub enum IdleEntityType {
    Lumberjack = 0,
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
            idle_entities: vec![IdleEntity::build("Lumberjack", 0.1, 1.00, 1.15)],
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

        self.current_gold += self.idle_entities[0].get_gold(self.current_delta_time.as_millis());

        self.last_time_stamp = SystemTime::now();
    }

    pub fn upgrade(&mut self, entity_type: IdleEntityType, amount: u32) {
        let mut successful_upgrades = 0;
        while successful_upgrades < amount {
            let result = match entity_type {
                IdleEntityType::Lumberjack => self.idle_entities[0].upgrade(&mut self.current_gold),
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
        println!("{:>4}{}", indent, self.idle_entities[0]);
    }
}
