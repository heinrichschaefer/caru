#[derive(Debug)]
pub struct GameState {
    pub game_info: GameInformation,
    pub lumberjack_info: IdleEntityInformation,
    pub stonemason_info: IdleEntityInformation,
    pub bowmaker_info: IdleEntityInformation,
    pub weaponsmith_info: IdleEntityInformation,
    pub academic_info: IdleEntityInformation,
    pub catapult_info: IdleEntityInformation,
    pub king_info: IdleEntityInformation,
}

#[derive(Debug)]
pub struct GameInformation {
    pub gold_per_second: f64,
    pub gold: f64,
}

#[derive(Debug)]
pub struct IdleEntityInformation {
    pub name: String,
    pub level: u32,
    pub gold_per_second: f64,
    pub gold_per_second_percent: f64,
    pub maximum_upgrade_quantity: u32,
    pub upgrade_cost_next: f64,
    pub upgrade_cost_max: f64,
}
