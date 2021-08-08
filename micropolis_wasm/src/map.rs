use wasm_bindgen::prelude::*;

use micropolis_rs_core::map::{Map, MapRectangle, Tile, TileMap};
use micropolis_rs_core::utils::Percentage;
use micropolis_rs_core::{
    map::generator::{GeneratorCreateIsland, MapGenerator},
    utils::random::MicropolisRandom,
};

/// Web wrapper for a `TileMap`.
#[wasm_bindgen]
pub struct WebTileMap {
    map: TileMap,
}

impl WebTileMap {
    pub fn from_value(map: TileMap) -> Self {
        Self { map }
    }

    pub fn extract_value(self) -> TileMap {
        self.map
    }
}

#[wasm_bindgen]
impl WebTileMap {
    pub fn get_tiles(&self) -> JsValue {
        let tilemap = self.map.tiles();
        JsValue::from_serde(&tilemap).unwrap()
    }
}

/// Wrapper for the new game screen where
/// one or more map(s) can be randomly generated.
///
/// Each generated map can be looped back to.
#[wasm_bindgen]
pub struct WebMapGenerator {
    generator: MapGenerator,
}

#[wasm_bindgen]
pub fn create_terrain_generator() -> WebMapGenerator {
    WebMapGenerator {
        generator: MapGenerator::with_options(GeneratorCreateIsland::Sometimes(
            Percentage::from_integer(10).unwrap(),
        )),
    }
}

#[wasm_bindgen]
pub fn generate_new_map(
    wrapper: WebMapGenerator,
    seed: i32,
    width: usize,
    height: usize,
) -> Result<JsValue, JsValue> {
    let mut rng = MicropolisRandom::from_random_system_seed();
    let dimensions = MapRectangle::new(width, height);
    let result = wrapper
        .generator
        .random_map_terrain(&mut rng, seed, &dimensions);
    if let Ok(generated) = result {
        let tilemap = generated.generated_terrain.tiles();
        Ok(JsValue::from_serde(&tilemap).unwrap())
    } else {
        Err(JsValue::from_str(&result.err().unwrap()[..]))
    }
}
