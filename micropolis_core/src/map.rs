use serde::Serialize;

pub mod animations;
pub mod buildings;
pub mod connect;
pub mod generator;
pub mod position;
pub mod tiles;
pub mod tiles_type;
pub mod tools;

pub use position::*;
pub use tiles::Tile;
pub use tiles_type::TileType;

use self::tiles::TILE_LOW_MASK;

pub const WORLD_WIDTH: usize = 120;
pub const WORLD_HEIGHT: usize = 100;

pub type MapData<T> = Vec<Vec<T>>;

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub enum MapClusteringStrategy {
    BlockSize1,
    BlockSize2,
    BlockSize4,
    BlockSize8,
}

impl MapClusteringStrategy {
    pub fn block_size(&self) -> usize {
        use MapClusteringStrategy::*;
        match self {
            BlockSize1 => 1,
            BlockSize2 => 2,
            BlockSize4 => 4,
            BlockSize8 => 8,
        }
    }

    pub fn transform(&self, position: &MapPosition) -> MapPosition {
        let block_size = self.block_size();
        MapPosition::new(
            (position.x as usize / block_size) as i32,
            (position.y as usize / block_size) as i32,
        )
    }
}

/// Generic class for maps in the Micropolis game.
///
/// A map is assumed to cover a 2D grid of #WORLD_W times #WORLD_H positions.
/// A block of positions may be clustered, and represented by a single data
/// value.
#[derive(Clone, Debug, Serialize)]
pub struct Map<T> {
    /// Blocks clustering strategy.
    clustering_strategy: MapClusteringStrategy,
    /// The internal data structure storing the tiles of the map (assumed as rectangular).
    ///
    /// First dimension is the X (horizontal) axis, second dimension is the Y (vertical) axis:
    ///
    /// (0, 0) +-----------> (W, 0)
    ///        |           X
    ///        |
    ///        |           |
    ///        v Y        -+
    ///       (0, H)       (W, H)
    data: MapData<T>,
}

impl<T: Clone> Map<T> {
    pub fn with_data(data: MapData<T>, clustering_strategy: MapClusteringStrategy) -> Self {
        Self {
            data,
            clustering_strategy,
        }
    }

    pub fn in_bounds(&self, position: &MapPosition) -> bool {
        let transformed = self.clustering_strategy.transform(position);
        if transformed.x < 0 || transformed.y < 0 || transformed.x >= self.data.len() as i32 {
            false
        } else {
            match self.data.first() {
                Some(first) => transformed.y < first.len() as i32,
                None => false,
            }
        }
    }

    pub fn bounds(&self) -> MapRectangle {
        MapRectangle {
            width: self.data.len(),
            height: match self.data.first() {
                Some(first) => first.len(),
                None => 0,
            },
        }
    }

    pub fn tiles(&self) -> &MapData<T> {
        &self.data
    }

    pub fn tiles_mut(&mut self) -> &mut MapData<T> {
        &mut self.data
    }

    pub fn get_tile_at(&self, position: &MapPosition) -> Option<&T> {
        if self.in_bounds(position) {
            let transformed = self.clustering_strategy.transform(position);
            Some(
                self.data
                    .get(transformed.x as usize)?
                    .get(transformed.y as usize)?,
            )
        } else {
            None
        }
    }

    pub fn set_tile_at(&mut self, position: &MapPosition, tile: T) -> bool {
        let transformed = self.clustering_strategy.transform(position);
        if let Some(column) = self.data.get_mut(transformed.x as usize) {
            if let Some(cell) = column.get_mut(transformed.y as usize) {
                *cell = tile;
                return true;
            }
        }
        false
    }

    pub fn get_neighboring_tile_at(
        &self,
        position: &MapPosition,
        direction: &MapPositionOffset,
        default_if_out_of_bounds: T,
    ) -> T {
        if direction.is_cardinal() {
            if let Some(neighboring_position) =
                direction.apply_with_bounds(position, &self.bounds())
            {
                self.get_tile_at(&neighboring_position)
                    .or(Some(&default_if_out_of_bounds))
                    .cloned()
                    .unwrap()
            } else {
                default_if_out_of_bounds
            }
        } else {
            default_if_out_of_bounds
        }
    }
}

pub type TileMap = Map<Tile>;

impl Map<Tile> {
    pub fn tilemap_with_dimensions(
        dimensions: &MapRectangle,
        uniform_type: TileType,
    ) -> Result<Self, String> {
        let tilemap =
            vec![vec![Tile::from_type(uniform_type)?; dimensions.height]; dimensions.width];
        Ok(Map {
            data: tilemap,
            clustering_strategy: MapClusteringStrategy::BlockSize8,
        })
    }

    pub fn get_tile_char_at(&self, position: &MapPosition) -> Option<u16> {
        self.get_tile_at(position)
            .and_then(|t| Some(t.get_raw() & TILE_LOW_MASK))
    }
}
