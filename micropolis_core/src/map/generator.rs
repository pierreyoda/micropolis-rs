mod constants;
pub mod trees;
mod utils;
mod water;

use constants::ISLAND_RADIUS;
use trees::make_forests;
use water::{make_lakes, make_rivers, plop_big_river, plop_small_river, smooth_rivers};

use super::MapClusteringStrategy;
use super::{Map, MapPosition, MapRectangle, Tile, TileMap, TileType};
use crate::utils::random::MicropolisRandom;
use crate::utils::Percentage;

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
            make_rivers(
                rng,
                self.level_river_curves,
                &mut terrain,
                &starting_position,
            );
        }

        // generate a few lakes
        if self.level_lakes != 0 {
            make_lakes(rng, self.level_lakes, &mut terrain);
        }

        smooth_rivers(rng, &mut terrain)?;

        // plant some trees
        if self.level_trees != 0 {
            make_forests(rng, self.level_trees, &mut terrain)?;
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
        smooth_rivers(rng, &mut terrain)?;
        make_forests(rng, self.level_trees, &mut terrain)?;
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
            let y1 = rng.get_e_random(ISLAND_RADIUS);
            plop_big_river(&mut terrain, &(x, y1 as i32).into());

            let y2 = (dimensions.height as i16 - 10) - rng.get_e_random(ISLAND_RADIUS);
            plop_big_river(&mut terrain, &(x, y2 as i32).into());

            plop_small_river(&mut terrain, &(x, 0).into());
            plop_small_river(&mut terrain, &(x, dimensions.height as i32 - 6).into());
        }

        for y in (0..y_max).step_by(2) {
            let x1 = rng.get_e_random(ISLAND_RADIUS);
            plop_big_river(&mut terrain, &(x1 as i32, y).into());

            let x2 = (dimensions.width as i16 - 10) - rng.get_e_random(ISLAND_RADIUS);
            plop_big_river(&mut terrain, &(x2 as i32, y).into());

            plop_small_river(&mut terrain, &(0, y).into());
            plop_small_river(
                &mut terrain,
                &MapPosition {
                    x: dimensions.width as i32 - 6,
                    y,
                },
            );
        }

        terrain
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
