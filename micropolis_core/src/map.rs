pub mod generator;
pub mod tiles;
pub mod tiles_type;

pub use tiles::Tile;
pub use tiles_type::TileType;

pub type TileMap = Vec<Vec<Tile>>;

/// Stores the state of the entire map containing a Metropolis city.
#[derive(Clone, Debug)]
pub struct Map {
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
    tilemap: TileMap,
}

impl Map {
    pub fn with_dimensions(
        dimensions: &MapRectangle,
        uniform_type: TileType,
    ) -> Result<Self, String> {
        let tilemap = vec![
            vec![Tile::from_type(uniform_type)?; dimensions.get_width()];
            dimensions.get_height()
        ];
        Ok(Map { tilemap })
    }

    pub fn in_bounds(&self, position: &MapPosition) -> bool {
        if position.x < 0 || position.y < 0 || position.x >= self.tilemap.len() as i32 {
            false
        } else {
            match self.tilemap.first() {
                Some(first) => position.y < first.len() as i32,
                None => false,
            }
        }
    }

    pub fn bounds(&self) -> MapRectangle {
        MapRectangle {
            height: self.tilemap.len(),
            width: match self.tilemap.first() {
                Some(first) => first.len(),
                None => 0,
            },
        }
    }

    pub fn tiles(&self) -> &TileMap {
        &self.tilemap
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
    pub fn apply_with_bounds(&self, position: &MapPosition, bounds: &MapRectangle) -> MapPosition {
        let applied = self.apply(position);
        if 0 <= applied.x
            && applied.x < bounds.width as i32
            && 0 <= applied.y
            && applied.y <= bounds.height as i32
        {
            applied
        } else {
            position.clone()
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
