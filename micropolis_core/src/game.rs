use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{FromPrimitive as FromPrimitiveTrait, ToPrimitive as ToPrimitiveTrait};

#[derive(Clone, Debug, PartialEq, Eq, FromPrimitive, ToPrimitive)]
pub enum GameLevelDifficulty {
    Easy = 0,
    Normal = 1,
    Hard = 2,
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

    pub fn from_usize(value: usize) -> Option<Self> {
        FromPrimitiveTrait::from_usize(value)
    }

    pub fn to_usize(&self) -> Option<usize> {
        match self {
            _ => ToPrimitiveTrait::to_usize(self),
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

impl GameScenario {
    /// Get the disaster delay.
    ///
    /// See `disasterWaitTable` in the C++ code.
    pub fn get_disaster_timer(&self) -> u16 {
        use GameScenario::*;

        match self {
            None => 0,
            Dullsville => 2,
            SanFrancisco => 10,
            Hamburg => 4 * 10,
            Bern => 20,
            Tokyo => 3,
            Detroit => 5,
            Boston => 5,
            Rio => 2 * 48,
        }
    }
}

#[derive(Clone, Debug)]
pub struct GameSpeed {
    /// Determines how often the animation timer fires, in milliseconds.
    animations_delay: u16,
    /// Determines how many simulation steps are fired at each screen update.
    ///
    /// One simulation step is triggered for every two animation steps.
    sim_steps_per_update: u16,
}

impl GameSpeed {
    pub fn get_animations_delay(&self) -> u16 {
        self.animations_delay
    }
    pub fn get_sim_steps_per_update(&self) -> u16 {
        self.sim_steps_per_update
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
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
