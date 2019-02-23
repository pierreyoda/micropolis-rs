pub mod constants;

use std::cmp;

use rand::Rng;

use super::{Map, MapPosition, MapPositionOffset, MapRect, TileType};

pub struct Percentage(f64);

impl Percentage {
    pub fn from_integer(percentage: u8) -> Option<Self> {
        if percentage <= 100 {
            Some(Percentage(percentage as f64 / 100f64))
        } else {
            None
        }
    }

    /// Value, in percentage (%).
    pub fn value(&self) -> f64 {
        self.0
    }
}

/// Should the map generated as an island?
pub enum GeneratorCreateIsland {
    Never,
    Always,
    /// X% of the time (10% by default).
    Sometimes(Percentage),
}

/// Random map terrain generator.
pub struct MapGenerator {
    /// Controls how often the generated terrain should be an island.
    create_island: GeneratorCreateIsland,
    /// Controls the level of tree creation..
    ///
    /// -1 => default number of trees.
    ///  0 => no trees.
    /// >0 => roughly equal to the remaining number of trees to randomly place.
    level_trees: i32,
    /// Controls the level of river curviness.
    ///
    /// -1 => default curve level.
    ///  0 => no rivers.
    /// >0 => curvier rivers.
    level_river_curves: i32,
    /// Level for lakes creation.
    ///
    /// -1 => default number of lakes.
    ///  0 => no lakes.
    /// >0 => extra lakes.
    level_lakes: i32,
}

impl MapGenerator {
    pub fn with_options(create_island: GeneratorCreateIsland) -> Self {
        MapGenerator {
            create_island,
            level_trees: -1,
            level_river_curves: -1,
            level_lakes: -1,
        }
    }

    pub fn random_map_terrain<R: Rng>(&self, rng: &mut R, dimensions: &MapRect) -> Map {
        // initial landscape
        let mut terrain = match &self.create_island {
            GeneratorCreateIsland::Never => Map::with_dimensions(dimensions, TileType::Dirt),
            GeneratorCreateIsland::Always => self.make_naked_island(rng, dimensions),
            GeneratorCreateIsland::Sometimes(chance) => {
                if rng.gen_bool(chance.value()) {
                    self.make_island(rng, dimensions)
                } else {
                    Map::with_dimensions(dimensions, TileType::Dirt)
                }
            }
        };

        // lay a river
        if self.level_river_curves != 0 {
            let start_position = MapPosition {
                x: 40 + Self::random_in_range(rng, 0, dimensions.width as i32 - 80),
                y: 33 + Self::random_in_range(rng, 0, dimensions.height as i32 - 67),
            };
            self.make_rivers(rng, &mut terrain, &start_position);
        }

        // lay down a few lakes
        if self.level_lakes != 0 {
            self.make_lakes(rng, &mut terrain);
        }

        self.smooth_rivers(&mut terrain);

        // plant trees
        if self.level_trees != 0 {
            self.make_forests(rng, &mut terrain);
        }

        terrain
    }

    fn make_island<R: Rng>(&self, rng: &mut R, dimensions: &MapRect) -> Map {
        let mut terrain = self.make_naked_island(rng, dimensions);
        self.smooth_rivers(&mut terrain);
        self.make_forests(rng, &mut terrain);
        terrain
    }

    /// Generate a plain island surrounded by 5 tiles of river.
    fn make_naked_island<R: Rng>(&self, rng: &mut R, dimensions: &MapRect) -> Map {
        // rectangular island
        let (x_max, y_max) = (dimensions.width as i32 - 5, dimensions.height as i32 - 5);
        let mut tiles: Vec<Vec<TileType>> = (0..dimensions.width)
            .map(|x| {
                (0..dimensions.height)
                    .map(|y| {
                        if x >= 5 && x < x_max as usize && y >= 5 && y < y_max as usize {
                            TileType::Dirt
                        } else {
                            TileType::River
                        }
                    })
                    .collect()
            })
            .collect();
        let mut terrain = Map { tiles };

        for x in (0..x_max).step_by(2) {
            let y1 = Self::erandom_in_range(rng, 0, constants::ISLAND_RADIUS as i32);
            Self::plop_big_river(&mut terrain, &MapPosition { x, y: y1 });
            let y2 = Self::erandom_in_range(rng, 0, constants::ISLAND_RADIUS as i32);
            Self::plop_big_river(&mut terrain, &MapPosition { x, y: y2 });

            Self::plop_small_river(&mut terrain, &MapPosition { x, y: 0 });
            Self::plop_small_river(
                &mut terrain,
                &MapPosition {
                    x,
                    y: dimensions.height as i32 - 6,
                },
            );
        }

        for y in (0..y_max).step_by(2) {
            let x1 = Self::erandom_in_range(rng, 0, constants::ISLAND_RADIUS as i32);
            Self::plop_big_river(&mut terrain, &MapPosition { x: x1, y });
            let x2 = Self::erandom_in_range(rng, 0, constants::ISLAND_RADIUS as i32);
            Self::plop_big_river(&mut terrain, &MapPosition { x: x2, y });

            Self::plop_small_river(&mut terrain, &MapPosition { x: 0, y });
            Self::plop_small_river(
                &mut terrain,
                &MapPosition {
                    x: dimensions.width as i32 - 6,
                    y,
                },
            );
        }

        terrain
    }

