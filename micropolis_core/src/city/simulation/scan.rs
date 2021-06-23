use std::cmp::min;

use num_traits::abs;

use crate::{
    map::{
        tiles::{TILE_LOW_MASK, TILE_ZONE_BIT},
        Map, MapClusteringStrategy, MapPosition, TileMap, TileType,
    },
    utils::{clamp, random::MicropolisRandom},
};

use super::zones::{
    count_free_population, get_commercial_zone_population, get_industrial_zone_population,
    get_residential_zone_population,
};

pub struct CitySimulationScanner {
    /// Temporary map 1.
    ///
    /// Used to smooth population density, pollution.
    temp_map_1: Map<u8>,
    /// Temporary map 2.
    ///
    /// Used to smooth population density, pollution.
    temp_map_2: Map<u8>,
    /// Temporary map 3.
    ///
    /// Used to smooth development density, for `terrain_density_map`.
    temp_map_3: Map<u8>,
    /// Integer with bits 0..2 that controls smoothing.
    ///
    /// TODO: variable is always 0, can we delete the variable?
    don_dither: i32,
}

impl CitySimulationScanner {
    pub fn new(map: &TileMap) -> Self {
        let dimensions = map.bounds();
        Self {
            temp_map_1: Map::with_data(
                vec![vec![0x00; dimensions.get_height() / 2]; dimensions.get_width() / 2],
                MapClusteringStrategy::BlockSize2,
            ),
            temp_map_2: Map::with_data(
                vec![vec![0x00; dimensions.get_height() / 2]; dimensions.get_width() / 2],
                MapClusteringStrategy::BlockSize2,
            ),
            temp_map_3: Map::with_data(
                vec![vec![0x00; dimensions.get_height() / 2]; dimensions.get_width() / 2],
                MapClusteringStrategy::BlockSize2,
            ),
            don_dither: 0,
        }
    }

    /// Performs the population density scan, and returns the new Center of Mass for the City.
    pub fn population_density_scan(
        &mut self,
        map: &TileMap,
        population_density_map: &mut Map<u8>,
        commercial_rate_map: &mut Map<i16>,
        current_city_center: &MapPosition,
    ) -> Result<MapPosition, String> {
        self.temp_map_1.clear(0x00);

        let (mut x_total, mut y_total, mut z_total) = (0, 0, 0);

        let bounds = map.bounds();
        for x in 0..bounds.get_width() {
            for y in 0..bounds.get_height() {
                let position: MapPosition = (x, y).into();
                if let Some(tile) = map.get_tile_at(&position) {
                    if tile.get_raw() & TILE_ZONE_BIT == 0x00 {
                        continue;
                    }
                    let tile_value = tile.get_raw() & TILE_LOW_MASK;
                    let population = min(
                        254,
                        8 * Self::get_population_density_at(map, &position, tile_value),
                    );
                    self.temp_map_1.set_tile_at(&position, population as u8);
                    x_total += x;
                    y_total += y;
                    z_total += 1;
                }
            }
        }

        self.do_smooth_1(); // temp_map_1 -> temp_map_2
        self.do_smooth_2(); // temp_map_2 -> temp_map_1
        self.do_smooth_1(); // temp_map_1 -> temp_map_2

        let destination_bounds = population_density_map.bounds();
        debug_assert!(destination_bounds == self.temp_map_2.bounds());

        // Copy temp_map_2 to population_density_map, multiplying by 2
        for x in 0..destination_bounds.get_width() {
            for y in 0..destination_bounds.get_height() {
                let position: MapPosition = (x, y).into();
                let value = self.temp_map_2.get_tile_at(&position).unwrap() * 2;
                population_density_map.set_tile_at(&position, value);
            }
        }

        Self::compute_commercial_rate_map(commercial_rate_map, current_city_center);

        // Compute the new city center
        let new_city_center: MapPosition = if z_total > 0 {
            // find Center of Mass for City
            (x_total / z_total, y_total / z_total).into()
        } else {
            // if population = 0, the center of the map is the city center
            (bounds.get_width() / 2, bounds.get_height() / 2).into()
        };

        Ok(new_city_center)
    }

