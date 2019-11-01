mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc`
// as the global allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// use micropolis_rs_core::map::{Map, MapRectangle};
// use micropolis_rs_core::map::generator::{MapGenerator};

#[wasm_bindgen]
pub fn greet() -> String {
    "Hello WebAssembly!".into()
}

// #[wasm_bindgen]
// pub fn instantiate_random_map_generator() -> MapGenerator {

// }

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    utils::set_panic_hook();

    Ok(())
}
