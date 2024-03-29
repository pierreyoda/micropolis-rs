use std::todo;

use parameters::SimulationParameters;
use statistics::SimulationStatistics;
use taxes::SimulationTaxes;

use self::{
    census::CitySimulationCensus,
    parameters::MAX_ROAD_EFFECT,
    scan::CitySimulationScanner,
    sprites::{generate_copter, generate_plane, generate_ship, generate_train},
};

use super::{
    disasters::CityDisasters,
    power::CityPower,
    sprite::{ActiveSpritesList, SpriteType},
    traffic::CityTraffic,
    City,
};
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
    utils::random::MicropolisRandom,
};
use crate::{
    game::{GameSpeed, GameSpeedPreset},
    map::Tile,
    utils::clamp,
};

mod census;
pub mod parameters;
mod scan;
pub mod sprites;
pub mod statistics;
pub mod taxes;
mod zones;

const ZONE_MELTDOWN_TABLE: [i16; 3] = [30000, 20000, 10000];
const SMOKE_TILES: [u16; 4] = [
    916, // TileType::CoalSmoke1.to_u16().unwrap(),
    920, // TileType::CoalSmoke2.to_u16().unwrap(),
    924, // TileType::CoalSmoke3.to_u16().unwrap(),
    928, // TileType::CoalSmoke4.to_u16().unwrap(),
];
const SMOKE_DX: [i32; 4] = [1, 2, 1, 2];
const SMOKE_DY: [i32; 4] = [-1, -1, 0, 0];
const FIRE_DX: [i32; 4] = [-1, 0, 1, 0];
const FIRE_DY: [i32; 4] = [0, -1, 0, 1];

const CENSUS_MONTHLY_FREQUENCY: u32 = 4;
const CENSUS_YEARLY_FREQUENCY: u32 = CENSUS_MONTHLY_FREQUENCY * 12;
const TAX_FREQUENCY: u32 = 48;

const SPEED_POWER_SCAN: [u16; 3] = [2, 4, 5];
const SPEED_POLLUTION_TERRAIN_LAND_VALUE_SCAN: [u16; 3] = [2, 7, 17];
const SPEED_CRIME_SCAN: [u16; 3] = [1, 8, 18];
const SPEED_POPULATION_DENSITY_SCAN: [u16; 3] = [1, 9, 19];
const SPEED_FIRE_ANALYSIS: [u16; 3] = [1, 10, 20];

pub struct Simulation {
    scanner: CitySimulationScanner,
    parameters: SimulationParameters,
    statistics: SimulationStatistics,
    census: CitySimulationCensus,
    taxes: SimulationTaxes,
    speed: GameSpeed,
    speed_cycle: u16,
    phase_cycle: u8,
    simulation_cycle: u16,
    do_initial_evaluation: bool,
    new_power: bool,
    /// Number of passes through the simulator loop.
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
    /// Coordinates of the city center.
    city_center: MapPosition,
    /// Pollution density map.
    pollution_density: Map<u8>,
    /// Land value map.
    land_value_map: Map<u8>,
    /// Crime rate map.
    crime_rate_map: Map<u8>,
    /// Terrain development density map.
    ///
    /// Used to calculate land value.
    terrain_density: Map<u8>,
    /// Rate of growth map.
    ///
    /// Affected by DecROGMem, incROG called by zones. Decreased by fire
    /// explosions from sprites, fire spreading. Doesn't seem to
    /// actually feed back into the simulation. Output only.
    rate_of_growth: Map<i16>,
    /// Fire station map.
    ///
    /// Effectivity of fire control in each area.
    /// Affected by fire stations (powered or not), fire funding ratio and road access.
    /// Affects how long fires burn.
    fire_station_map: Map<i16>,
    /// Copy of fire station map to display.
    fire_station_effect_map: Map<i16>,
    /// Police station map.
    ///
    /// Effectivity of police in fighting crime.
    /// Affected by police stations (powered or not), police funding ratio and road access.
    /// Affects crime rate.
    police_station_map: Map<i16>,
    /// Copy of police station map to display.
    police_station_effect_map: Map<i16>,
    /// Commercial rate map.
    ///
    /// Depends on distance to city center.
    /// Affects commercial zone evaluation.
    commercial_rate_map: Map<i16>,
}

