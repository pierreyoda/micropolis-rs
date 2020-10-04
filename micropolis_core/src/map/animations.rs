use std::path::PathBuf;

use loader::{load_sequences_from_file, TileAnimatorSequences};

use super::{
    tiles::TILE_ALL_BITS, tiles::TILE_ANIM_BIT, tiles::TILE_LOW_MASK, Tile, TileMap, TileType,
};

mod loader;

pub struct TileMapAnimator {
    sequences: TileAnimatorSequences,
}

impl TileMapAnimator {
    pub fn load() -> Result<Self, String> {
        let mut filepath = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        filepath.pop();
        filepath.push("./res/animations_sequences.txt");
        Ok(Self {
            sequences: load_sequences_from_file(filepath)?,
        })
    }

    pub fn animate_world(&self, map: &mut TileMap) -> Result<(), String> {
        for column in map.tiles_mut().iter_mut() {
            for tile in column.iter_mut() {
                let mut tile_raw = tile.get_raw();
                if tile_raw & TILE_ANIM_BIT != 0x00 {
                    let tile_flags = tile_raw & TILE_ALL_BITS;
                    tile_raw &= TILE_LOW_MASK;
                    tile_raw = *self.sequences.get(tile_raw as usize).ok_or(format!(
                        "TileAnimator.animate_world: out of bounds tile value {:0>4X}",
                        tile_raw
                    ))?;
                    tile_raw |= tile_flags;
                    *tile = Tile::from_raw(tile_raw)?;
                }
            }
        }
        Ok(())
    }

    pub fn next_animated_tile(&self, index: usize) -> Option<TileType> {
        if index >= self.sequences.len() {
            None
        } else {
            TileType::from_u16(*self.sequences.get(index)?)
        }
    }
}
