use std::fmt::Display;

use crate::game::Gold;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct IdleEntity {
    name: String,
    level: u32,
    base_gold_per_second: Gold,
    base_upgrade_cost: Gold,
    upgrade_cost_factor: Gold,
}

impl IdleEntity {
    pub fn build(
        name: String,
        base_gold_per_second: Gold,
        base_upgrade_cost: Gold,
        upgrade_cost_factor: f64,
    ) -> Self {
        IdleEntity {
            name: name.to_owned(),
            level: 0,
            base_gold_per_second: base_gold_per_second,
            base_upgrade_cost: base_upgrade_cost,
            upgrade_cost_factor: upgrade_cost_factor,
        }
    }

    fn add_level(&mut self, amount: u32) {
        self.level += amount;
    }

    pub fn cost_for_next_upgrade(&self) -> Gold {
        self.base_upgrade_cost * f64::powi(self.upgrade_cost_factor, self.level as i32)
    }

    /// Calculates the cummulative cost of upgrades
    pub fn cost_for_next_upgrades(&self, levels_to_upgrade: u32) -> Gold {
        // offset levels by -1 to consider that upgrade
        // calculations starts with 0 and level with 1
        let from_level = self.level as i32;
        let to_level = (self.level + levels_to_upgrade) as i32;

        let b = self.base_upgrade_cost;
        let u = self.upgrade_cost_factor;

        // geometric series runs to n-1
        let from_value = b * (1.0 - f64::powi(u, from_level)) / (1.0 - u);
        let to_value = b * (1.0 - f64::powi(u, to_level)) / (1.0 - u);

        to_value - from_value
    }

    pub fn quanity_of_possible_upgrades(&self, gold: &Gold) -> u32 {
        let b = self.base_upgrade_cost;
        let u = self.upgrade_cost_factor;
        let l = self.level as i32;
        let n = f64::log((gold / (b * f64::powi(u, l))) * (u - 1.0) + 1.0, u);
        n as u32
    }

    /// Calculates the gold gain in deltaTime milliseconds
    pub fn get_gold(&self, delta_time: u128) -> Gold {
        self.gold_per_second() * (delta_time as f64) / 1000.0
    }

    pub fn gold_per_second(&self) -> Gold {
        ((self.level + 1) as f64) * self.base_gold_per_second
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_level(&self) -> u32 {
        self.level
    }

    /// Upgrades the entity by 1 and returns the remaining gold
    pub fn upgrade(&mut self, gold: &mut Gold) -> Result<(), ()> {
        if *gold >= self.cost_for_next_upgrade() {
            *gold -= self.cost_for_next_upgrade();
            self.add_level(1);
            return Ok(());
        }
        return Err(());
    }
}

impl Display for IdleEntity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[{}] {:<16} {:>8.02} g/s.",
            self.level,
            self.name.as_str(),
            self.gold_per_second()
        )
    }
}

#[cfg(test)]
mod tests {
    use float_cmp::approx_eq;

    use super::IdleEntity;
    use crate::game::Gold;

    fn create_entity() -> IdleEntity {
        IdleEntity {
            name: "Entity".to_owned(),
            level: 0,
            base_gold_per_second: 0.1,
            base_upgrade_cost: 1.0,
            upgrade_cost_factor: 2.0,
        }
    }

    #[test]
    fn gold_gain_after_1_seconds() {
        let entity = create_entity();
        let delta_time = 1000 * 1;

        let gold_gain = entity.get_gold(delta_time);

        assert!(approx_eq!(
            Gold,
            entity.gold_per_second() * 1.0,
            gold_gain,
            ulps = 2
        ));
    }

    #[test]
    fn gold_gain_after_3_seconds() {
        let entity = create_entity();
        let delta_time = 1000 * 3;

        let gold_gain = entity.get_gold(delta_time);

        assert!(approx_eq!(
            Gold,
            entity.gold_per_second() * 3.0,
            gold_gain,
            ulps = 2
        ));
    }

    #[test]
    fn gold_per_second_of_level_3_entity() {
        let mut entity = create_entity();
        entity.add_level(2);

        let gold_per_second = entity.base_gold_per_second * (entity.level + 1) as f64;
        assert!(approx_eq!(
            Gold,
            gold_per_second,
            entity.gold_per_second(),
            ulps = 2
        ))
    }