    /// Get the population density of a zone center.
    fn get_population_density_at(map: &TileMap, position: &MapPosition, tile_value: u16) -> u16 {
        if tile_value == TileType::FreeZoneCenter.to_u16().unwrap() {
            count_free_population(map, position)
        } else if tile_value < TileType::CommercialBase.to_u16().unwrap() {
            get_residential_zone_population(tile_value)
        } else if tile_value < TileType::IndustrialBase.to_u16().unwrap() {
            get_commercial_zone_population(tile_value) * 8
        } else if tile_value < TileType::PortBase.to_u16().unwrap() {
            get_industrial_zone_population(tile_value) * 8
        } else {
            0
        }
    }

    /// Performs the pollution, terrain and land value scan.
    ///
    /// Returns the new average pollution, and the maximum pollution position.
    pub fn pollution_terrain_land_value_scan(
        &mut self,
        rng: &mut MicropolisRandom,
        map: &TileMap,
        current_city_center: &MapPosition,
        current_max_pollution_at: &MapPosition,
        terrain_density: &Map<u8>,
        crime_rate_map: &Map<u8>,
        land_value_map: &mut Map<u8>,
        pollution_density: &mut Map<u8>,
    ) -> (u16, MapPosition) {
        // temp_map_3 is a map of development density, smoothed into terrain_map
        self.temp_map_3.clear(0x00);

        let (mut land_value_total, mut land_value_num) = (0, 0);

        let land_value_bounds = land_value_map.bounds();
        for x in 0..land_value_bounds.get_width() {
            for y in 0..land_value_bounds.get_height() {
                let (mut pollution_level, mut land_value_flag) = (0, false);

                let position: MapPosition = (x, y).into();
                let world_position: MapPosition = (x * 2, y * 2).into();
                for m_x in world_position.get_x()..world_position.get_x() + 1 {
                    for m_y in world_position.get_y()..world_position.get_y() + 1 {
                        let tile_value = map
                            .get_tile_at(&(m_x, m_y).into())
                            .expect("CitySimulationScanner.pollution_terrain_land_value_scan should get tile at (m_x, m_y)")
                            .get_raw() & TILE_LOW_MASK;
                        if tile_value == 0 {
                            continue;
                        }
                        if tile_value < TileType::Rubble.to_u16().unwrap() {
                            // increment terrain memory
                            let value = *self.temp_map_3.get_tile_at(&(x >> 1, y >> 1).into())
                                .expect("CitySimulationScanner.pollution_terrain_land_value_scan should get value from temp_map_3 at (x >> 1, y >> 1)");
                            self.temp_map_3
                                .set_tile_at(&(x >> 1, y >> 1).into(), value + 15);
                            continue;
                        }
                        pollution_level += Self::get_pollution_value(tile_value);
                        if tile_value >= TileType::HorizontalBridge.to_u16().unwrap() {
                            land_value_flag = true;
                        }
                    }
                }

                pollution_level = min(pollution_level, 255);
                self.temp_map_1.set_tile_at(&(x, y).into(), pollution_level);

                if land_value_flag {
                    // land value equation
                    let mut distance = 34
                        - Self::get_distance_from_city_center(current_city_center, &world_position);
                    distance <<= 2;
                    distance += *terrain_density.get_tile_at(&(x >> 1, y >> 1).into())
                        .expect("CitySimulationScanner.pollution_terrain_land_value_scan should get value from terrain_density at (x >> 1, y >> 1)") as i32;
                    distance -= *pollution_density.get_tile_at(&position)
                        .expect("CitySimulationScanner.pollution_terrain_land_value_scan should get value from pollution_density at (x, y)") as i32;
                    if *crime_rate_map.get_tile_at(&position)
                        .expect("CitySimulationScanner.pollution_terrain_land_value_scan should get value from crime_rate_map at (x, y)") > 190 {
                        distance -= 20;
                    }
                    distance = clamp(distance, 1, 250);
                    land_value_map.set_tile_at(&position, distance as u8);
                    land_value_total += distance;
                    land_value_num += 1;
                } else {
                    land_value_map.set_tile_at(&position, 0);
                }
            }
        }

        let land_value_average = if land_value_num > 0 {
            (land_value_total / land_value_num) as i16
        } else {
            0
        };

        self.do_smooth_1(); // temp_map_1 -> temp_map_2
        self.do_smooth_2(); // temp_map_2 -> temp_map_1

        let (mut pollution_max, mut pollution_num, mut pollution_total) = (0, 0, 0);
        let mut pollution_max_at = *current_max_pollution_at;

        let world_bounds = map.bounds();
        let pollution_block_size = pollution_density.get_clustering_strategy().block_size();
        for x in (0..world_bounds.get_width()).step_by(pollution_block_size) {
            for y in (0..world_bounds.get_height()).step_by(pollution_block_size) {
                let position: MapPosition = (x, y).into();
                let z = *self.temp_map_1.get_tile_at(&position)
                    .expect("CitySimulationScanner.pollution_terrain_land_value_scan should get value from temp_map_1 at (x, y)");
                pollution_density.set_tile_at(&position, z);
                if z == 0 {
                    continue;
                }
                // get pollution average
                pollution_num += 1;
                pollution_total += z;
                // find the maximum pollution position for the monster
                if z > pollution_max || (z == pollution_max && (rng.get_random_16() & 0x03) == 0x00)
                {
                    pollution_max = z;
                    pollution_max_at = (x, y).into();
                }
            }
        }

        let pollution_average = if pollution_num != 0 {
            (pollution_total / pollution_num) as u16
        } else {
            0
        };

        self.smooth_terrain();

        (pollution_total, pollution_max_at)
    }

