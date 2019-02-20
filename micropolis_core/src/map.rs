pub mod generator;
pub mod tiles;
pub mod tiles_type;

use tiles_type::TileType;

pub type TileMap = Vec<Vec<TileType>>;

/// Stores the state of the entire map containing a Metropolis city.
#[derive(Clone, Debug)]
pub struct Map {
    /// The tiles of the map (assumed as rectangular).
    ///
    /// First dimension is the X (horizontal) axis, second dimension is the Y (vertical) axis.
    tiles: TileMap,
}

impl Map {
    fn with_dimensions(dimensions: &MapRect, uniform_type: TileType) -> Self {
        let tiles = vec![vec![uniform_type; dimensions.get_width()]; dimensions.get_height()];
        Map { tiles }
    }

    pub fn in_bounds(&self, position: &MapPosition) -> bool {
        if position.x < 0 || position.y < 0 || position.x >= self.tiles.len() as i32 {
            false
        } else {
            match self.tiles.first() {
                Some(first) => position.y < first.len() as i32,
                None => false,
            }
        }
    }

    pub fn get_bounds(&self) -> MapRect {
        MapRect {
            height: self.tiles.len(),
            width: match self.tiles.first() {
                Some(first) => first.len(),
                None => 0,
            },
        }
    }
}

/// Represents a position on a 2D map.
///
/// Uses signed integers to allow for offsetting.
#[derive(Clone, Debug, PartialEq, Eq)]
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
pub struct MapRect {
    width: usize,
    height: usize,
}

impl MapRect {
    pub fn new(width: usize, height: usize) -> Self {
        MapRect { width, height }
    }

    pub fn get_width(&self) -> usize {
        self.width
    }
    pub fn get_height(&self) -> usize {
        self.height
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

impl MapPositionOffset {
    pub fn apply(&self, position: &MapPosition) -> MapPosition {
        let (offset_x, offset_y) = self.offset();
        // TODO: test casting
        MapPosition {
            x: position.x + offset_x as i32,
            y: position.y + offset_y as i32,
        }
    }

    pub fn offset(&self) -> (i8, i8) {
        use MapPositionOffset::*;
        match self {
            None => (0, 0),
            NorthWest => (-1, 1),
            North => (0, 1),
            NorthEast => (1, 1),
            East => (1, 0),
            SouthEast => (1, -1),
            South => (0, -1),
            SouthWest => (-1, -1),
            West => (-1, 0),
        }
    }

    /// Get the direction rotated by 45 degrees clock-wise.
    pub fn rotated_45(&self) -> MapPositionOffset {
        use MapPositionOffset::*;
        match self {
            None => None,
            NothWest => North,
            North => NorthEast,
            NorthEast => East,
            East => SouthEast,
            SouthEast => South,
            South => SouthWest,
            SouthWest => West,
            West => NorthWest,
        }
    }

    pub fn rotated_180(&self) -> MapPositionOffset {
        use MapPositionOffset::*;
        match self {
            None => None,
            NothWest => SouthEast,
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
