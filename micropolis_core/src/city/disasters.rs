use std::todo;

use crate::{
    game::{GameLevelDifficulty, GameScenario},
    map::tiles::TILE_ANIM_BIT,
    map::tiles::TILE_BULL_BIT,
    map::tiles::TILE_BURN_BIT,
    map::tiles::TILE_LOW_MASK,
    map::tiles::TILE_ZONE_BIT,
    map::MapPosition,
    map::Tile,
    map::TileMap,
    map::TileType,
    map::WORLD_HEIGHT,
    map::WORLD_WIDTH,
    utils::random::MicropolisRandom,
};

use super::sprite::{ActiveSpritesList, Sprite, SpriteType};

/// Chance of disasters at levels 0, 1 and 2 respectively.
const DISASTER_CHANCE: [u16; 3] = [
    10 * 48, // Game level 0
    5 * 48,  // Game level 1
    60,      // Game level 2
];

const FLOOD_DX: [i32; 4] = [0, 1, 0, -1];
const FLOOD_DY: [i32; 4] = [-1, 0, 1, 0];

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

    fn scenario_disaster(
        &mut self,
        rng: &mut MicropolisRandom,
        map: &mut TileMap,
    ) -> Result<(), String> {
        match self.disaster_event {
            GameScenario::Dullsville => (),
            GameScenario::SanFrancisco if self.disaster_timer == 1 => {
                Self::make_earthquake(rng, map)?
            }
            GameScenario::Hamburg if self.disaster_timer % 10 == 0 => {
                Self::make_fire_bombs(rng, map)?
            }
            GameScenario::Bern => (),
            // GameScenario::Tokyo if self.disaster_timer == 1 => Self::make_monster(),
            // GameScenario::Detroit => (),
            // GameScenario::Boston if self.disaster_timer == 1 => Self::make_meltdown(),
            // GameScenario::Rio if (self.disaster_timer % 24) == 0 => Self::make_flood(),
            GameScenario::None => unreachable!(),
            _ => todo!(),
        }

        if self.disaster_timer > 0 {
            self.disaster_timer -= 1;
        } else {
            self.disaster_event = GameScenario::None;
        }

        Ok(())
    }

    /// Make a nuclear power plant melt.
    /// TODO: randomize which nuke plant melts down
    fn make_meltdown(rng: &mut MicropolisRandom, map: &mut TileMap) -> Result<(), String> {
        for x in 0..WORLD_WIDTH - 1 {
            for y in 0..WORLD_HEIGHT - 1 {
                let position = MapPosition::new(x as i32, y as i32);
                let tile = map.get_tile_at(&position).ok_or(format!(
                    "CityDisasters::make_meltdown cannot get tile at position {}.",
                    position,
                ))?;
                if tile.get_raw() & TILE_LOW_MASK == TileType::Nuclear.to_u16().unwrap() {
                    return Self::do_meltdown(rng, map, &position);
                }
            }
        }

        Ok(())
    }

    pub fn do_meltdown(
        rng: &mut MicropolisRandom,
        map: &mut TileMap,
        position: &MapPosition,
    ) -> Result<(), String> {
        let (x, y) = position.as_tuple();
        Self::make_explosion(map, &(x - 1, y - 1).into());
        Self::make_explosion(map, &(x - 1, y + 2).into());
        Self::make_explosion(map, &(x + 2, y - 1).into());
        Self::make_explosion(map, &(x + 2, y + 2).into());

        // whole power plant is on fire
        for x in x - 1..x + 3 {
            for y in y - 1..y + 3 {
                map.set_tile_at(position, Tile::from_raw(Self::random_fire(rng)).unwrap());
            }
        }

        // add lots of radiation tiles around the plant
        for z in 0..200 {
            let radiation_position = MapPosition::new(
                position.get_x() - 20 + rng.get_random(40) as i32,
                position.get_y() - 15 + rng.get_random(30) as i32,
            );

            if let Some(tile) = map.get_tile_at(&radiation_position) {
                let t = tile.get_raw();
                if t & TILE_ZONE_BIT != 0x00 {
                    continue;
                }

                if t & TILE_BURN_BIT != 0x00 || t == TileType::Dirt.to_u16().unwrap() {
                    map.set_tile_at(
                        &radiation_position,
                        Tile::from_type(TileType::Radioactive).unwrap(),
                    );
                }
            }
        }

        // report disaster to the user
        // TODO: sendMessage(MESSAGE_NUCLEAR_MELTDOWN, position, true, true)

        Ok(())
    }

    /// Let a fire bomb explode at a random location.
    fn fire_bomb(rng: &mut MicropolisRandom, map: &TileMap) -> Result<(), String> {
        let crash_position = MapPosition::new(
            rng.get_random((WORLD_WIDTH - 1) as i16) as i32,
            rng.get_random((WORLD_HEIGHT - 1) as i16) as i32,
        );
        Self::make_explosion(map, &crash_position)?;
        // TODO: sendMessage(MESSAGE_FIREBOMBING, crash_position, true, true)
        Ok(())
    }

    /// Throw several bombs onto the city.
    fn make_fire_bombs(rng: &mut MicropolisRandom, map: &TileMap) -> Result<(), String> {
        let mut count = 2 + (rng.get_random_16() & 0x01);
        while count > 0 {
            Self::fire_bomb(rng, map)?;
            count -= 1;
        }

        // TODO: schedule periodic fire bombs over time, every few ticks
        Ok(())
    }

    /// Tell the front-end to show an earthquake to the user
    /// (shaking the map for some time).
    fn do_earthquake(strength: i16) {
        // TODO: makeSound("city", "ExplosionLow")
        // TODO: callback("startEarthquake", "d", strength)
    }

    /// Change random tiles to fire or dirt as a result of the earthquake.
    fn make_earthquake(rng: &mut MicropolisRandom, map: &mut TileMap) -> Result<(), String> {
        let strength = rng.get_random(700) + 300;
        Self::do_earthquake(strength);
        // TODO: sendMessage(MESSAGE_EARTHQUAKE, city_center, true)
        for z in 0..strength {
            let position = MapPosition::new(
                rng.get_random((WORLD_WIDTH - 1) as i16) as i32,
                rng.get_random((WORLD_HEIGHT - 1) as i16) as i32,
            );
            if let Some(tile) = map.get_tile_at(&position) {
                if tile.is_vulnerable() {
                    map.set_tile_at(
                        &position,
                        Tile::from_raw(if z & 0x03 != 0x00 {
                            // 3 out of 4 times: reduce the tile to rubble
                            Self::random_rubble(rng)
                        } else {
                            // 1 out of 4 times: start a fire
                            Self::random_fire(rng)
                        })?,
                    );
                }
            }
        }

        Ok(())
    }

    /// Start a fire at a random place, random disaster or scenario.
    fn set_fire(rng: &mut MicropolisRandom, map: &mut TileMap) -> Result<(), String> {
        let at = MapPosition::new_random(rng, &map.bounds());
        if let Some(tile) = map.get_tile_at(&at) {
            let mut z = tile.get_raw() & TILE_ZONE_BIT;
            if z != 0x00 {
                return Ok(());
            }
            z = z & TILE_LOW_MASK;
            if z > TileType::House.to_u16().unwrap() && z < TileType::LastZone.to_u16().unwrap() {
                map.set_tile_at(&at, Tile::from_raw(Self::random_fire(rng))?);
                // TODO: sendMessage(MESSAGE_FIRE_REPORTED, at, true)
            }
        }
        Ok(())
    }

    /// Start a fire at a random place, requested by the user.
    fn make_fire(rng: &mut MicropolisRandom, map: &mut TileMap) -> Result<(), String> {
        for t in 0..40 {
            let at = MapPosition::new_random(rng, &map.bounds());
            if let Some(tile) = map.get_tile_at(&at) {
                let mut z = tile.get_raw();
                if z & TILE_ZONE_BIT == 0x00 || z & TILE_BURN_BIT == 0x00 {
                    return Ok(());
                }
                z = z & TILE_LOW_MASK;
                if z > 21 && z < TileType::LastZone.to_u16().unwrap() {
                    map.set_tile_at(&at, Tile::from_raw(Self::random_fire(rng))?);
                    // TODO: sendMessage(MESSAGE_FIRE_REPORTED, at, true)
                    return Ok(());
                }
            }
        }
        Ok(())
    }

    /// Flood many tiles.
    /// TODO: use direction and some form of XYPosition class here
    fn make_flood(&mut self, rng: &mut MicropolisRandom, map: &mut TileMap) -> Result<(), String> {
        for z in 0..300 {
            let at = MapPosition::new_random(rng, &map.bounds());
            let mut c = map
                .get_tile_at(&at)
                .ok_or(format!(
                    "CityDisasters::make_flood cannot get tile at {}",
                    at
                ))
                .map(|t| t.get_raw() & TILE_LOW_MASK)?;
            if c <= TileType::Channel.to_u16().unwrap()
                || c > TileType::LastRiverEdge.to_u16().unwrap()
            {
                continue;
            }
            for t in 0..4 {
                let current_position = at + (FLOOD_DX[t], FLOOD_DY[t]).into();
                if let Some(tile) = map.get_tile_at(&current_position) {
                    if tile.is_floodable() {
                        map.set_tile_at(&current_position, Tile::from_type(TileType::Flood)?);
                        self.flood_count = 30;
                        // TODO: sendMessage(MESSAGE_FLOODING_REPORTED, current_position, true)
                        return Ok(());
                    }
                }
            }
        }
        Ok(())
    }

    /// Flood around the given position.
    fn do_flood(
        &mut self,
        rng: &mut MicropolisRandom,
        map: &mut TileMap,
        at: &MapPosition,
    ) -> Result<(), String> {
        if self.flood_count > 0 {
            for z in 0..4 {
                if rng.get_random_16() & 0x07 != 0x00 {
                    continue;
                }
                // 12.5% chance
                let current_position = *at + (FLOOD_DX[z], FLOOD_DY[z]).into();
                if let Some(tile) = map.get_tile_at(&current_position) {
                    let c = tile.get_raw();
                    let t = c & TILE_LOW_MASK;
                    if c & TILE_BURN_BIT == TILE_BURN_BIT
                        || c == TileType::Dirt.to_u16().unwrap()
                        || (t >= TileType::Woods5.to_u16().unwrap()
                            && t < TileType::Flood.to_u16().unwrap())
                    {
                        if c & TILE_ZONE_BIT == TILE_ZONE_BIT {}
                        map.set_tile_at(
                            &current_position,
                            Tile::from_raw(
                                TileType::Flood.to_u16().unwrap() + rng.get_random(2) as u16,
                            )?,
                        );
                    }
                }
            }
        } else {
            // if random_16(rng) & 0x0F == 0x00 {
            //     // 1/16 chance
            //     map.set_tile_at(position, Tile::from_type(TileType::Dirt)?);
            // }
        }

        Ok(())
    }

    /// Construct an explosion sprite at the given position.
    fn make_explosion(map: &TileMap, at: &MapPosition) -> Result<(), String> {
        todo!()
        // if !map.in_bounds(at) {
        //     return;
        // }
        // Self::make_explosion_at(&MapPosition::new(
        //     (at.get_x() << 4) + 8,
        //     (at.get_y() << 4) + 8,
        // ))
    }

    /// Construct an explosion sprite.
    pub fn make_explosion_at(
        rng: &mut MicropolisRandom,
        sprites: &mut ActiveSpritesList,
        at: &MapPosition,
        max_pollution_at: &MapPosition,
    ) {
        sprites.add_sprite(Sprite::new(
            rng,
            "".into(),
            &SpriteType::Explosion,
            *at - (40, 16).into(),
            &max_pollution_at,
        ))
    }

    fn random_fire(rng: &mut MicropolisRandom) -> u16 {
        TileType::Fire.to_u16().unwrap() + ((rng.get_random_16() & 0x07) as u16 | TILE_ANIM_BIT)
    }

    fn random_rubble(rng: &mut MicropolisRandom) -> u16 {
        TileType::Rubble.to_u16().unwrap() + ((rng.get_random_16() & 0x03) as u16 | TILE_BULL_BIT)
    }
}
