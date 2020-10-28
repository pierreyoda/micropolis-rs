use parameters::SimulationParameters;
use rand::Rng;

use super::{disasters::CityDisasters, power::CityPower, City};
use crate::{
    game::GameLevelDifficulty,
    map::{
        tiles::TILE_ANIM_BIT,
        tiles::TILE_BULL_BIT,
        tiles::TILE_BURN_BIT,
        tiles::TILE_LOW_MASK,
        tiles::{TILE_CONDUCT_BIT, TILE_POWER_BIT, TILE_ZONE_BIT},
        Map, MapClusteringStrategy, MapPosition, TileMap, TileType,
    },
    utils::random_in_range,
};
use crate::{
    game::{GameSpeed, GameSpeedPreset},
    map::Tile,
    utils::clamp,
};

pub mod parameters;

const ZONE_MELTDOWN_TABLE: [i16; 3] = [30000, 20000, 10000];
const SMOKE_TILES: [u16; 4] = [
    916, // TileType::CoalSmoke1.to_u16().unwrap(),
    920, // TileType::CoalSmoke2.to_u16().unwrap(),
    924, // TileType::CoalSmoke3.to_u16().unwrap(),
    928, // TileType::CoalSmoke4.to_u16().unwrap(),
];
const SMOKE_DX: [i32; 4] = [1, 2, 1, 2];
const SMOKE_DY: [i32; 4] = [-1, -1, 0, 0];

pub struct Simulation {
    parameters: SimulationParameters,
    speed: GameSpeed,
    speed_cycle: u16,
    phase_cycle: u8,
    /// Number of passes through the similator loop.
    passes: u32,
    /// Current simulator loop pass counter.
    pass_index: usize,
    /// Incremented every time the map changes.
    map_serial: u32,
    /// City time unit counter, increnented once every 16 runs through
    /// the simulator (at fast speed). A time unit is 7.6 days. 4 units
    /// per month, 48 units per year, relative to #startingYear
    ///
    /// Four units per month, so one unit is about a week (7.6 days).
    city_time: u64,
    /// Rate of growth map.
    ///
    /// Affected by DecROGMem, incROG called by zones. Decreased by fire
    /// explosions from sprites, fire spreading. Doesn't seem to
    /// actually feed back into the simulation. Output only.
    rate_of_growth: Map<i16>,
}

impl Simulation {
    pub fn new(map: &TileMap) -> Self {
        let dimensions = map.bounds();
        Self {
            parameters: Default::default(),
            speed: GameSpeed::from(GameSpeedPreset::Normal),
            speed_cycle: 0,
            phase_cycle: 0,
            passes: 0,
            pass_index: 0,
            map_serial: 1,
            city_time: 0,
            rate_of_growth: Map::with_data(
                vec![vec![0x00; dimensions.get_height()]; dimensions.get_width()],
                MapClusteringStrategy::BlockSize8,
            ),
        }
    }
    pub fn reset_pass_counter(&mut self) {
        self.pass_index = 0;
    }

    pub fn on_map_updated(&mut self) {
        self.map_serial += 1;
    }

    /// Advance the city simulation and its visualization by one frame tick.
    pub fn step(&mut self, city: &mut City) -> Result<(), String> {
        let sim_steps_per_update = self.speed.get_sim_steps_per_update();
        if sim_steps_per_update == 0 {
            return Ok(());
        }
        self.speed_cycle += 1;
        if self.speed_cycle == 1024 {
            self.speed_cycle = 0;
        }
        match sim_steps_per_update {
            1 if self.speed_cycle % 5 != 0 => return Ok(()),
            2 if self.speed_cycle % 3 != 0 => return Ok(()),
            _ => {}
        }

        self.simulate(city)
    }

    fn simulate(&mut self, city: &mut City) -> Result<(), String> {
        // The simulator has 16 different phases, which we cycle through
        // according to `phase_cycle`, which is incremented and wrapped at the
        // end of this switch.

        let map_size = city.map.bounds();

        // TODO: initSimLoad behavior
        self.phase_cycle &= 15;
        // TODO: replace phase_cycle integer by enum?
        match self.phase_cycle {
            0 => {}
            // Scan 1/8th of the map for each of these 8 phases
            1..=8 => {
                let phase_cycle = self.phase_cycle as usize;
                self.scan_map_section(
                    city,
                    (phase_cycle - 1) * map_size.get_width() / 8,
                    phase_cycle * map_size.get_height() / 8,
                )?
            }
            9 => {}
            10 => {}
            11 => {}
            12 => {}
            13 => {}
            14 => {}
            15 => {}
            _ => unreachable!(),
        };
        self.phase_cycle = (self.phase_cycle + 1) & 15;

        Ok(())
    }

    fn scan_map_section(&self, city: &mut City, x1: usize, x2: usize) -> Result<(), String> {
        let map_height = city.map.bounds().get_height();
        let flood_type_raw = TileType::Flood
            .to_u16()
            .ok_or("Flood tile type raw conversion error")?;
        for x in x1..x2 {
            for y in 0..map_height {
                let tile = city
                    .map
                    .tiles()
                    .get(x)
                    .ok_or(format!(
                        "Simulation.scan_map_section map X overflow at {}",
                        x
                    ))?
                    .get(y)
                    .ok_or(format!(
                        "Simulation.scan_map_section map Y overflow at {}",
                        y
                    ))?;
                if tile.get_type() == &Some(TileType::Dirt) {
                    continue;
                }

                let tile_type_raw = tile.get_type_raw();
                if tile_type_raw < flood_type_raw {
                    continue;
                }

                let position = MapPosition::new(x as i32, y as i32);
                if tile_type_raw < TileType::HorizontalBridge as u16
                    && tile_type_raw >= TileType::Fire as u16
                {
                    city.fires_count += 1;
                }
            }
        }

        Ok(())
    }

