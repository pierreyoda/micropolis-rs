use rand::Rng;

use crate::{
    game::{GameLevelDifficulty, GameScenario},
    map::tiles::TILE_LOW_MASK,
    map::MapPosition,
    map::TileMap,
    map::TileType,
    map::WORLD_HEIGHT,
    map::WORLD_WIDTH,
    utils::random_in_range,
};

/// Chance of disasters at levels 0, 1 and 2 respectively.
const DISASTER_CHANCE: [u16; 3] = [
    10 * 48, // Game level 0
    5 * 48,  // Game level 1
    60,      // Game level 2
];

pub struct CityDisasters {
    /// Size of flooding disaster.
    flood_count: i16,
    /// The disaster for which a count-down is running.
    disaster_event: GameScenario,
    /// Count-down timer for the disaster.
    disaster_timer: u16,
}

impl CityDisasters {
    /// Let disasters happen.
    pub fn do_disasters(&mut self, difficulty: &GameLevelDifficulty, scenario: &GameScenario) {
        if self.flood_count > 0 {
            self.flood_count -= 1;
        }

        if self.disaster_event != GameScenario::None {}
    }

    fn scenario_disaster(&mut self) {
        match self.disaster_event {
            GameScenario::Dullsville => (),
            GameScenario::SanFrancisco if self.disaster_timer == 1 => self.make_earthquake(),
            GameScenario::Hamburg if self.disaster_timer % 10 == 0 => self.make_firebombs(),
            GameScenario::Bern => (),
            GameScenario::Tokyo if self.disaster_timer == 1 => self.make_monster(),
            GameScenario::Detroit => (),
            GameScenario::Boston if self.disaster_timer == 1 => self.make_meltdown(),
            GameScenario::Rio if (self.disaster_timer % 24) == 0 => self.make_flood(),
            GameScenario::None => unreachable!(),
        }

        if self.disaster_timer > 0 {
            self.disaster_timer -= 1;
        } else {
            self.disaster_event = GameScenario::None;
        }
    }

    /// Make a nuclear power plant melt.
    /// TODO: randomize which nuke plant melts down
    fn make_meltdown(map: &mut TileMap) -> Result<(), String> {
        for x in 0..WORLD_WIDTH {
            for y in 0..WORLD_HEIGHT {
                let position = MapPosition::new(x as i32, y as i32);
                let tile = map.get_tile_at(&position).ok_or(format!(
                    "CityDisasters::make_meltdown cannot get tile at position {}.",
                    position
                ))?;
                if tile.get_raw() & TILE_LOW_MASK == TileType::Nuclear.to_u16().unwrap() {
                    return self.do_meltdown();
                }
            }
        }

        Ok(())
    }

    /// Let a fire bomb explode at a random location.
    fn fire_bomb<R: Rng>(rng: &mut R) {
        let at = MapPosition::new(
            random_in_range(rng, 0, (WORLD_WIDTH - 1) as i32),
            random_in_range(rng, 0, (WORLD_HEIGHT - 1) as i32),
        );
    }

    /// Construct an explosion sprite at the given position.
    fn make_explosion(at: &MapPosition, map: &TileMap) {
        if !map.in_bounds(at) {
            return;
        }
        CityDisasters::make_explosion_at(&MapPosition::new(
            (at.get_x() << 4) + 8,
            (at.get_y() << 4) + 8,
        ))
    }

    /// Construct an explosion sprite.
    pub fn make_explosion_at(at: &MapPosition) {}
}
