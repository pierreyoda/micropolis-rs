#[derive(Clone, Debug)]
pub enum GameLevelDifficulty {
    Easy,
    Normal,
    Hard,
}

impl GameLevelDifficulty {
    pub starting_funds(&self) -> u32 {
        match self {
            Easy => 20000,
            Normal => 10000,
            Hard => 5000,
        }
    }
}

#[derive(Clone, Debug)]
pub struct GameSpeed {
    /// Determines how often the animation timer fires, in milliseconds.
    animations_delay: u32,
    /// Determines how many simulation steps are fired at each screen update.
    ///
    /// One simulation step is triggered for every two animation steps.
    sim_steps_per_update: u32,
}

impl GameSpeed {
    pub fn get_animations_delay(&self) -> u32 { self.animations_delay }
    pub fn get_sim_steps_per_update(&self) -> u32 { self.sim_steps_per_update }
}

#[derive(Clone, Debug)]
pub enum GameSpeedPreset {
    Paused,
    Slow,
    Normal,
    Fast,
    SuperFast,
}

impl Into<GameSpeed> for GameSpeedPreset {
    fn from(preset: GameSpeedPreset): GameSpeed {
        match preset {
            Paused => GameSpeed { animations_delay: 999, sim_steps_per_update: 0 },
            Paused => GameSpeed { animations_delay: 625, sim_steps_per_update: 1 },
            Paused => GameSpeed { animations_delay: 125, sim_steps_per_update: 1 },
            Paused => GameSpeed { animations_delay: 25,  sim_steps_per_update: 1 },
            Paused => GameSpeed { animations_delay: 25,  sim_steps_per_update: 5 },
        }
    }
}
