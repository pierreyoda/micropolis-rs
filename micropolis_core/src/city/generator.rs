use crate::{
    game::{GameLevelDifficulty, GameScenario},
    map::{
        generator::{GeneratorCreateIsland, MapGenerator},
        MapRectangle, TileMap,
    },
    utils::random::MicropolisRandom,
};

use super::City;

pub struct CityMapGeneratorOptions {
    pub dimensions: MapRectangle,
    pub create_island: GeneratorCreateIsland,
}

pub struct CityGeneratorBuilder {
    simulation_rng_seed: Option<i32>,
    city_map_generator_options: Option<CityMapGeneratorOptions>,
}

impl CityGeneratorBuilder {
    pub fn new() -> Self {
        Self {
            simulation_rng_seed: None,
            city_map_generator_options: None,
        }
    }

    pub fn with_simulation_rng_seed(mut self, simulation_seed: i32) -> Self {
        self.simulation_rng_seed = Some(simulation_seed);
        self
    }

    pub fn with_city_map_generator_options(
        mut self,
        generator_options: CityMapGeneratorOptions,
    ) -> Self {
        self.city_map_generator_options = Some(generator_options);
        self
    }

    pub fn build(self) -> Result<CityGenerator, String> {
        let map_generator_options = self.city_map_generator_options.ok_or_else(|| {
            "CityGeneratorBuilder.build() error: no city_map_generator_options provided".to_string()
        })?;

        let map_generator = MapGenerator::with_options(map_generator_options.create_island.clone());
        Ok(CityGenerator {
            map_generator,
            map_generator_options,
            simulation_rng_seed: self.simulation_rng_seed,
        })
    }
}

pub struct CityGenerator {
    map_generator: MapGenerator,
    map_generator_options: CityMapGeneratorOptions,
    simulation_rng_seed: Option<i32>,
}

impl CityGenerator {
    pub fn build_random_map(&self, seed: i32) -> Result<TileMap, String> {
        let mut rng = MicropolisRandom::from_seed(seed);
        let result = self.map_generator.random_map_terrain(
            &mut rng,
            seed,
            &self.map_generator_options.dimensions,
        )?;
        Ok(result.generated_terrain)
    }

    pub fn generate(
        &self,
        name: String,
        scenario: GameScenario,
        difficulty: GameLevelDifficulty,
        map: TileMap,
    ) -> Result<City, String> {
        City::from_map(name, scenario, difficulty, map, self.simulation_rng_seed)
    }
}
