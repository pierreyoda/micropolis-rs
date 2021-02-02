pub mod constants;

use super::tiles::{TILE_BLBNBIT_MASK, TILE_BURN_BULL_BIT};
use super::tiles_type::{WOODS_HIGH, WOODS_LOW};
use super::{
    Map, MapClusteringStrategy, MapPosition, MapPositionOffset, MapRectangle, Tile, TileMap,
    TileType,
};
use crate::utils::Percentage;
use crate::{map::tools::ToolEffects, utils::random::MicropolisRandom};

/// Should the map generator form an island?
#[derive(Clone, Debug, PartialEq)]
pub enum GeneratorCreateIsland {
    Never,
    Always,
    /// X% of the time. 10% by default.
    Sometimes(Percentage),
}

#[derive(Debug, PartialEq)]
/// Random map terrain generator.
pub struct MapGenerator {
    /// Controls how often the generated terrain should be an island.
    create_island: GeneratorCreateIsland,
    /// Controls the level of tree creation..
    ///
    /// -1 => default number of trees.
    ///  0 => no trees.
    /// >0 => roughly equal to the remaining number of trees to randomly place.
    level_trees: i16,
    /// Controls the level of river curviness.
    ///
    /// -1 => default curve level.
    ///  0 => no rivers.
    /// >0 => curvier rivers.
    level_river_curves: i16,
    /// Level for lakes creation.
    ///
    /// -1 => default number of lakes.
    ///  0 => no lakes.
    /// >0 => extra lakes.
    level_lakes: i16,
}

#[derive(Debug)]
pub struct GeneratedTileMap {
    generation_seed: i32,
    pub generated_terrain: TileMap,
}

impl GeneratedTileMap {
    pub fn get_seed(&self) -> i32 {
        self.generation_seed
    }
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

    pub fn random_map_terrain(
        &self,
        rng: &mut MicropolisRandom,
        seed: i32,
        dimensions: &MapRectangle,
    ) -> Result<GeneratedTileMap, String> {
        // random setup
        rng.seed(seed);

        // initial landscape
        let mut terrain = match &self.create_island {
            GeneratorCreateIsland::Never => {
                Map::tilemap_with_dimensions(dimensions, TileType::Dirt)?
            }
            GeneratorCreateIsland::Always => {
                Self::make_naked_island(rng, self.level_lakes, dimensions)
            }
            GeneratorCreateIsland::Sometimes(chance) => {
                if (rng.get_random(100) as f64) / 100f64 < chance.value() {
                    let generated_terrain = self.generate_terrain_as_island(rng, dimensions)?;
                    return Ok(GeneratedTileMap {
                        generation_seed: seed,
                        generated_terrain,
                    });
                } else {
                    Map::tilemap_with_dimensions(dimensions, TileType::Dirt)?
                }
            }
        };

        // generate some rivers
        if self.level_river_curves != 0 {
            let starting_position = MapPosition::new(
                40 + rng.get_random(dimensions.width as i16 - 80) as i32,
                33 + rng.get_random(dimensions.height as i16 - 67) as i32,
            );
            Self::make_rivers(
                rng,
                self.level_river_curves,
                &mut terrain,
                &starting_position,
            );
        }

        // generate a few lakes
        if self.level_lakes != 0 {
            Self::make_lakes(rng, self.level_lakes, &mut terrain);
        }

        Self::smooth_rivers(rng, &mut terrain)?;

        // plant some trees
        if self.level_trees != 0 {
            Self::make_forests(rng, self.level_trees, &mut terrain)?;
        }

        Ok(GeneratedTileMap {
            generation_seed: seed,
            generated_terrain: terrain,
        })
    }

    fn generate_terrain_as_island(
        &self,
        rng: &mut MicropolisRandom,
        dimensions: &MapRectangle,
    ) -> Result<TileMap, String> {
        let mut terrain = Self::make_naked_island(rng, self.level_lakes, dimensions);
        Self::smooth_rivers(rng, &mut terrain)?;
        Self::make_forests(rng, self.level_trees, &mut terrain)?;
        Ok(terrain)
    }

