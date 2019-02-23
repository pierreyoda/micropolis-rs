use std::collections::HashMap;
use std::path::Path;

use quicksilver::Future;
use quicksilver::{
    geom::Rectangle,
    graphics::{Background::Img, Image},
    lifecycle::Window,
    Error, Result,
};

use micropolis_rs_core::map::Map;

const TILE_SIZE: u16 = 16;
const TILES_ATLAS_ROWS: u16 = 16;
const TILES_ATLAS_COLUMNS: u16 = 60;
const TILES_ATLAS_COUNT: u16 = TILES_ATLAS_ROWS * TILES_ATLAS_COLUMNS;

#[derive(Clone, Debug)]
pub struct TilesRenderer {
    tiles_atlas: HashMap<u16, Image>,
}

impl TilesRenderer {
    /// Load the 16x16 tiles corresponding to each tile type.
    pub fn load_tiles<P: AsRef<Path>>(path: P) -> impl Future<Item = Self, Error = Error> {
        Image::load(path).map(|atlas_image| {
            let tile_size = (TILE_SIZE, TILE_SIZE);
            let mut tiles_atlas = HashMap::new();
            for tile_index in 0..TILES_ATLAS_COUNT {
                let tile_region = Rectangle::new(
                    (
                        tile_index % TILES_ATLAS_ROWS * TILE_SIZE,
                        tile_index / TILES_ATLAS_ROWS * TILE_SIZE,
                    ),
                    tile_size,
                );
                let tile_image = atlas_image.subimage(tile_region);
                tiles_atlas.insert(tile_index, tile_image);
            }
            TilesRenderer { tiles_atlas }
        })
    }

    pub fn render(&self, window: &mut Window, map: &Map) -> Result<()> {
        let tiles = map.tiles();
        for (x, row) in tiles.iter().enumerate() {
            for (y, tile) in row.iter().enumerate() {
                let tile_image = match self.tiles_atlas.get(&tile.get_type_raw()) {
                    Some(image) => image,
                    None => continue,
                };
                // TODO: cache tile position? in map tile or in renderer
                window.draw(
                    &Rectangle::new(
                        (x as u16 * TILE_SIZE, y as u16 * TILE_SIZE),
                        (TILE_SIZE, TILE_SIZE),
                    ),
                    Img(tile_image),
                )
            }
        }
        Ok(())
    }
}
