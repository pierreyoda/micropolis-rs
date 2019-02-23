#[derive(Clone, Debug, PartialEq, Eq)]
pub enum GameLevelDifficulty {
    Easy,
    Normal,
    Hard,
}

impl GameLevelDifficulty {
    pub fn starting_funds(&self) -> u32 {
        use GameLevelDifficulty::*;
        match self {
            Easy => 20000,
            Normal => 10000,
            Hard => 5000,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum GameScenario {
    /// Free play.
    None,
    /// Dullsville (boredom).
    Dullsville,
    /// San Francisco (earthquake).
    SanFrancisco,
    /// Hamburg (fire bombs).
    Hamburg,
    /// Bern (traffic).
    Bern,
    /// Tokyo (scary monster).
    Tokyo,
    /// Detroit (crime).
    Detroit,
    /// Boston (nuclear meltdown).
    Boston,
    /// Rio (flooding).
    Rio,
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
    pub fn get_animations_delay(&self) -> u32 {
        self.animations_delay
    }
    pub fn get_sim_steps_per_update(&self) -> u32 {
        self.sim_steps_per_update
    }
}

#[derive(Clone, Debug)]
pub enum GameSpeedPreset {
    Paused,
    Slow,
    Normal,
    Fast,
    SuperFast,
}

impl From<GameSpeedPreset> for GameSpeed {
    fn from(preset: GameSpeedPreset) -> GameSpeed {
        use GameSpeedPreset::*;
        match preset {
            Paused => GameSpeed {
                animations_delay: 999,
                sim_steps_per_update: 0,
            },
            Slow => GameSpeed {
                animations_delay: 625,
                sim_steps_per_update: 1,
            },
            Normal => GameSpeed {
                animations_delay: 125,
                sim_steps_per_update: 1,
            },
            Fast => GameSpeed {
                animations_delay: 25,
                sim_steps_per_update: 1,
            },
            SuperFast => GameSpeed {
                animations_delay: 25,
                sim_steps_per_update: 5,
            },
        }
    }
}
