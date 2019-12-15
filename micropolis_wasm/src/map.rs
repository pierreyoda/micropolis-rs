use rand::rngs::OsRng;
use wasm_bindgen::prelude::*;

use micropolis_rs_core::map::generator::{GeneratorCreateIsland, MapGenerator};
use micropolis_rs_core::map::{Map, MapRectangle};
use micropolis_rs_core::utils::Percentage;

/// Wrapper for the new game screen where
/// one or more map(s) can be randomly generated.
///
/// Each generated map can be looped back to.
#[wasm_bindgen]
pub struct WebMapGenerator {
    generator: MapGenerator,
    maps: Vec<Map>,
}

#[wasm_bindgen]
pub fn create_terrain_generator() -> WebMapGenerator {
    WebMapGenerator {
        generator: MapGenerator::with_options(GeneratorCreateIsland::Sometimes(
            Percentage::from_integer(10).unwrap(),
        )),
        maps: vec![],
    }
}

#[wasm_bindgen]
pub fn generate_new_map(
    mut wrapper: WebMapGenerator,
    width: usize,
    height: usize,
) -> Option<String> {
    let mut rng = OsRng;
    let dimensions = MapRectangle::new(width, height);
    match wrapper.generator.random_map_terrain(&mut rng, &dimensions) {
        Ok(new_map) => {
            wrapper.maps.push(new_map);
            None
        }
        Err(why) => Some(why),
    }
}
