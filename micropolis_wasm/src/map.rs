use rand::rngs::OsRng;
use wasm_bindgen::prelude::*;

use micropolis_rs_core::map::generator::{GeneratorCreateIsland, MapGenerator};
use micropolis_rs_core::map::{Map, MapRectangle, Tile};
use micropolis_rs_core::utils::Percentage;

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
    width: usize,
    height: usize,
) -> Result<Box<[u16]>, JsValue> {
    let mut rng = OsRng;
    let dimensions = MapRectangle::new(width, height);
    let result = wrapper.generator.random_map_terrain(&mut rng, &dimensions);
    if let Ok(map) = result {
        let tilemap = map.tiles();
        let tiles: Vec<u16> = tilemap
            .iter()
            .flat_map(|column| column.iter())
            .map(|tile| tile.get_type_raw())
            .collect();
        Ok(tiles.into_boxed_slice())
    } else {
        Err(JsValue::from_str(&result.err().unwrap()[..]))
    }
}
