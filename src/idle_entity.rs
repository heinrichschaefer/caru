use std::fmt::Display;

use crate::game::Gold;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct IdleEntity {
    name: String,
    level: i32,
    base_gold_per_second: Gold,
    base_upgrade_cost: Gold,
    upgrade_cost_factor: Gold,
}

impl IdleEntity {
    pub fn build(
        name: &str,
        base_gold_per_second: Gold,
        base_upgrade_cost: Gold,
        upgrade_cost_factor: f64,
    ) -> Self {
        IdleEntity {
            name: name.to_owned(),
            level: 1,
            base_gold_per_second: base_gold_per_second,
            base_upgrade_cost: base_upgrade_cost,
            upgrade_cost_factor: upgrade_cost_factor,
        }
    }

    fn add_level(&mut self, amount: i32) {
        self.level += amount;
    }

    pub fn calculate_upgrade_cost(&self) -> Gold {
        self.base_upgrade_cost * f64::powf(self.upgrade_cost_factor, (self.level - 1) as f64)
    }

    /// Calculates the gold gain in deltaTime milliseconds
    pub fn get_gold(&self, delta_time: u128) -> Gold {
        self.gold_per_second() * (delta_time as f64) / 1000.0
    }

    pub fn gold_per_second(&self) -> Gold {
        (self.level as f64) * self.base_gold_per_second
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }
    /// Upgrades the entity by 1 and returns the remaining gold
    pub fn upgrade(&mut self, gold: &mut Gold) -> Result<(), ()> {
        if *gold >= self.calculate_upgrade_cost() {
            *gold -= self.calculate_upgrade_cost();
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
            "{} [{}]: {} g/s.",
            self.name.as_str(),
            self.level,
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
            level: 1,
            base_gold_per_second: 0.1,
            base_upgrade_cost: 1.0,
            upgrade_cost_factor: 1.15,
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

        assert!(approx_eq!(
            Gold,
            entity.base_gold_per_second * entity.level as f64,
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
            entity.calculate_upgrade_cost(),
            ulps = 2
        ))
    }

    #[test]
    fn upgrade_entity_once() {
        let mut entity = create_entity();
        let start_gold: Gold = 2.0;
        let mut gold: Gold = start_gold.clone();

        entity.upgrade(&mut gold);
        let total_cost: Gold = entity.base_upgrade_cost;

        assert_eq!(2, entity.level);
        assert!(approx_eq!(Gold, start_gold - total_cost, gold, ulps = 2));
    }

    #[test]
    fn upgrade_entity_three_times() {
        let mut entity = create_entity();
        let start_gold: Gold = 100.0;
        let mut gold: Gold = start_gold.clone();

        entity.upgrade(&mut gold);
        entity.upgrade(&mut gold);
        entity.upgrade(&mut gold);

        assert_eq!(4, entity.level);
        let total_cost: Gold = entity.base_upgrade_cost
            + entity.base_upgrade_cost * entity.upgrade_cost_factor
            + entity.base_upgrade_cost * entity.upgrade_cost_factor * entity.upgrade_cost_factor;
        assert!(approx_eq!(Gold, start_gold - total_cost, gold, ulps = 2));
    }
}