    fn make_lakes<R: Rng>(&self, rng: &mut R, map: &mut Map) {
        let mut remaining_lakes: i32 = if self.level_lakes < 0 {
            Self::random_in_range(rng, 0, 10)
        } else {
            self.level_lakes / 2
        };

        let map_size = map.get_bounds();
        while remaining_lakes > 0 {
            let x = 10 + Self::random_in_range(rng, 0, map_size.width as i32 - 21);
            let y = 10 + Self::random_in_range(rng, 0, map_size.width as i32 - 20);
            self.make_single_lake(rng, map, MapPosition { x, y });
            remaining_lakes -= 1;
        }
    }

    /// Generate a single lake around the given rough position.
    fn make_single_lake<R: Rng>(&self, rng: &mut R, terrain: &mut Map, at: MapPosition) {
        let mut num_plops = 2 + Self::random_in_range(rng, 0, 12);
        while num_plops > 0 {
            let offset_x = Self::random_in_range(rng, 0, 12) - 6;
            let offset_y = Self::random_in_range(rng, 0, 12) - 6;
            let plop_position = MapPosition {
                x: at.x + offset_x,
                y: at.y + offset_y,
            };
            // TODO: check equivalence with C++ code
            match rng.gen_ratio(4, 5) {
                true => Self::plop_small_river(terrain, &plop_position),
                false => Self::plop_big_river(terrain, &plop_position),
            };
            num_plops -= 1;
        }
    }

    fn make_rivers<R: Rng>(&self, rng: &mut R, terrain: &mut Map, start: &MapPosition) {
        let mut global_direction = Self::random_straight_direction(rng);
        self.make_big_river(rng, terrain, start, &global_direction, &global_direction);

        global_direction = global_direction.rotated_180();
        let local_direction =
            self.make_big_river(rng, terrain, start, &global_direction, &global_direction);

        global_direction = Self::random_straight_direction(rng);
        self.make_small_river(rng, terrain, start, &global_direction, &local_direction);
    }

    /// Make a big river.
    ///
    /// Returns the last local river direction.
    fn make_big_river<R: Rng>(
        &self,
        rng: &mut R,
        terrain: &mut Map,
        start: &MapPosition,
        global_direction: &MapPositionOffset,
        local_direction: &MapPositionOffset,
    ) -> MapPositionOffset {
        let (rate1, rate2) = match self.level_river_curves {
            level if level < 0 => (100, 200),
            level => (10 + level, 100 + level),
        };

        let mut position = start.clone();
        let mut local_direction = MapPositionOffset::None;
        while terrain.in_bounds(&MapPosition {
            x: position.x + 4,
            y: position.y + 4,
        }) {
            Self::plop_big_river(terrain, &position);
            if Self::random_in_range(rng, 0, rate1) < 10 {
                local_direction = global_direction.clone();
            } else {
                if Self::random_in_range(rng, 0, rate2) > 90 {
                    local_direction = local_direction.rotated_45();
                }
                if Self::random_in_range(rng, 0, rate2) > 90 {
                    // FIXME: C++ code has a 7 'count' parameter?
                    local_direction = local_direction.rotated_45();
                }
            }
            position = local_direction.apply(&position);
        }
        local_direction
    }

    // TODO: factorize code with make_big_river (macro/closures)
    fn make_small_river<R: Rng>(
        &self,
        rng: &mut R,
        terrain: &mut Map,
        start: &MapPosition,
        global_direction: &MapPositionOffset,
        local_direction: &MapPositionOffset,
    ) -> MapPositionOffset {
        let (rate1, rate2) = match self.level_river_curves {
            level if level < 0 => (100, 200),
            level => (10 + level, 100 + level),
        };

        let mut position = start.clone();
        let mut local_direction = MapPositionOffset::None;
        while terrain.in_bounds(&MapPosition {
            x: position.x + 3,
            y: position.y + 3,
        }) {
            Self::plop_small_river(terrain, &position);
            if Self::random_in_range(rng, 0, rate1) < 10 {
                local_direction = global_direction.clone();
            } else {
                if Self::random_in_range(rng, 0, rate2) > 90 {
                    local_direction = local_direction.rotated_45();
                }
                if Self::random_in_range(rng, 0, rate2) > 90 {
                    // FIXME: C++ code has a 7 'count' parameter?
                    local_direction = local_direction.rotated_45();
                }
            }
            position = local_direction.apply(&position);
        }
        local_direction
    }

    fn smooth_rivers(&self, terrain: &mut Map) {}

    fn make_forests<R: Rng>(&self, rng: &mut R, terrain: &mut Map) {
        let amount: i32 = match self.level_trees {
            level if level < 0 => 50 + Self::random_in_range(rng, 0, 100),
            level => 3 + level,
        };
        let map_size = terrain.get_bounds();
        for i in 0..amount {
            let x = Self::random_in_range(rng, 0, map_size.width as i32 - 1);
            let y = Self::random_in_range(rng, 0, map_size.height as i32 - 1);
            self.splash_trees(rng, terrain, &MapPosition { x, y });
        }

        self.smooth_trees(terrain);
        self.smooth_trees(terrain);
    }