impl Simulation {
    pub fn new(map: &TileMap) -> Self {
        let dimensions = map.bounds();
        let scanner = CitySimulationScanner::new(map);
        Self {
            scanner,
            parameters: Default::default(),
            statistics: Default::default(),
            census: CitySimulationCensus::new(),
            taxes: Default::default(),
            speed: GameSpeed::from(GameSpeedPreset::Normal),
            speed_cycle: 0,
            phase_cycle: 0,
            simulation_cycle: 0,
            do_initial_evaluation: true,
            new_power: false,
            passes: 0,
            pass_index: 0,
            map_serial: 1,
            city_time: 0,
            city_center: (0, 0).into(),
            pollution_density: Map::with_data(
                vec![vec![0x00; dimensions.get_height() / 2]; dimensions.get_width() / 2],
                MapClusteringStrategy::BlockSize2,
            ),
            land_value_map: Map::with_data(
                vec![vec![0x00; dimensions.get_height() / 2]; dimensions.get_width() / 2],
                MapClusteringStrategy::BlockSize2,
            ),
            crime_rate_map: Map::with_data(
                vec![vec![0x00; dimensions.get_height() / 2]; dimensions.get_width() / 2],
                MapClusteringStrategy::BlockSize2,
            ),
            terrain_density: Map::with_data(
                vec![vec![0x00; dimensions.get_height() / 4]; dimensions.get_width() / 4],
                MapClusteringStrategy::BlockSize4,
            ),
            rate_of_growth: Map::with_data(
                vec![vec![0x00; dimensions.get_height() / 8]; dimensions.get_width() / 8],
                MapClusteringStrategy::BlockSize8,
            ),
            fire_station_map: Map::with_data(
                vec![vec![0x00; dimensions.get_height() / 8]; dimensions.get_width() / 8],
                MapClusteringStrategy::BlockSize8,
            ),
            fire_station_effect_map: Map::with_data(
                vec![vec![0x00; dimensions.get_height() / 8]; dimensions.get_width() / 8],
                MapClusteringStrategy::BlockSize8,
            ),
            police_station_map: Map::with_data(
                vec![vec![0x00; dimensions.get_height() / 8]; dimensions.get_width() / 8],
                MapClusteringStrategy::BlockSize8,
            ),
            police_station_effect_map: Map::with_data(
                vec![vec![0x00; dimensions.get_height() / 8]; dimensions.get_width() / 8],
                MapClusteringStrategy::BlockSize8,
            ),
            commercial_rate_map: Map::with_data(
                vec![vec![0x00; dimensions.get_height() / 8]; dimensions.get_width() / 8],
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
    pub fn step(&mut self, rng: &mut MicropolisRandom, city: &mut City) -> Result<(), String> {
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

        self.simulate(rng, city)
    }

    fn simulate(&mut self, rng: &mut MicropolisRandom, city: &mut City) -> Result<(), String> {
        // The simulator has 16 different phases, which we cycle through
        // according to `phase_cycle`, which is incremented and wrapped at the
        // end of this switch.

        let map_size = city.map.bounds();
        let speed_index = clamp(city.simulation_speed - 1, 0, 2) as usize;

        // TODO: initSimLoad behavior
        self.phase_cycle &= 15;
        // TODO: replace phase_cycle integer by enum?
        match self.phase_cycle {
            0 => {
                self.simulation_cycle += 1;
                if self.simulation_cycle > 1023 {
                    self.simulation_cycle = 0;
                }

                if self.do_initial_evaluation {
                    self.do_initial_evaluation = false;
                    city.evaluate()?;
                }

                self.city_time += 1;
                self.taxes.city_tax_average += self.taxes.city_tax;

                if self.simulation_cycle & 0x01 == 0x00 {
                    self.compute_valves();
                }

                self.clear_census();
            }
            // Scan 1/8th of the map for each of these 8 phases
            1..=8 => {
                let phase_cycle = self.phase_cycle as usize;
                self.scan_map_section(
                    city,
                    (phase_cycle - 1) * map_size.get_width() / 8,
                    phase_cycle * map_size.get_height() / 8,
                )?
            }
            9 => {
                if city.city_time % CENSUS_MONTHLY_FREQUENCY == 0 {
                    self.census.take_monthly_census(
                        &city.population,
                        &mut self.statistics,
                        city.cash_flow,
                    );
                }
                if city.city_time % CENSUS_YEARLY_FREQUENCY == 0 {
                    self.census.take_yearly_census(&city.population);
                }
                if city.city_time % TAX_FREQUENCY == 0 {
                    self.taxes
                        .collect_taxes(&mut city.cash_flow, &self.statistics);
                }
            }
            10 => {
                if self.simulation_cycle % 5 != 0 {
                    self.decrease_rate_of_growth_map();
                }
                self.decrease_traffic_map(&city.map, &mut city.traffic);

                // TODO: sendMessages()
            }
            11 => {
                if (self.simulation_cycle % SPEED_POWER_SCAN[speed_index]) == 0 {
                    city.power.do_power_scan(&city.map);
                    self.new_power = true;
                }
            }
            12 => {
                if (self.simulation_cycle % SPEED_POLLUTION_TERRAIN_LAND_VALUE_SCAN[speed_index])
                    == 0
                {
                    let (pollution_average, pollution_max_at, land_value_average) =
                        self.scanner.pollution_terrain_land_value_scan(
                            rng,
                            &city.map,
                            &self.city_center,
                            &self.statistics.maximum_pollution_at,
                            &self.crime_rate_map,
                            &mut self.terrain_density,
                            &mut self.land_value_map,
                            &mut self.pollution_density,
                        );
                    self.statistics.average_pollution = pollution_average;
                    self.statistics.maximum_pollution_at = pollution_max_at;
                    self.statistics.average_land_value = land_value_average;
                }
            }
            13 => {
                if (self.simulation_cycle % SPEED_CRIME_SCAN[speed_index]) == 0 {
                    let (crime_average, crime_maximum_at, police_station_effect_map) =
                        self.scanner.crime_scan(
                            rng,
                            &city.map,
                            &self.land_value_map,
                            &mut self.police_station_map,
                            &mut self.crime_rate_map,
                            city.population.get_density_map(),
                            &self.statistics.maximum_crime_at,
                        );
                    self.statistics.average_crime = crime_average;
                    self.statistics.maximum_crime_at = crime_maximum_at;
                    self.police_station_effect_map = police_station_effect_map;
                }
            }
            14 => {
                if (self.simulation_cycle % SPEED_POPULATION_DENSITY_SCAN[speed_index]) == 0 {
                    self.city_center = self.scanner.population_density_scan(
                        &city.map,
                        city.population.get_density_map_mut(),
                        &mut self.commercial_rate_map,
                        &self.city_center,
                    )?;
                }
            }
            15 => {
                if (self.simulation_cycle % SPEED_FIRE_ANALYSIS[speed_index]) == 0 {
                    self.fire_station_effect_map =
                        self.scanner.fire_analysis(&mut self.fire_station_map);
                }

                city.disasters.do_disasters(
                    rng,
                    &mut city.map,
                    &mut city.sprites,
                    &city.difficulty,
                    &city.scenario,
                    self.statistics.average_pollution,
                    &self.statistics.maximum_pollution_at,
                )?;
            }
            _ => unreachable!(),
        };
        self.phase_cycle = (self.phase_cycle + 1) & 15;

        Ok(())
    }

    /// Compute the RCI valves, standing for Residential, Commercial and Industrial zone demands.
    fn compute_valves(&mut self) {
        todo!()
    }

    fn clear_census(&mut self) {
        todo!()
    }

    /// Decrease rate of growth.
    ///
    /// TODO: Limiting rate should not be done here, but when we add a new value to it.
    fn decrease_rate_of_growth_map(&mut self) {
        let bounds = self.rate_of_growth.bounds();
        for x in 0..bounds.get_width() {
            for y in 0..bounds.get_height() {
                let position: MapPosition = (x, y).into();
                let z = self.rate_of_growth.get_tile_mut_at(&position).expect(
                    "Simulation.decrease_rate_of_growth_map get tile should not be out of bounds",
                );

                if *z > 0 {
                    *z -= 1;
                    *z = clamp(*z, -200, 200);
                } else if *z < 0 {
                    *z += 1;
                    *z = clamp(*z, -200, 200);
                }
            }
        }
    }

    // Decrease traffic memory.
    fn decrease_traffic_map(&mut self, map: &TileMap, traffic: &mut CityTraffic) {
        let map_bounds = map.bounds();
        let traffic_density_map = traffic.get_density_map_mut();
        let bounds = traffic_density_map.bounds();
        let traffic_density_blocksize = traffic_density_map.get_clustering_strategy().block_size();

        for x in (0..map_bounds.get_width()).step_by(traffic_density_blocksize) {
            for y in (0..map_bounds.get_height()).step_by(traffic_density_blocksize) {
                let position: MapPosition = (x, y).into();
                let z = traffic_density_map
                    .get_tile_mut_at(&position)
                    .expect("Simulation.decrease_traffic_map get tile should not be out of bounds");
                if *z == 0 {
                    continue;
                }

                *z -= if *z <= 24 {
                    0
                } else if *z > 200 {
                    34
                } else {
                    24
                }
            }
        }
    }

    fn scan_map_section(&mut self, city: &mut City, x1: usize, x2: usize) -> Result<(), String> {
        let rng = &mut city.rng;
        let sprites = &mut city.sprites;

        let map = &mut city.map;
        let map_height = map.bounds().get_height();

        let flood_type_raw = TileType::Flood
            .to_u16()
            .ok_or("Flood tile type raw conversion error")?;

        for x in x1..x2 {
            for y in 0..map_height {
                let position: MapPosition = (x, y).into();
                let tile = map.get_tile_at(&position).expect(
                    &format!(
                        "Simulation.scan_map_section cannot get tile at {}",
                        position
                    )[..],
                );

                let tile_type = tile.get_type().as_ref().ok_or(format!(
                    "Simulation.scan_map_section invalid tile {:?}",
                    tile
                ))?;
                if *tile_type == TileType::Dirt {
                    continue;
                }

                if *tile_type < TileType::Flood {
                    continue;
                }

                if *tile_type < TileType::HorizontalBridge {
                    if *tile_type >= TileType::Fire {
                        self.statistics.fire_station_count += 1;
                        if rng.get_random_16() & 0x03 == 0x00 {
                            // 1 in 4 times
                            self.do_fire(rng, map, sprites, &position)?;
                        }
                        continue;
                    }

                    if *tile_type < TileType::Radioactive {
                        city.disasters.do_flood(rng, map, &position)?;
                    } else {
                        self.do_radioactive_tile(rng, map, &position)?;
                    }
                    continue;
                }

                // TODO:
            }
        }

        Ok(())
    }

    /// Repair the zone at the given position.
    ///
    /// - at: center-tile position of the zone
    /// - zone_center: type of the center-tile
    fn repair_zone(
        &self,
        map: &mut TileMap,
        at: &MapPosition,
        zone_center: TileType,
        zone_size: u16,
    ) -> Result<(), String> {
        let mut tile_raw = zone_center.to_u16().unwrap() - 2 - zone_size;

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
    fn do_special_zone(
        &mut self,
        rng: &mut MicropolisRandom,
        map: &mut TileMap,
        power: &mut CityPower,
        sprites: &mut ActiveSpritesList,
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
                    .ok_or("Simulation::do_special_zone invalid tile type".to_string())
            })
            .ok_or("Simulation::do_special_zone cannot read tile".to_string())??;
        Ok(match tile_type {
            TileType::PowerPlant => {
                // coal power generation
                power.coal_generators_count += 1;
                if self.city_time & 0x07 == 0x00 {
                    self.repair_zone(map, at, TileType::PowerPlant, 4)?;
                }
                power.push_power_stack(*at);
                Self::coal_smoke(map, at)?;
            }
            TileType::Nuclear => {
                // trigger nuclear meltdown?
                if disasters_enabled
                    && (rng.get_random(ZONE_MELTDOWN_TABLE[difficulty.to_usize().unwrap()]) == 0x00)
                {
                    CityDisasters::do_meltdown(rng, map, at)?;
                    return Ok(());
                }

                // otherwise, nuclear power generation
                power.nuclear_generators_count += 1;
                if self.city_time & 0x07 == 0x00 {
                    self.repair_zone(map, at, TileType::Nuclear, 4)?;
                }
                power.push_power_stack(*at);
            }
            TileType::FireStation => {
                self.statistics.fire_station_count += 1;
                if self.city_time & 0x07 == 0x00 {
                    self.repair_zone(map, at, TileType::FireStation, 3)?;
                }

                let mut z = (self.parameters.get_fire_effect()
                    / if is_zone_powered {
                        1 // powered effect
                    } else {
                        2 // otherwise: from the funding ratio
                    }) as i16;

                let (found_road, road_position) = CityTraffic::find_perimeter_road(map, at)?;
                if !found_road {
                    z /= 2;
                }

                let fire_control =
                    self.fire_station_map
                        .get_tile_at(&road_position)
                        .ok_or(format!(
                        "Simulation.do_special_zone cannot get fire_station_map tile value at {}",
                        road_position
                    ))? + z;
                self.fire_station_map
                    .set_tile_at(&road_position, fire_control);
            }
            TileType::PoliceStation => {
                self.statistics.police_station_count += 1;
                if self.city_time & 0x07 == 0x00 {
                    self.repair_zone(map, at, TileType::PoliceStation, 3)?;
                }

                let mut z = (self.parameters.get_police_effect()
                    / if is_zone_powered {
                        1 // powered effect
                    } else {
                        2 // otherwise: from the funding ratio
                    }) as i16;

                let (found_road, road_position) = CityTraffic::find_perimeter_road(map, at)?;
                if !found_road {
                    z /= 2;
                }

                let police_efficiency =
                    self.police_station_map
                        .get_tile_at(&road_position)
                        .ok_or(format!(
                        "Simulation.do_special_zone cannot get police_station_map tile value at {}",
                        road_position
                    ))? + z;
                self.police_station_map
                    .set_tile_at(&road_position, police_efficiency);
            }
            // Empty stadium
            TileType::Stadium => {
                self.statistics.stadium_count += 1;

                if self.city_time & 0x0F == 0x00 {
                    self.repair_zone(map, at, TileType::Stadium, 4)?;
                }

                if is_zone_powered {
                    // start a match every now and then
                    if (self.city_time as i32 + at.get_x() + at.get_y()) & 0x1F == 0x00 {
                        Self::draw_stadium(map, at, TileType::FullStadium)?;
                        map.set_tile_at(
                            &(*at + (1, 0).into()),
                            Tile::from_raw(
                                TileType::FootballGame1.to_u16().unwrap() + TILE_ANIM_BIT,
                            )?,
                        );
                        map.set_tile_at(
                            &(*at + (1, 1).into()),
                            Tile::from_raw(
                                TileType::FootballGame2.to_u16().unwrap() + TILE_ANIM_BIT,
                            )?,
                        );
                    }
                }
            }
            // Full stadium
            TileType::FullStadium => {
                self.statistics.stadium_count += 1;

                if (self.city_time as i32 + at.get_x() + at.get_y()) & 0x07 == 0x00 {
                    // stop the match
                    Self::draw_stadium(map, at, TileType::Stadium)?;
                }
            }
            TileType::Airport => {
                self.statistics.airport_count += 1;

                if self.city_time & 0x07 == 0x00 {
                    self.repair_zone(map, at, TileType::Airport, 6)?;

                    // display a rotating radar if powered
                    let radar_position = *at + (1, -1).into();
                    let radar_tile = map.get_tile_mut_at(&radar_position).ok_or(format!(""))?;
                    if is_zone_powered {
                        if radar_tile.get_raw() & TILE_LOW_MASK == TileType::Radar.to_u16().unwrap()
                        {
                            radar_tile.set_raw(
                                TileType::HBRDG_END.to_u16().unwrap()
                                    + TILE_ANIM_BIT
                                    + TILE_CONDUCT_BIT
                                    + TILE_BURN_BIT,
                            );
                        }
                    } else {
                        radar_tile.set_raw(
                            TileType::Radar.to_u16().unwrap() + TILE_CONDUCT_BIT + TILE_BURN_BIT,
                        );
                    }

                    // handle the airport activity if powered
                    if is_zone_powered {
                        Self::do_airport(rng, sprites, at)?;
                    }
                }
            }
            TileType::Port => {
                self.statistics.seaport_count += 1;

                if self.city_time & 0x15 == 0x00 {
                    self.repair_zone(map, at, TileType::Port, 4)?;
                }

                // if no ship and powered, generate a new one
                if is_zone_powered && sprites.get_sprite(&SpriteType::Ship).is_none() {
                    generate_ship(rng, sprites, map)?;
                }
            }
            _ => (),
        })
    }

    /// Draw coal smoke tiles around the given coal power plant position.
    fn coal_smoke(map: &mut TileMap, at: &MapPosition) -> Result<(), String> {
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

    /// Draw a stadium (either full or empty).
    fn draw_stadium(
        map: &mut TileMap,
        center: &MapPosition,
        base_value: TileType,
    ) -> Result<(), String> {
        debug_assert!(base_value.to_u16().unwrap() >= 5);

        // Center
        let tile = map.get_tile_mut_at(center).ok_or(format!(
            "Simulation::draw_stadium cannot access center tile at position {}",
            center,
        ))?;
        tile.set_raw(tile.get_raw() | TILE_ZONE_BIT | TILE_POWER_BIT);

        // Other tiles
        let mut value = base_value.to_u16().unwrap() - 5;
        let (center_x, center_y) = center.as_tuple();
        for y in center_y - 1..center_y + 3 {
            for x in center_x - 1..center_x + 3 {
                let at = (x, y).into();
                let tile = map.get_tile_mut_at(&at).ok_or(format!(
                    "Simulation::draw_stadium cannot access tile at position {}",
                    at,
                ))?;
                tile.set_raw(value | TILE_BURN_BIT | TILE_CONDUCT_BIT);
                value += 1;
            }
        }

        Ok(())
    }

    /// Handle a rail tracks.
    ///
    /// Generate a train, and handle road deteriorating effects.
    fn do_rail(
        &mut self,
        rng: &mut MicropolisRandom,
        map: &mut TileMap,
        sprites: &mut ActiveSpritesList,
        position: &MapPosition,
        total_population: u32,
    ) -> Result<(), String> {
        self.statistics.rail_total += 1;

        generate_train(rng, sprites, position, total_population)?;

        if self.parameters.get_road_effect() >= (15 * MAX_ROAD_EFFECT / 16) {
            return Ok(());
        }

        // rail deteriorates if not enough budget
        if rng.get_random_16() & 0x01FF != 0x00 {
            let tile = map.get_tile_mut_at(position).ok_or(format!(
                "Simulation.do_rail: cannot get tile at {}",
                position
            ))?;

            if tile.get_raw() & TILE_CONDUCT_BIT != 0x00 {
                debug_assert!(MAX_ROAD_EFFECT == 32); // otherwise the random16() & 31 makes no sense
                if self.parameters.get_road_effect() < (rng.get_random_16() as u64) & 31 {
                    let tile_value = tile.get_raw() & TILE_LOW_MASK;
                    tile.set_raw(
                        if tile_value < TileType::UnderwaterHorizontalRail.to_u16().unwrap() {
                            TileType::River.to_u16().unwrap()
                        } else {
                            Self::random_rubble_tile_value(rng)
                        },
                    )
                }
            }
        }

        Ok(())
    }

    /// Handle decay of a radioactive tile.
    fn do_radioactive_tile(
        &mut self,
        rng: &mut MicropolisRandom,
        map: &mut TileMap,
        position: &MapPosition,
    ) -> Result<(), String> {
        if rng.get_random_16() & 0x0FFF == 0x00 {
            map.set_tile_at(position, Tile::from_type(TileType::Dirt)?);
        }
        Ok(())
    }

    /// Handle a tile on fire at the given map position.
    ///
    /// TODO: needs a notion of iterative neighbour tiles computing
    /// TODO: use a getFromMap()-like function here
    /// TODO: extract constants of fire station effectiveness from here
    fn do_fire(
        &mut self,
        rng: &mut MicropolisRandom,
        map: &mut TileMap,
        sprites: &mut ActiveSpritesList,
        position: &MapPosition,
    ) -> Result<(), String> {
        // try to set neighbouring tiles on fire as well
        for z in 0..4 {
            if rng.get_random_16() & 0x07 == 0x00 {
                let position_temp = *position + (FIRE_DX[z], FIRE_DY[z]).into();

                if let Some(tile) = map.get_tile_at(&position_temp).cloned() {
                    let tile_raw = tile.get_raw();
                    if tile_raw & TILE_BURN_BIT == 0x00 {
                        continue; // not burnable
                    }

                    if tile_raw & TILE_ZONE_BIT != 0x00 {
                        // neighbour tile is a burnable zone
                        self.fire_zone(map, position, &tile)?;

                        // explode?
                        if tile_raw & TILE_LOW_MASK > TileType::IndustrialZoneBase.to_u16().unwrap()
                        {
                            let explosion_position: MapPosition =
                                position_temp * 16 + (8, 8).into();
                            CityDisasters::make_explosion_at(rng, sprites, &explosion_position)?;
                        }
                    }

                    map.set_tile_at(
                        &position_temp,
                        Tile::from_raw(Self::random_fire_tile_value(rng))?,
                    );
                }
            }
        }

        // compute likelihood of fire running out of fuel
        let z = self.fire_station_map.get_tile_at(position).ok_or(format!(
            "Simulation.do_fire: cannot get fire control efficience at {}",
            position
        ))?;
        let rate = match z {
            0..=19 => 3,
            20..=99 => 2,
            _ if *z > 100 => 1,
            _ => 10,
        };

        // should we put out the fire?
        if rng.get_random(rate) == 0x00 {
            map.get_tile_mut_at(position)
                .ok_or(format!(
                    "Simulation.do_fire: cannot get tile at {}",
                    position
                ))?
                .set_raw(Self::random_rubble_tile_value(rng));
        }

        Ok(())
    }

    /// Handle a zone on fire.
    ///
    /// Decreases rate of growth of the zone, and makes remaining tiles bulldozable.
    fn fire_zone(
        &mut self,
        map: &mut TileMap,
        position: &MapPosition,
        zone_tile: &Tile,
    ) -> Result<Tile, String> {
        let value = self
            .rate_of_growth
            .get_tile_mut_at(&position)
            .ok_or(format!(
                "Simulation.fire_zone: cannot get rate of growth value at {}",
                position
            ))?;
        *value = clamp(*value - 20, -200, 200);

        let tile_raw = (zone_tile.get_raw() & TILE_LOW_MASK) as i16;
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

    /// Generate an airplane or helicopter every now and then.
    fn do_airport(
        rng: &mut MicropolisRandom,
        sprites: &mut ActiveSpritesList,
        position: &MapPosition,
    ) -> Result<(), String> {
        if rng.get_random(5) == 0 {
            return generate_plane(rng, sprites, position);
        }
        if rng.get_random(12) == 0 {
            return generate_copter(rng, sprites, position);
        }

        Ok(())
    }

    /// Generate a random animated `TileType::Fire` tile.
    fn random_fire_tile_value(rng: &mut MicropolisRandom) -> u16 {
        (TileType::Fire.to_u16().unwrap() + (rng.get_random_16() as u16 & 0x07)) | TILE_ANIM_BIT
    }

    /// Generate a random `TileType::Rubble` tile.
    fn random_rubble_tile_value(rng: &mut MicropolisRandom) -> u16 {
        (TileType::Rubble.to_u16().unwrap() + (rng.get_random_16() as u16 & 0x03)) | TILE_BULL_BIT
    }
}