    /// Handle a zone on fire.
    ///
    /// Decreases rate of growth of the zone, and makes remaining tiles bulldozable.
    fn fire_zone(
        &mut self,
        position: &MapPosition,
        tile: &Tile,
        map: &mut TileMap,
    ) -> Result<Tile, String> {
        let mut value = *self.rate_of_growth.get_tile_at(&position).ok_or(format!(
            "Simulation.fire_zone: cannot get rate of growth value at {}",
            position
        ))? as i16;
        value = clamp(value - 20, -200, 200);
        self.rate_of_growth.set_tile_at(&position, value);

        let tile_raw = (tile.get_raw() & TILE_LOW_MASK) as i16;
        let xy_max = if tile_raw < TileType::PortBase.to_i16().unwrap() {
            2
        } else if tile_raw == TileType::Airport.to_i16().unwrap() {
            5
        } else {
            4
        };

        // make remaining tiles of the zone bulldozable
        for x in -1..xy_max {
            for y in -1..xy_max {
                let current_position = *position + (x, y).into();
                if let Some(current_tile) = map.get_tile_at(&current_position) {
                    let current_tile_raw = current_tile.get_raw();
                    if current_tile_raw & TILE_LOW_MASK
                        >= TileType::HorizontalBridge.to_u16().unwrap()
                    {
                        // post release
                        map.set_tile_at(
                            &current_position,
                            Tile::from_raw(current_tile_raw | TILE_BULL_BIT)?,
                        );
                    }
                }
            }
        }

        Ok(Tile::from_raw(tile_raw as u16)?)
    }

    /// Repair the zone at the given position.
    fn repair_zone(
        &self,
        map: &mut TileMap,
        at: &MapPosition,
        zone_size: u16,
    ) -> Result<(), String> {
        let mut tile_raw = zone_size - 2 - zone_size;

        // y and x loops one position shifted to compensate for the center-tile position.
        for y in -1..(zone_size as i32) - 1 {
            for x in -1..(zone_size as i32) - 1 {
                let zone_position = *at + (x, y).into();
                tile_raw += 1;

                if let Some(zone_tile) = map.get_tile_at(&zone_position) {
                    let mut zone_tile_raw = zone_tile.get_raw();
                    if zone_tile_raw & TILE_ZONE_BIT != 0x00
                        || zone_tile_raw & TILE_ANIM_BIT != 0x00
                    {
                        continue;
                    }
                    zone_tile_raw &= TILE_LOW_MASK;
                    if zone_tile_raw < TileType::Rubble.to_u16().unwrap()
                        || zone_tile_raw >= TileType::HorizontalBridge.to_u16().unwrap()
                    {
                        map.set_tile_at(
                            &zone_position,
                            Tile::from_raw(tile_raw | TILE_CONDUCT_BIT | TILE_BURN_BIT)?,
                        );
                    }
                }
            }
        }

        Ok(())
    }

    /// Update special zones.
    fn do_special_zone<R: Rng>(
        &mut self,
        rng: &mut R,
        map: &mut TileMap,
        power: &mut CityPower,
        at: &MapPosition,
        is_zone_powered: bool,
        disasters_enabled: bool,
        difficulty: &GameLevelDifficulty,
    ) -> Result<(), String> {
        let tile_type = map
            .get_tile_at(at)
            .map(|t| t.get_raw() & TILE_LOW_MASK)
            .map(|r| {
                TileType::from_u16(r)
                    .ok_or(format!("Simulation::do_special_zone invalid tile type"))
            })
            .ok_or(format!("Simulation::do_special_zone cannot read tile"))??;
        match tile_type {
            TileType::PowerPlant => {
                power.coal_generators_count += 1;
                if self.city_time & 0x07 == 0x00 {
                    self.repair_zone(map, at, 4)?;
                }
                power.push_power_stack(*at);
                self.coal_smoke(map, at)?
            }
            TileType::Nuclear => {
                // if disasters_enabled
                //     && random_in_range(rng, 0, ZONE_MELTDOWN_TABLE[difficulty.to_usize().unwrap()])
                // {
                //     CityDisasters::do_meltdown(rng, map, at)?;
                // }
            }
            _ => todo!(),
        }

        Ok(())
    }

    /// Draw coal smoke tiles around the given coal power plant position.
    fn coal_smoke(&self, map: &mut TileMap, at: &MapPosition) -> Result<(), String> {
        for i in 0..4 {
            map.set_tile_at(
                &(*at + (SMOKE_DX[i], SMOKE_DY[i]).into()),
                Tile::from_raw(
                    SMOKE_TILES[i]
                        | TILE_ANIM_BIT
                        | TILE_CONDUCT_BIT
                        | TILE_POWER_BIT
                        | TILE_BURN_BIT,
                )?,
            );
        }
        Ok(())
    }
}
