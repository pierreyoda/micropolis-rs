use serde::Serialize;

pub mod generator;
pub mod tiles;
pub mod tiles_type;

pub use tiles::Tile;
pub use tiles_type::TileType;

pub const WORLD_WIDTH: usize = 120;
pub const WORLD_HEIGHT: usize = 100;

pub type MapData<T> = Vec<Vec<T>>;

/// Generic class for maps in the Micropolis game.
///
/// A map is assumed to cover a 2D grid of #WORLD_W times #WORLD_H positions.
/// A block of positions may be clustered, and represented by a single data
/// value.
#[derive(Clone, Debug, Serialize)]
pub struct Map<T> {
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
    pub(crate) data: MapData<T>,
}

impl<T> Map<T> {
    pub fn in_bounds(&self, position: &MapPosition) -> bool {
        if position.x < 0 || position.y < 0 || position.x >= self.data.len() as i32 {
            false
        } else {
            match self.data.first() {
                Some(first) => position.y < first.len() as i32,
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

    pub fn get_tile_at(&self, position: &MapPosition) -> Option<&T> {
        if self.in_bounds(position) {
            Some(
                self.data
                    .get(position.x as usize)?
                    .get(position.y as usize)?,
            )
        } else {
            None
        }
    }

    pub fn set_tile_at(&mut self, position: &MapPosition, tile: T) -> bool {
        if let Some(column) = self.data.get_mut(position.x as usize) {
            if let Some(cell) = column.get_mut(position.y as usize) {
                std::mem::replace(cell, tile);
                return true;
            }
        }
        false
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
        Ok(Map { data: tilemap })
    }
}

/// Represents a position on a 2D map.
///
/// Uses signed integers to allow for offsetting.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct MapPosition {
    /// Horizontal position on the map.
    x: i32,
    /// Vertical position on the map.
    y: i32,
}

impl MapPosition {
    pub fn new(x: i32, y: i32) -> Self {
        MapPosition { x, y }
    }
}

/// Describes the width and height of a rectangle section of a Metropolis city.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MapRectangle {
    width: usize,
    height: usize,
}

impl MapRectangle {
    pub fn new(width: usize, height: usize) -> Self {
        MapRectangle { width, height }
    }

    pub fn get_width(&self) -> usize {
        self.width
    }
    pub fn get_height(&self) -> usize {
        self.height
    }

    pub fn is_inside(&self, position: &MapPosition) -> bool {
        0 <= position.x
            && position.x < self.width as i32
            && 0 <= position.y
            && position.y < self.height as i32
    }
}

/// Describes a tile position relative to an adjacent tile.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MapPositionOffset {
    None,
    NorthWest,
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
}

pub const MAP_POSITION_DIRECTIONS: [MapPositionOffset; 8] = [
    MapPositionOffset::North,
    MapPositionOffset::NorthEast,
    MapPositionOffset::East,
    MapPositionOffset::SouthEast,
    MapPositionOffset::South,
    MapPositionOffset::SouthWest,
    MapPositionOffset::West,
    MapPositionOffset::NorthWest,
];

impl MapPositionOffset {
    /// Apply the relative position offset of this instance to an absolute position.
    pub fn apply(&self, position: &MapPosition) -> MapPosition {
        let (offset_x, offset_y) = self.offset();
        MapPosition {
            x: position.x + offset_x as i32,
            y: position.y + offset_y as i32,
        }
    }

    /// Apply the relative position offset of this instance to an absolute position,
    /// constrained to the given bounds.
    pub fn apply_with_bounds(
        &self,
        position: &MapPosition,
        bounds: &MapRectangle,
    ) -> Option<MapPosition> {
        let applied = self.apply(position);
        if 0 <= applied.x
            && applied.x < bounds.width as i32
            && 0 <= applied.y
            && applied.y <= bounds.height as i32
        {
            Some(applied)
        } else {
            None
        }
    }

    /// Get the relative position offset described by this instance.
    pub fn offset(&self) -> (i8, i8) {
        use MapPositionOffset::*;
        match self {
            None => (0, 0),
            NorthWest => (-1, -1),
            North => (0, -1),
            NorthEast => (1, -1),
            East => (1, 0),
            SouthEast => (1, 1),
            South => (0, 1),
            SouthWest => (-1, 1),
            West => (-1, 0),
        }
    }

    /// Get the direction rotated by 45 degrees clock-wise.
    pub fn rotated_45(&self) -> MapPositionOffset {
        use MapPositionOffset::*;
        match self {
            None => None,
            NorthWest => North,
            North => NorthEast,
            NorthEast => East,
            East => SouthEast,
            SouthEast => South,
            South => SouthWest,
            SouthWest => West,
            West => NorthWest,
        }
    }

    /// Get the direction rotated by 180 degrees clock-wise.
    pub fn rotated_180(&self) -> MapPositionOffset {
        use MapPositionOffset::*;
        match self {
            None => None,
            NorthWest => SouthEast,
            North => South,
            NorthEast => SouthWest,
            East => West,
            SouthEast => NorthWest,
            South => North,
            SouthWest => NorthEast,
            West => East,
        }
    }
}