    /// Splash a bunch of trees near the given position.
    ///
    /// The amount of trees generated depends on `level_trees`.
    /// Note: trees are not smoothed.
    /// TODO: function generates trees even if `level_trees` is 0 (original bug).
    fn splash_trees<R: Rng>(&self, rng: &mut R, terrain: &mut Map, at: &MapPosition) {
        let mut num_trees: i32 = match self.level_trees {
            level if level < 0 => 50 + Self::random_in_range(rng, 0, 150),
            level => 50 + Self::random_in_range(rng, 0, 100 + level * 2),
        };
        while num_trees > 0 {
            let direction = Self::random_direction(rng);
            let tree_position = direction.apply(at);
            if !terrain.in_bounds(&tree_position) {
                // TODO: can the loop be stuck on edges?
                continue;
            }
            // TODO: implement once tile handles woods level
            num_trees -= 1;
        }
    }

    fn smooth_trees(&self, terrain: &mut Map) {}

    /// Put down a big diamond-like shaped river, where `base` is the top-left position of the blob.
    fn plop_big_river(terrain: &mut Map, base: &MapPosition) {
        for x in 0..9 {
            for y in 0..9 {
                Self::set_tile_type(
                    terrain,
                    constants::BLOB_RIVER_BIG[y][x].clone(),
                    &MapPosition {
                        x: base.x + x as i32,
                        y: base.y + y as i32,
                    },
                );
            }
        }
    }

    /// Put down a small diamond-like shaped river, where `base` is the top-left position of the blob.
    fn plop_small_river(terrain: &mut Map, base: &MapPosition) {
        for x in 0..6 {
            for y in 0..6 {
                Self::set_tile_type(
                    terrain,
                    constants::BLOB_RIVER_SMALL[y][x].clone(),
                    &MapPosition {
                        x: base.x + x as i32,
                        y: base.y + y as i32,
                    },
                );
            }
        }
    }

    fn set_tile_type(terrain: &mut Map, tile_type: TileType, at: &MapPosition) {
        if tile_type == TileType::Dirt || !terrain.in_bounds(at) {
            return;
        }
        let row = terrain
            .tiles
            .get_mut(at.x as usize)
            .expect("MapGenerator::set_tile_type X overflow");
        let tile = row
            .get_mut(at.y as usize)
            .expect("MapGenerator::set_tile_type Y overflow");
        match *tile {
            TileType::Dirt => *tile = tile_type,
            TileType::River if tile_type != TileType::Channel => return,
            TileType::Channel => return,
            _ => *tile = tile_type,
        }
    }

    // TODO: make this into enum standard distribution
    fn random_direction<R: Rng>(rng: &mut R) -> MapPositionOffset {
        use MapPositionOffset::*;
        match rng.gen_range(0u8, 8u8) {
            0 => NorthWest,
            1 => North,
            2 => NorthEast,
            3 => East,
            4 => SouthEast,
            5 => South,
            6 => SouthWest,
            _ => West,
        }
    }

    fn random_straight_direction<R: Rng>(rng: &mut R) -> MapPositionOffset {
        use MapPositionOffset::*;
        match rng.gen_range(0u8, 4u8) {
            0 => North,
            1 => East,
            2 => South,
            _ => West,
        }
    }

    /// Generate a random integer in the given inclusive range.
    fn random_in_range<R: Rng>(rng: &mut R, lower: i32, upper: i32) -> i32 {
        rng.gen_range(lower, upper + 1)
    }

    /// Generate a random integer in the given inclusive range with a bias
    /// towards smaller values.
    fn erandom_in_range<R: Rng>(rng: &mut R, lower: i32, upper: i32) -> i32 {
        let z = Self::random_in_range(rng, lower, upper);
        let x = Self::random_in_range(rng, lower, upper);
        return cmp::min(z, x);
    }
}

#[cfg(test)]
mod tests {
    use rand::rngs::OsRng;

    use super::*;
    use crate::map::tiles_type::TileType;
    use crate::map::MapRect;

    #[test]
    fn test_temp_print() {
        let mut rng = OsRng::new().unwrap();
        let generator = MapGenerator::with_options(GeneratorCreateIsland::Sometimes(
            Percentage::from_integer(50).unwrap(),
        ));
        let terrain = generator.random_map_terrain(&mut rng, &MapRect::new(80, 80));
        let tiles = terrain.tiles;
        println!("TEMP: generating map for console print...");
        let mut repr = String::new();
        for row in tiles.iter() {
            repr.push('/');
            for tile in row.iter() {
                repr.push(match tile {
                    TileType::River => '~',
                    TileType::Channel => '#',
                    TileType::Dirt => '.',
                    _ => 'T',
                });
            }
            repr.push_str("/\n");
        }
        println!("{}", repr);
    }
}
