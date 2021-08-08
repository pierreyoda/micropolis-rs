use wasm_bindgen::prelude::*;

use micropolis_rs_core::{
    city::{
        generator::{CityGenerator, CityGeneratorBuilder, CityMapGeneratorOptions},
        City,
    },
    game::{GameLevelDifficulty, GameScenario},
    map::{generator::GeneratorCreateIsland, MapRectangle},
    utils::Percentage,
};

use crate::map::WebTileMap;

#[wasm_bindgen]
pub struct WebCityGeneratorBuilder {
    builder: CityGeneratorBuilder,
}

#[wasm_bindgen]
impl WebCityGeneratorBuilder {
    pub fn with_simulation_rng_seed(mut self, simulation_seed: i32) -> Self {
        self.builder = self.builder.with_simulation_rng_seed(simulation_seed);
        self
    }

    pub fn with_city_map_generator_options(
        mut self,
        map_width: usize,
        map_height: usize,
        create_islands: bool,
    ) -> Self {
        self.builder = self
            .builder
            .with_city_map_generator_options(CityMapGeneratorOptions {
                dimensions: MapRectangle::new(map_width, map_height),
                create_island: if create_islands {
                    GeneratorCreateIsland::Sometimes(Percentage::from_integer(10).unwrap())
                } else {
                    GeneratorCreateIsland::Never
                },
            });
        self
    }

    pub fn build(self) -> Result<WebCityGenerator, JsValue> {
        match self.builder.build() {
            Ok(generator) => Ok(WebCityGenerator { generator }),
            Err(why) => Err(JsValue::from_str(&why[..])),
        }
    }
}

#[wasm_bindgen]
pub struct WebCityGenerator {
    generator: CityGenerator,
}

#[wasm_bindgen]
impl WebCityGenerator {
    pub fn build_random_map(&self, seed: i32) -> Result<WebTileMap, JsValue> {
        match self.generator.build_random_map(seed) {
            Ok(random_map) => Ok(WebTileMap::from_value(random_map)),
            Err(why) => Err(JsValue::from_str(&why[..])),
        }
    }

    pub fn generate(&self, name: String, map_wrapper: WebTileMap) -> Result<WebCity, JsValue> {
        match self.generator.generate(
            name,
            GameScenario::None,
            GameLevelDifficulty::Normal,
            map_wrapper.extract_value(),
        ) {
            Ok(city) => Ok(WebCity { city }),
            Err(why) => Err(JsValue::from_str(&why[..])),
        }
    }
}

/// `City` web wrapper.
#[wasm_bindgen]
pub struct WebCity {
    city: City,
}

#[wasm_bindgen]
pub fn create_city_generator_builder() -> WebCityGeneratorBuilder {
    WebCityGeneratorBuilder {
        builder: CityGeneratorBuilder::new(),
    }
}
