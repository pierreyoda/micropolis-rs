use core::panic;
use std::{fs::File, io::Write, path::PathBuf};

use clap::Parser;
use serde::Serialize;
use serde_json;

use micropolis_rs_core::{
    map::{
        generator::{GeneratorCreateIsland, MapGenerator},
        tiles_type::TileType,
        MapRectangle,
    },
    utils::{random::MicropolisRandom, Percentage},
};

/// Program options.
#[derive(Parser, Debug)]
#[clap(
    version = "0.0.1",
    author = "pierreyoda <pierreyoda@users.noreply.github.com>"
)]
struct Opts {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Parser, Debug)]
enum SubCommand {
    #[clap(
        version = "0.1.0",
        author = "pierreyoda <pierreyoda@users.noreply.github.com>",
        name = "export-random-map"
    )]
    /// Generate a basic JSON TileMap (effectively a 2D **rows-first** array).
    GenerateBasicJsonTileMap(GenerateBasicJsonTileMapOptions),
}

/// A subcommand for controlling testing
#[derive(Parser, Debug)]
struct GenerateBasicJsonTileMapOptions {
    #[clap(long)]
    width: usize,
    #[clap(long)]
    height: usize,
}

#[derive(Debug, Serialize)]
struct ExportedTestTileMap {
    seed: i32,
    tiles_data: Vec<Vec<u16>>,
}

fn main() {
    let opts: Opts = Opts::parse();

    match opts.subcmd {
        SubCommand::GenerateBasicJsonTileMap(options) => {
            // prepare
            let mut rng = MicropolisRandom::from_random_system_seed();
            let dimensions = MapRectangle::new(options.width, options.height);

            // generate
            let generator = MapGenerator::with_options(GeneratorCreateIsland::Sometimes(
                Percentage::from_integer(10).unwrap(),
            ));
            let terrain = generator
                .random_map_terrain(&mut rng, 12345, &dimensions)
                .unwrap();
            let tiles = terrain.generated_terrain.tiles();

            // export
            let seed = rng.get_seed();
            let tiles_data: Vec<Vec<u16>> = tiles
                .iter()
                .map(|t| {
                    t.iter()
                        .map(|t| {
                            t.get_type()
                                .as_ref()
                                .unwrap_or(&TileType::Dirt)
                                .to_u16()
                                .unwrap()
                        })
                        .collect()
                })
                .collect();
            let exported = ExportedTestTileMap { seed, tiles_data };
            let json = serde_json::to_string(&exported).unwrap();

            // output
            let mut filepath = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
            filepath.push("./output/test-front-map.json");

            let mut file = match File::create(&filepath) {
                Err(why) => panic!("could not create file {}: {}", filepath.display(), why),
                Ok(file) => file,
            };
            match file.write_all(json.as_bytes()) {
                Err(why) => panic!("could not write to file {}: {}", filepath.display(), why),
                Ok(_) => println!("successfully wrote to file {}", filepath.display()),
            };
        }
    }
}