    /// Returns the pollution value of a tile.
    ///
    /// Returns the value of the polution (0..255, bigger is worse).
    fn get_pollution_value(tile_value: u16) -> u8 {
        if tile_value < TileType::HorizontalPower.to_u16().unwrap() {
            if tile_value >= TileType::HighTrafficBase.to_u16().unwrap() {
                return 75; // high traffic
            }
            if tile_value >= TileType::LowTrafficBase.to_u16().unwrap() {
                return 50; // low traffic
            }

            if tile_value < TileType::HorizontalBridge.to_u16().unwrap() {
                if tile_value > TileType::Fire.to_u16().unwrap() {
                    return 90; // fire
                }
                if tile_value >= TileType::Radioactive.to_u16().unwrap() {
                    return 255; // radioactivity
                }
            }
            return 0;
        }

        if tile_value <= TileType::LastIndustrial.to_u16().unwrap() {
            return 0;
        }
        if tile_value < TileType::PortBase.to_u16().unwrap() {
            return 50; // industrial
        }
        if tile_value <= TileType::LastPowerPlant.to_u16().unwrap() {
            return 100; // port, airport, coal power plant
        }

        0
    }

    /// Compute the distance to the city center for the entire map.
    fn compute_commercial_rate_map(
        commercial_rate_map: &mut Map<i16>,
        current_city_center: &MapPosition,
    ) {
        let bounds = commercial_rate_map.bounds();
        for x in 0..bounds.get_width() {
            for y in 0..bounds.get_height() {
                let mut z = ((Self::get_distance_from_city_center(
                    current_city_center,
                    &(x * 8, y * 8).into(),
                )) / 2) as i16; // 0..32
                z *= 4; // 0..128
                z = 64 - z; // -64..64
                commercial_rate_map.set_tile_at(&(x, y).into(), z);
            }
        }
    }

    /// Compute the Manhattan distance between the center of the city and the given world position.
    fn get_distance_from_city_center(city_center: &MapPosition, position: &MapPosition) -> i32 {
        let distance_x = abs(position.get_x() - city_center.get_x());
        let distance_y = abs(position.get_y() - city_center.get_y());
        min(distance_x + distance_y, 64)
    }

    fn smooth_terrain(&mut self) {
        todo!()
    }

