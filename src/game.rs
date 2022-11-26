use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Game {
    pub lumberjack: u64,
    pub stonemason: u64,
}

impl Game {
    pub fn update(&mut self) {
        self.lumberjack += 2;
        self.stonemason += 1;
    }
}

impl Default for Game {
    fn default() -> Self {
        Self {
            lumberjack: 1,
            stonemason: 0,
        }
    }
}
