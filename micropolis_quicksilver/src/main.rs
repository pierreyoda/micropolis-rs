mod tiles;

// use std::panic;

use quicksilver::{
    geom::Vector,
    graphics::{Color, Font},
    input::Key,
    lifecycle::{run, Asset, Settings, State, Window},
    Future, Result,
};
use rand::rngs::OsRng;

use micropolis_rs_core::map::generator::{GeneratorCreateIsland, MapGenerator};
use micropolis_rs_core::map::{MapRectangle, TileMap};
use micropolis_rs_core::utils::Percentage;
use tiles::TilesRenderer;

struct MicropolisClient {
    rng: OsRng,
    terrain_generator: MapGenerator,
    just_generated: bool,
    test_map: TileMap,
    assets: Asset<(Font, TilesRenderer)>,
}

impl State for MicropolisClient {
    fn new() -> Result<Self> {
        println!("Initializing micropolis-rs...");
        let mut rng = OsRng;
        let terrain_generator_island_chance =
            GeneratorCreateIsland::Sometimes(Percentage::from_integer(50).unwrap());
        let terrain_generator = MapGenerator::with_options(terrain_generator_island_chance);
        let test_map = terrain_generator
            .random_map_terrain(&mut rng, &MapRectangle::new(120, 100))
            .expect("map generator error");
        let assets = Asset::new(
            Font::load("open-sans/OpenSans-Regular.ttf")
                .join(TilesRenderer::load_tiles("tiles.png"))
                .and_then(|(ui_font, tiles_renderer)| Ok((ui_font, tiles_renderer))),
        );
        Ok(MicropolisClient {
            rng,
            terrain_generator,
            just_generated: false,
            test_map,
            assets,
        })
    }

    fn update(&mut self, window: &mut Window) -> Result<()> {
        if window.keyboard()[Key::R].is_down() {
            // println!("Generating random terrain...");
            if self.just_generated {
                return Ok(());
            }
            self.test_map = self
                .terrain_generator
                .random_map_terrain(&mut self.rng, &self.test_map.bounds())
                .expect("map generator error");
            self.just_generated = true;
        } else {
            self.just_generated = false;
        }
        Ok(())
    }

    fn draw(&mut self, window: &mut Window) -> Result<()> {
        window.clear(Color::BLACK)?;
        let map = &self.test_map;
        self.assets
            .execute(|(_, tiles_renderer)| tiles_renderer.render(window, map))
    }
}

fn main() {
    // panic::set_hook(Box::new(console_error_panic_hook::hook));
    run::<MicropolisClient>(
        "micropolis-rs",
        Vector::new(800, 600),
        Settings {
            icon_path: Some("test.png"),
            ..Settings::default()
        },
    )
}