    /// Generate a plain island surrounded by 5 tiles of river.
    fn make_naked_island(
        rng: &mut MicropolisRandom,
        level_lakes: i16,
        dimensions: &MapRectangle,
    ) -> TileMap {
        // rectangular island
        let (x_max, y_max) = (dimensions.width as i32 - 5, dimensions.height as i32 - 5);
        let tilemap: Vec<Vec<Tile>> = (0..dimensions.width)
            .map(|x| {
                (0..dimensions.height)
                    .map(|y| {
                        Tile::from_type(
                            if x >= 5 && x < x_max as usize && y >= 5 && y < y_max as usize {
                                TileType::Dirt
                            } else {
                                TileType::River
                            },
                        )
                        .unwrap()
                    })
                    .collect()
            })
            .collect();
        let mut terrain = TileMap {
            clustering_strategy: MapClusteringStrategy::BlockSize8,
            data: tilemap,
        };

        for x in (0..x_max).step_by(2) {
            let y1 = rng.get_e_random(constants::ISLAND_RADIUS);
            Self::plop_big_river(&mut terrain, &(x, y1 as i32).into());

            let y2 = (dimensions.height as i16 - 10) - rng.get_e_random(constants::ISLAND_RADIUS);
            Self::plop_big_river(&mut terrain, &(x, y2 as i32).into());

            Self::plop_small_river(&mut terrain, &(x, 0).into());
            Self::plop_small_river(&mut terrain, &(x, dimensions.height as i32 - 6).into());
        }

        for y in (0..y_max).step_by(2) {
            let x1 = rng.get_e_random(constants::ISLAND_RADIUS);
            Self::plop_big_river(&mut terrain, &(x1 as i32, y).into());

            let x2 = (dimensions.width as i16 - 10) - rng.get_e_random(constants::ISLAND_RADIUS);
            Self::plop_big_river(&mut terrain, &(x2 as i32, y).into());

            Self::plop_small_river(&mut terrain, &(0, y).into());
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

    fn make_lakes(rng: &mut MicropolisRandom, level_lakes: i16, map: &mut TileMap) {
        let mut remaining_lakes = if level_lakes < 0 {
            rng.get_random(10)
        } else {
            level_lakes / 2
        };

        let map_size = map.bounds();
        while remaining_lakes > 0 {
            let x = 10 + rng.get_random(map_size.width as i16 - 21);
            let y = 10 + rng.get_random(map_size.width as i16 - 20);
            Self::make_single_lake(rng, map, (x as i32, y as i32).into());
            remaining_lakes -= 1;
        }
    }

    /// Generate a single lake around the given rough position.
    fn make_single_lake(rng: &mut MicropolisRandom, terrain: &mut TileMap, at: MapPosition) {
        let mut num_plops = 2 + rng.get_random(12);
        while num_plops > 0 {
            let offset_x = rng.get_random(12) - 6;
            let offset_y = rng.get_random(12) - 6;
            let plop_position = MapPosition {
                x: at.x + offset_x as i32,
                y: at.y + offset_y as i32,
            };

            if rng.get_random(4) > 0 {
                Self::plop_small_river(terrain, &plop_position)
            } else {
                Self::plop_big_river(terrain, &plop_position)
            };
            num_plops -= 1;
        }
    }

    fn make_rivers(
        rng: &mut MicropolisRandom,
        level_river_curves: i16,
        terrain: &mut TileMap,
        start: &MapPosition,
    ) {
        let mut global_direction = Self::random_straight_direction(rng);
        Self::make_big_river(
            rng,
            level_river_curves,
            terrain,
            start,
            &global_direction,
            &global_direction,
        );

        global_direction = global_direction.rotated_180();
        let local_direction = Self::make_big_river(
            rng,
            level_river_curves,
            terrain,
            start,
            &global_direction,
            &global_direction,
        );

        global_direction = Self::random_straight_direction(rng);
        Self::make_small_river(
            rng,
            level_river_curves,
            terrain,
            start,
            &global_direction,
            &local_direction,
        );
    }

    /// Make a big river.
    ///
    /// Returns the last local river direction.
    fn make_big_river(
        rng: &mut MicropolisRandom,
        level_river_curves: i16,
        terrain: &mut TileMap,
        start: &MapPosition,
        global_direction: &MapPositionOffset,
        local_direction: &MapPositionOffset,
    ) -> MapPositionOffset {
        let (rate1, rate2) = match level_river_curves {
            level if level < 0 => (100, 200),
            level => (10 + level, 100 + level),
        };

        let mut position = *start;
        let mut last_local_direction = *local_direction;
        while terrain.in_bounds(&MapPosition {
            x: position.x + 4,
            y: position.y + 4,
        }) {
            Self::plop_big_river(terrain, &position);
            if rng.get_random(rate1) < 10 {
                last_local_direction = *global_direction;
            } else {
                if rng.get_random(rate2) > 90 {
                    last_local_direction = last_local_direction.rotated_45();
                }
                if rng.get_random(rate2) > 90 {
                    // FIXME: C++ code has a 7 'count' parameter?
                    last_local_direction = last_local_direction.rotated_45();
                }
            }
            position = last_local_direction.apply(&position);
        }
        last_local_direction
    }

    // TODO: factorize code with make_big_river (macro/closures)
    fn make_small_river(
        rng: &mut MicropolisRandom,
        level_river_curves: i16,
        terrain: &mut TileMap,
        start: &MapPosition,
        global_direction: &MapPositionOffset,
        local_direction: &MapPositionOffset,
    ) -> MapPositionOffset {
        let (rate1, rate2) = match level_river_curves {
            level if level < 0 => (100, 200),
            level => (10 + level, 100 + level),
        };

        let mut position = start.clone();
        let mut last_local_direction = local_direction.clone();
        while terrain.in_bounds(&MapPosition {
            x: position.x + 3,
            y: position.y + 3,
        }) {
            Self::plop_small_river(terrain, &position);
            if rng.get_random(rate1) < 10 {
                last_local_direction = global_direction.clone();
            } else {
                if rng.get_random(rate2) > 90 {
                    last_local_direction = last_local_direction.rotated_45();
                }
                if rng.get_random(rate2) > 90 {
                    // FIXME: C++ code has a 7 'count' parameter?
                    last_local_direction = last_local_direction.rotated_45();
                }
            }
            position = local_direction.apply(&position);
        }
        last_local_direction
    }

    fn smooth_rivers(rng: &mut MicropolisRandom, terrain: &mut TileMap) -> Result<(), String> {
        let map_size = terrain.bounds();
        let dirt_type_raw = TileType::Dirt
            .to_u16()
            .ok_or("Dirt tile type raw conversion error")?;
        let river_type_raw = TileType::River
            .to_u16()
            .ok_or("River tile type raw conversion error")?;
        for x in 0..map_size.width {
            for y in 0..map_size.height {
                {
                    // avoid immutable / mutable borrow conflict
                    // TODO: find better way
                    let tile = terrain
                        .data
                        .get(x)
                        .ok_or(format!(
                            "MapGenerator.smooth_rivers map X overflow at {}",
                            x
                        ))?
                        .get(y)
                        .ok_or(format!(
                            "MapGenerator.smooth_rivers map Y overflow at {}",
                            y
                        ))?;
                    if tile.get_type() != &Some(TileType::RiverEdge) {
                        continue;
                    }
                }
                let mut bit_index = 0;
                for i in 0..4 {
                    bit_index <<= 1;
                    let temp_position = MapPosition {
                        x: x as i32 + constants::SMOOTH_TILES_DX[i],
                        y: y as i32 + constants::SMOOTH_TILES_DY[i],
                    };
                    if !map_size.is_inside(&temp_position) {
                        continue;
                    }
                    let temp_tile_type_raw = terrain
                        .data
                        .get(temp_position.x as usize)
                        .ok_or(format!(
                            "MapGenerator.smooth_rivers map X overflow at temp X={}",
                            temp_position.x
                        ))?
                        .get(temp_position.y as usize)
                        .ok_or(format!(
                            "MapGenerator.smooth_rivers map Y overflow at temp Y={}",
                            temp_position.y
                        ))?
                        .get_type_raw();
                    if temp_tile_type_raw == dirt_type_raw {
                        continue;
                    }
                    if !(WOODS_LOW..=WOODS_HIGH).contains(&temp_tile_type_raw) {
                        bit_index += 1;
                    }
                }
                let tile = terrain.data.get_mut(x).unwrap().get_mut(y).unwrap();
                let mut tile_raw = constants::SMOOTH_RIVER_EDGES_TABLE[bit_index & 0x000F];
                if tile_raw != river_type_raw && rng.get_random(1) > 0 {
                    tile_raw += 1;
                }
                tile.set_raw(tile_raw);
            }
        }
        Ok(())
    }

    fn make_forests(
        rng: &mut MicropolisRandom,
        level_trees: i16,
        terrain: &mut TileMap,
    ) -> Result<(), String> {
        let amount = match level_trees {
            level if level < 0 => 50 + rng.get_random(100),
            level => 3 + level,
        };
        let map_size = terrain.bounds();
        for _ in 0..amount {
            let x = rng.get_random(map_size.width as i16 - 1);
            let y = rng.get_random(map_size.height as i16 - 1);
            Self::splash_trees(rng, level_trees, terrain, &(x, y).into());
        }

        Self::smooth_trees(terrain)?;
        Self::smooth_trees(terrain)?; // TODO: why the repetition ?

        Ok(())
    }

    /// Splash a bunch of trees near the given position.
    ///
    /// The amount of trees generated depends on `level_trees`.
    /// Note: trees are not smoothed.
    ///
    /// TODO: function generates trees even if `level_trees` is 0 (original bug).
    fn splash_trees(
        rng: &mut MicropolisRandom,
        level_trees: i16,
        terrain: &mut TileMap,
        at: &MapPosition,
    ) {
        let mut trees_count = match level_trees {
            level if level < 0 => 50 + rng.get_random(150),
            level => 50 + rng.get_random(100 + level * 2),
        };
        let mut tree_position = *at;
        let woods_type_raw = TileType::Woods.to_u16().unwrap();

        while trees_count > 0 {
            let direction = Self::random_direction(rng);
            tree_position = direction.apply(&tree_position);
            if !terrain.in_bounds(&tree_position) {
                return;
            }
            let tile = terrain
                .data
                .get_mut(tree_position.x as usize)
                .unwrap()
                .get_mut(tree_position.y as usize)
                .unwrap();
            if tile.get_type() == &Some(TileType::Dirt) {
                tile.set_raw(TILE_BLBNBIT_MASK | woods_type_raw);
            }
            trees_count -= 1;
        }
    }

    fn smooth_trees(terrain: &mut TileMap) -> Result<(), String> {
        let map_size = terrain.bounds();
        let dirt_type_raw = TileType::Dirt
            .to_u16()
            .ok_or("Dirt tile type raw conversion error")?;
        let woods_type_raw = TileType::Woods
            .to_u16()
            .ok_or("Woods tile type raw conversion error")?;
        for x in 0..map_size.width {
            for y in 0..map_size.height {
                {
                    // avoid immutable / mutable borrow conflict
                    // TODO: find better way
                    let tile = terrain
                        .data
                        .get_mut(x)
                        .ok_or(format!("MapGenerator.smooth_trees map overflow at X={}", x,))?
                        .get_mut(y)
                        .ok_or(format!("MapGenerator.smooth_trees map overflow at Y={}", y))?;
                    if !tile.is_tree() {
                        continue;
                    }
                }
                let mut bit_index = 0;
                for i in 0..4 {
                    bit_index <<= 1;
                    let temp_position = MapPosition {
                        x: x as i32 + constants::SMOOTH_TILES_DX[i],
                        y: y as i32 + constants::SMOOTH_TILES_DY[i],
                    };
                    if !map_size.is_inside(&temp_position) {
                        continue;
                    }
                    let temp_tile = terrain
                        .data
                        .get(temp_position.x as usize)
                        .ok_or(format!(
                            "MapGenerator.smooth_trees map X overflow at temp X={}",
                            temp_position.x
                        ))?
                        .get(temp_position.y as usize)
                        .ok_or(format!(
                            "MapGenerator.smooth_trees map Y overflow at temp Y={}",
                            temp_position.y
                        ))?;
                    if temp_tile.is_tree() {
                        bit_index += 1;
                    }
                }
                let tile = terrain.data.get_mut(x).unwrap().get_mut(y).unwrap();
                let mut tile_raw = constants::SMOOTH_FOREST_EDGES_TABLE[bit_index & 0x000F];
                if tile_raw == dirt_type_raw {
                    tile.set_type_raw(tile_raw);
                } else {
                    if tile_raw != woods_type_raw && (x + y) & 0x1 == 0x1 {
                        tile_raw -= 8;
                    }
                    tile.set_raw(TILE_BLBNBIT_MASK | tile_raw);
                }
            }
        }
        Ok(())
    }

    pub fn smooth_trees_at(
        terrain: &TileMap,
        position: &MapPosition,
        mut effects: ToolEffects,
        preserve: bool,
    ) -> Result<ToolEffects, String> {
        if !effects
            .get_map_value_at(terrain, position)
            .ok_or(format!(
                "MapGenerator.smooth_trees_at cannot read effects tile value at {}",
                position
            ))?
            .is_tree()
        {
            return Ok(effects);
        }

        let mut bit_index: u16 = 0;
        for z in 0..4 {
            bit_index <<= 0x01;
            if terrain.in_bounds(
                &(*position
                    + MapPosition::new(
                        constants::SMOOTH_TILES_DX[z],
                        constants::SMOOTH_TILES_DY[z],
                    )),
            ) {
                bit_index += 1;
            }
        }

        let table_index = (bit_index & 0x0F) as usize;
        let temp = *constants::SMOOTH_FOREST_EDGES_TABLE
            .get(table_index)
            .ok_or(format!(
                "MapGenerator.smooth_trees_at SMOOTH_FOREST_EDGES_TABLE overflow: {}",
                table_index
            ))?;
        match temp {
            0 => Ok(effects.add_modification(
                position,
                Tile::from_raw(
                    TILE_BURN_BULL_BIT
                        | if TileType::from_u16(temp).ok_or(format!(
                            "MapGenerator.smooth_trees_at cannot create tile from {}",
                            temp
                        ))? != TileType::Woods
                            && (position.x + position.y) & 0x01 != 0x00
                        {
                            temp - 8
                        } else {
                            temp
                        },
                )?,
            )),
            _ => Ok(if preserve {
                effects
            } else {
                effects.add_modification(position, Tile::from_raw(temp)?)
            }),
        }
    }

    /// Put down a big diamond-like shaped river, where `base` is the top-left position of the blob.
    fn plop_big_river(terrain: &mut TileMap, base: &MapPosition) {
        for x in 0..9 {
            for y in 0..9 {
                let position = *base + (x, y).into();
                if !terrain.in_bounds(&position) {
                    continue;
                }
                Self::set_tile(terrain, constants::BLOB_RIVER_BIG[y][x].clone(), &position)
                    .expect("MapGenerator.plop_big_river set tile error");
            }
        }
    }

    /// Put down a small diamond-like shaped river, where `base` is the top-left position of the blob.
    fn plop_small_river(terrain: &mut TileMap, base: &MapPosition) {
        for x in 0..6 {
            for y in 0..6 {
                let position = *base + (x, y).into();
                if !terrain.in_bounds(&position) {
                    continue;
                }
                Self::set_tile(
                    terrain,
                    constants::BLOB_RIVER_SMALL[y][x].clone(),
                    &position,
                )
                .expect("MapGenerator.plop_small_river set tile error");
            }
        }
    }

    fn set_tile(
        terrain: &mut TileMap,
        new_tile_type: TileType,
        at: &MapPosition,
    ) -> Result<(), String> {
        if new_tile_type == TileType::Dirt {
            return Ok(());
        }
        let row = terrain
            .data
            .get_mut(at.x as usize)
            .ok_or("MapGenerator.set_tile map X overflow")?;
        let tile = row
            .get_mut(at.y as usize)
            .ok_or("MapGenerator.set_tile map Y overflow")?;
        match tile.get_type() {
            Some(TileType::Dirt) => tile.set_type(new_tile_type),
            Some(TileType::River) if new_tile_type != TileType::Channel => Ok(()),
            Some(TileType::Channel) => Ok(()),
            _ => tile.set_type(new_tile_type),
        }
    }

    // TODO: make this into enum standard distribution
    fn random_direction(rng: &mut MicropolisRandom) -> MapPositionOffset {
        use MapPositionOffset::*;
        match rng.get_random(7) {
            0 => NorthWest,
            1 => North,
            2 => NorthEast,
            3 => East,
            4 => SouthEast,
            5 => South,
            6 => SouthWest,
            7 => West,
            _ => unreachable!(),
        }
    }

    fn random_straight_direction(rng: &mut MicropolisRandom) -> MapPositionOffset {
        use MapPositionOffset::*;
        match rng.get_random(3) {
            0 => North,
            1 => East,
            2 => South,
            3 => West,
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::map::tiles_type::TileType;
    use crate::map::MapRectangle;

    #[test]
    fn test_temp_print() {
        let mut rng = MicropolisRandom::from_random_system_seed();
        let generator = MapGenerator::with_options(GeneratorCreateIsland::Never);
        let generated = generator
            .random_map_terrain(&mut rng, 12345, &MapRectangle::new(120, 100))
            .unwrap();
        let tiles = generated.generated_terrain.tiles();
        println!("TEMP: generating map for console print...");
        let mut repr = String::new();
        for row in tiles.iter() {
            repr.push('/');
            for tile in row.iter() {
                repr.push(match tile.get_type() {
                    Some(TileType::River) => '~',
                    Some(TileType::RiverEdge) => '&',
                    Some(TileType::Channel) => '#',
                    Some(TileType::Dirt) => '.',
                    _ => 'T',
                });
            }
            repr.push_str("/\n");
        }
        println!("{}", repr);
    }
}