    /// Smooth a station map.
    ///
    /// Used for smoothing fire station and police station coverage maps.
    fn smooth_station_map(map: &mut Map<u8>) {
        let mut temp_map = map.clone();
        let bounds = temp_map.bounds();
        for x in 0..bounds.get_width() {
            for y in 0..bounds.get_height() {
                let mut edge = 0;
                if x > 0 {
                    edge += *temp_map.get_tile_at(&(x - 1, y).into()).expect(
                        "CitySimulationScanner.smooth_station_map should get tile at (x - 1, y)",
                    );
                }
                if x < bounds.get_width() - 1 {
                    edge += *temp_map.get_tile_at(&(x + 1, y).into()).expect(
                        "CitySimulationScanner.smooth_station_map should get tile at (x + 1, y)",
                    );
                }
                if y > 0 {
                    edge += *temp_map.get_tile_at(&(x, y - 1).into()).expect(
                        "CitySimulationScanner.smooth_station_map should get tile at (x, y - 1)",
                    );
                }
                if y < bounds.get_height() - 1 {
                    edge += *temp_map.get_tile_at(&(x, y + 1).into()).expect(
                        "CitySimulationScanner.smooth_station_map should get tile at (x, y + 1)",
                    );
                }

                let value_at_position = temp_map
                    .get_tile_mut_at(&(x, y).into())
                    .expect("CitySimulationScanner.smooth_station_map should get tile at (x, y)");
                edge = *value_at_position + edge / 4;
                *value_at_position = edge / 2;
            }
        }
    }

    /// Smooth `temp_map_1` to `temp_map_2`.
    fn do_smooth_1(&mut self) {
        Self::smooth_dither_map(
            &self.temp_map_1,
            &mut self.temp_map_2,
            self.don_dither & 0x02 != 0x00,
        )
    }

    /// Smooth `temp_map_2` to `temp_map_1`.
    fn do_smooth_2(&mut self) {
        Self::smooth_dither_map(
            &self.temp_map_2,
            &mut self.temp_map_1,
            self.don_dither & 0x04 != 0x00,
        )
    }

    /// Perform smoothing with or without dithering.
    fn smooth_dither_map(source_map: &Map<u8>, destination_map: &mut Map<u8>, dither_flag: bool) {
        let source_bounds = source_map.bounds();
        if dither_flag {
            let mut z = 0;
            let mut direction = 1;
            for x in 0..source_bounds.get_width() as i32 {
                let mut y: i32 = 0;
                while y != source_bounds.get_height() as i32 && y != -1 {
                    z += *source_map
                        .get_tile_at(&(if x == 0 { x } else { x - 1 }, y).into())
                        .unwrap()
                        + *source_map
                            .get_tile_at(
                                &(
                                    if x == source_bounds.get_width() as i32 - 1 {
                                        x
                                    } else {
                                        x + 1
                                    },
                                    y,
                                )
                                    .into(),
                            )
                            .unwrap()
                        + *source_map
                            .get_tile_at(&(x, if y == 0 { 0 } else { y - 1 }).into())
                            .unwrap()
                        + *source_map
                            .get_tile_at(
                                &(
                                    x,
                                    if y == source_bounds.get_height() as i32 - 1 {
                                        y
                                    } else {
                                        y + 1
                                    },
                                )
                                    .into(),
                            )
                            .unwrap()
                        + *source_map.get_tile_at(&(x, y).into()).unwrap();
                    let value = (z / 4) as u8;
                    destination_map.set_tile_at(&(x, y).into(), value);
                    z &= 0x03;

                    y += direction;
                }

                direction = -direction;
                // y += direction;
            }
        } else {
            for x in 0..source_bounds.get_width() {
                for y in 0..source_bounds.get_height() {
                    let mut z = 0;
                    if x > 0 {
                        z += *source_map.get_tile_at(&(x - 1, y).into()).unwrap();
                    }
                    if x < source_bounds.get_width() - 1 {
                        z += *source_map.get_tile_at(&(x + 1, y).into()).unwrap();
                    }
                    if y > 0 {
                        z += *source_map.get_tile_at(&(x, y - 1).into()).unwrap();
                    }
                    if y < source_bounds.get_height() - 1 {
                        z += *source_map.get_tile_at(&(x, y + 1).into()).unwrap();
                    }

                    z = (z + source_map.get_tile_at(&(x, y).into()).unwrap()) >> 2;
                    if z > 255 {
                        z = 255;
                    }
                    destination_map.set_tile_at(&(x, y).into(), z as u8);
                }
            }
        }
    }
}
