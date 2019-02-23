mod tiles;

// use std::panic;

use quicksilver::{
    geom::Vector,
    graphics::Color,
    input::Key,
    lifecycle::{run, Asset, Settings, State, Window},
    Result,
};
use rand::rngs::OsRng;

use micropolis_rs_core::map::generator::{GeneratorCreateIsland, MapGenerator, Percentage};
use micropolis_rs_core::map::{Map, MapRect};
use tiles::TilesRenderer;

struct MicropolisClient {
    rng: OsRng,
    terrain_generator: MapGenerator,
    just_generated: bool,
    test_map: Map,
    tiles_renderer: Asset<TilesRenderer>,
}

impl State for MicropolisClient {
    fn new() -> Result<Self> {
        println!("Initializing micropolis-rs...");
        let mut rng = OsRng::new().expect("cannot create OS RNG");
        let terrain_generator_island_chance =
            GeneratorCreateIsland::Sometimes(Percentage::from_integer(50).unwrap());
        let terrain_generator = MapGenerator::with_options(terrain_generator_island_chance);
        let test_map = terrain_generator.random_map_terrain(&mut rng, &MapRect::new(80, 80));
        let tiles_renderer = Asset::new(TilesRenderer::load_tiles("tiles.png"));
        Ok(MicropolisClient {
            rng,
            terrain_generator,
            just_generated: false,
            test_map,
            tiles_renderer,
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
                .random_map_terrain(&mut self.rng, &self.test_map.get_bounds());
            self.just_generated = true;
        } else {
            self.just_generated = false;
        }
        Ok(())
    }

    fn draw(&mut self, window: &mut Window) -> Result<()> {
        window.clear(Color::BLACK)?;
        let map = &self.test_map;
        self.tiles_renderer
            .execute(|renderer| renderer.render(window, map))
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
