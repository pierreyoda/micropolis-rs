use std::collections::HashMap;
use std::path::Path;

use quicksilver::{geom::Rectangle, geom::Vector, graphics::Image, Graphics, Result, Window};

use micropolis_rs_core::map::TileMap;

const TILE_SIZE: u16 = 16;
const TILES_ATLAS_ROWS: u16 = 16;
const TILES_ATLAS_COLUMNS: u16 = 60;
const TILES_ATLAS_COUNT: u16 = TILES_ATLAS_ROWS * TILES_ATLAS_COLUMNS;

#[derive(Clone)]
pub struct TilesRenderer {
    atlas: Image,
    tiles_size: Vector,
    tiles_regions: HashMap<u16, Rectangle>,
}

impl TilesRenderer {
    /// Load the 16x16 tiles corresponding to each tile type.
    pub async fn load_tiles<P: AsRef<Path>>(path: P, gfx: &Graphics) -> Result<Self> {
        let atlas = Image::load(gfx, path).await?;
        let mut tiles_regions = HashMap::new();
        let tiles_size = Vector::new(TILE_SIZE as f32, TILE_SIZE as f32);
        for tile_index in 0..TILES_ATLAS_COUNT {
            let tile_region = Rectangle::new(
                Vector::new(
                    (tile_index % TILES_ATLAS_ROWS * TILE_SIZE) as f32,
                    (tile_index / TILES_ATLAS_ROWS * TILE_SIZE) as f32,
                ),
                tiles_size,
            );
            tiles_regions.insert(tile_index, tile_region);
        }

        Ok(Self {
            atlas,
            tiles_size,
            tiles_regions,
        })
    }

    pub fn render(&self, gfx: &mut Graphics, map: &TileMap) -> Result<()> {
        let tiles = map.tiles();
        for (x, row) in tiles.iter().enumerate() {
            for (y, tile) in row.iter().enumerate() {
                let tile_region = match self.tiles_regions.get(&tile.get_type_raw()) {
                    Some(image) => image,
                    None => continue,
                };
                // TODO: cache tile position? in map tile or in renderer
                gfx.draw_subimage(
                    &self.atlas,
                    *tile_region,
                    Rectangle::new(
                        Vector::new((x as u16 * TILE_SIZE) as f32, (y as u16 * TILE_SIZE) as f32),
                        self.tiles_size,
                    ),
                )
            }
        }
        Ok(())
    }
}
