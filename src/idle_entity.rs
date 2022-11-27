use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct IdleEntity {
    name: String,
    level: i32,
    base_gold_per_second: f32,
    base_upgrade_cost: f32,
    upgrade_cost_factor: f32,
}

impl IdleEntity {
    pub fn build(
        name: &str,
        base_gold_per_second: f32,
        base_upgrade_cost: f32,
        upgrade_cost_factor: f32,
    ) -> Self {
        IdleEntity {
            name: name.to_owned(),
            level: 1,
            base_gold_per_second: base_gold_per_second,
            base_upgrade_cost: base_upgrade_cost,
            upgrade_cost_factor: upgrade_cost_factor,
        }
    }

    /// Calculates the gold gain in deltaTime milliseconds
    pub fn get_gold(&self, delta_time: u128) -> f32 {
        self.gold_per_second() * (delta_time as f32) / 1000.0
    }

    pub fn gold_per_second(&self) -> f32 {
        (self.level as f32) * self.base_gold_per_second
    }

    fn calculate_upgrade_cost(&self) -> f32 {
        self.upgrade_cost_factor * (self.level as f32)
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
    use super::IdleEntity;

    fn create_entity() -> IdleEntity {
        IdleEntity {
            name: "Entity".to_owned(),
            level: 1,
            base_gold_per_second: 0.1,
            base_upgrade_cost: 1.0,
            upgrade_cost_factor: 1.0,
        }
    }

    #[test]
    fn gold_gain_after_1_seconds() {
        let entity = create_entity();
        let delta_time = 1000 * 1;

        let gold_gain = entity.get_gold(delta_time);

        assert_eq!(1.0 * 2.4, gold_gain);
    }

    #[test]
    fn gold_gain_after_3_seconds() {
        let entity = create_entity();
        let delta_time = 1000 * 3;

        let gold_gain = entity.get_gold(delta_time);

        assert_eq!(3.0 * 2.4, gold_gain);
    }
}