    #[test]
    fn upgrade_cost_of_level_3_entity() {
        let mut entity = create_entity();
        entity.add_level(2);

        assert!(approx_eq!(
            Gold,
            entity.base_upgrade_cost * entity.upgrade_cost_factor * entity.upgrade_cost_factor,
            entity.cost_for_next_upgrade(),
            ulps = 2
        ))
    }

    #[test]
    fn cummulative_upgrade_cost_for_first_10_upgrades() {
        let entity = create_entity();
        let total_upgrade_cost = entity.cost_for_next_upgrades(10);
        let total_cost: Gold = entity.base_upgrade_cost * f64::powi(entity.upgrade_cost_factor, 0)
            + entity.base_upgrade_cost * f64::powi(entity.upgrade_cost_factor, 1)
            + entity.base_upgrade_cost * f64::powi(entity.upgrade_cost_factor, 2)
            + entity.base_upgrade_cost * f64::powi(entity.upgrade_cost_factor, 3)
            + entity.base_upgrade_cost * f64::powi(entity.upgrade_cost_factor, 4)
            + entity.base_upgrade_cost * f64::powi(entity.upgrade_cost_factor, 5)
            + entity.base_upgrade_cost * f64::powi(entity.upgrade_cost_factor, 6)
            + entity.base_upgrade_cost * f64::powi(entity.upgrade_cost_factor, 7)
            + entity.base_upgrade_cost * f64::powi(entity.upgrade_cost_factor, 8)
            + entity.base_upgrade_cost * f64::powi(entity.upgrade_cost_factor, 9);

        assert_eq!(total_cost, total_upgrade_cost);
        assert!(approx_eq!(Gold, total_cost, total_upgrade_cost, ulps = 2))
    }

    #[test]
    fn upgrade_entity_once() {
        let mut entity = create_entity();
        let start_gold: Gold = 2.0;
        let mut gold: Gold = start_gold.clone();

        let _ = entity.upgrade(&mut gold);
        let total_cost: Gold = entity.base_upgrade_cost;

        assert_eq!(1, entity.level);
        assert!(approx_eq!(Gold, start_gold - total_cost, gold, ulps = 2));
    }

    #[test]
    fn upgrade_entity_three_times() {
        let mut entity = create_entity();
        let start_gold: Gold = 100.0;
        let mut gold: Gold = start_gold.clone();

        let _ = entity.upgrade(&mut gold);
        let _ = entity.upgrade(&mut gold);
        let _ = entity.upgrade(&mut gold);

        assert_eq!(3, entity.level);
        let total_cost: Gold = entity.base_upgrade_cost
            + entity.base_upgrade_cost * entity.upgrade_cost_factor
            + entity.base_upgrade_cost * entity.upgrade_cost_factor * entity.upgrade_cost_factor;
        assert!(approx_eq!(Gold, start_gold - total_cost, gold, ulps = 2));
    }

    #[test]
    fn quanity_of_possible_upgrades_from_level_0() {
        let mut entity = create_entity();
        entity.upgrade_cost_factor = 2.0;
        let gold: Gold = 15.0;

        let quantity = entity.quanity_of_possible_upgrades(&gold);

        let total_cost_of_possible_upgrades: Gold = entity.base_upgrade_cost
            * f64::powf(entity.upgrade_cost_factor, 0.0)
            + entity.base_upgrade_cost * f64::powf(entity.upgrade_cost_factor, 1.0)
            + entity.base_upgrade_cost * f64::powf(entity.upgrade_cost_factor, 2.0)
            + entity.base_upgrade_cost * f64::powf(entity.upgrade_cost_factor, 3.0);

        assert_eq!(4, quantity);
        assert_eq!(total_cost_of_possible_upgrades, gold);
    }

    #[test]
    fn quanity_of_possible_upgrades_from_level_3() {
        let mut entity = create_entity();
        entity.upgrade_cost_factor = 2.0;
        entity.add_level(3);
        let gold: Gold = 56.0;

        let quantity = entity.quanity_of_possible_upgrades(&gold);
        let total_cost_of_possible_upgrades: Gold = entity.base_upgrade_cost
            * f64::powf(entity.upgrade_cost_factor, 3.0)
            + entity.base_upgrade_cost * f64::powf(entity.upgrade_cost_factor, 4.0)
            + entity.base_upgrade_cost * f64::powf(entity.upgrade_cost_factor, 5.0);

        assert_eq!(3, quantity);
        assert_eq!(total_cost_of_possible_upgrades, gold);
    }
}
