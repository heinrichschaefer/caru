use std::time::{Duration, SystemTime};

use serde::{Deserialize, Serialize};

use crate::idle_entity::IdleEntity;

#[derive(Serialize, Deserialize, Debug)]
pub struct Game {
    init_time_stamp: SystemTime,
    last_time_stamp: SystemTime,
    current_delta_time: Duration,
    current_gold: f32,
    lumberjack: IdleEntity,
}

// Game associated functions
impl Game {
    pub fn new() -> Self {
        Game {
            init_time_stamp: SystemTime::now(),
            last_time_stamp: SystemTime::now(),
            current_gold: 0.0,
            lumberjack: IdleEntity::build("Lumberjack", 0.1, 1.05, 1.05),
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

        self.current_gold += self
            .lumberjack
            .get_gold(self.current_delta_time.as_millis());

        self.last_time_stamp = SystemTime::now();
    }

    pub fn display_status(&self) {
        let indent = " ";
        println!(
            "[Game Status] - Time passed since last check: [{:?}]",
            self.current_delta_time
        );
        println!("{:>4}Current Gold: {:.2} Gold", indent, self.current_gold);
        println!("{:>4}{}", indent, self.lumberjack);
    }
}
