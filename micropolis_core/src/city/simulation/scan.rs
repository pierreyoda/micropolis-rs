use std::cmp::min;

use num_traits::abs;

use crate::{
    map::{
        tiles::{TILE_LOW_MASK, TILE_ZONE_BIT},
        Map, MapClusteringStrategy, MapPosition, TileMap, TileType,
    },
    utils::random::MicropolisRandom,
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
    fn get_population_density_at(map: &TileMap, position: &MapPosition, tile_raw: u16) -> u16 {
        if tile_raw == TileType::FreeZoneCenter.to_u16().unwrap() {
            count_free_population(map, position)
        } else if tile_raw < TileType::CommercialBase.to_u16().unwrap() {
            get_residential_zone_population(tile_raw)
        } else if tile_raw < TileType::IndustrialBase.to_u16().unwrap() {
            get_commercial_zone_population(tile_raw) * 8
        } else if tile_raw < TileType::PortBase.to_u16().unwrap() {
            get_industrial_zone_population(tile_raw) * 8
        } else {
            0
        }
    }

    pub fn pollution_terrain_land_value_scan(
        rng: &mut MicropolisRandom,
        pollution_density: &Map<u8>,
        terrain_density: &Map<u8>,
        land_value_map: &Map<u8>,
    ) {
        let bounds = land_value_map.bounds();
        for x in 0..bounds.get_width() {
            for y in 0..bounds.get_height() {
                let position: MapPosition = (x, y).into();
                let world_position: MapPosition = (x * 2, y * 2).into();
            }
        }
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

    fn smooth_terrain() {}

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
