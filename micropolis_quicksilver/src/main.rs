mod tiles;

use quicksilver::{
    graphics::Color, input::Event, input::Key, run, Graphics, Input, Result, Settings, Window,
};
use rand::rngs::OsRng;

use micropolis_rs_core::map::generator::{GeneratorCreateIsland, MapGenerator};
use micropolis_rs_core::map::MapRectangle;
use micropolis_rs_core::utils::Percentage;
use tiles::TilesRenderer;

async fn micropolis_client(window: Window, mut gfx: Graphics, mut input: Input) -> Result<()> {
    println!("Initializing micropolis-rs...");

    // init terrain generator
    let mut rng = OsRng;
    let terrain_generator_island_chance =
        GeneratorCreateIsland::Sometimes(Percentage::from_integer(50).unwrap());
    let terrain_generator = MapGenerator::with_options(terrain_generator_island_chance);
    let mut test_map = terrain_generator
        .random_map_terrain(&mut rng, &MapRectangle::new(120, 100))
        .expect("map generator error");
    let mut just_generated = false;

    // init tiles renderer
    let tiles_renderer = TilesRenderer::load_tiles("img/tiles.png", &gfx).await?;

    // load TTF font
    // let ttf = VectorFont::load("open-sans/OpenSans-Regular.ttf").await?;
    // let mut font = ttf.to_renderer(&gfx, 72.0);

    loop {
        while let Some(event) = input.next_event().await {
            match event {
                Event::KeyboardInput(key) if key.is_down() && key.key() == Key::R => {
                    if just_generated {
                        return Ok(());
                    }
                    test_map = terrain_generator
                        .random_map_terrain(&mut rng, &test_map.bounds())
                        .expect("map generator error");
                }
                _ => just_generated = false,
            }
        }

        gfx.clear(Color::BLACK);
        tiles_renderer.render(&mut gfx, &test_map)?;
        gfx.present(&window)?;
    }
}

fn main() {
    // let root_path = env::current_dir().unwrap();
    // let icon_path = format!("{:?}/test.png", root_path);
    run(
        Settings {
            title: "micropolis-rs",
            // icon_path: Some(&icon_path[..]),
            ..Settings::default()
        },
        micropolis_client,
    )
}
