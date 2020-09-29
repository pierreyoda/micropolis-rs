use std::{
    fmt,
    ops::{Add, Sub},
};

/// Represents a position on a 2D map.
///
/// Uses signed integers to allow for offsetting.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub struct MapPosition {
    /// Horizontal position on the map.
    pub(super) x: i32,
    /// Vertical position on the map.
    pub(super) y: i32,
}

impl MapPosition {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn with_offset(&self, offset_x: i8, offset_y: i8) -> Self {
        Self {
            x: self.x + offset_x as i32,
            y: self.y + offset_y as i32,
        }
    }

    pub fn with_x_offset(&self, offset_x: i8) -> Self {
        Self {
            x: self.x + offset_x as i32,
            y: self.y,
        }
    }

    pub fn with_y_offset(&self, offset_y: i8) -> Self {
        Self {
            x: self.x,
            y: self.y + offset_y as i32,
        }
    }
}

impl Add for MapPosition {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for MapPosition {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl fmt::Display for MapPosition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "MapPosition(x={}, y={})", self.x, self.y)
    }
}

/// Describes the width and height of a rectangle section of a Metropolis city.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MapRectangle {
    pub(super) width: usize,
    pub(super) height: usize,
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
    pub fn get_tuple(&self) -> (i32, i32) {
        (self.width as i32, self.height as i32)
    }

    pub fn is_inside(&self, position: &MapPosition) -> bool {
        0 <= position.x
            && position.x < self.width as i32
            && 0 <= position.y
            && position.y < self.height as i32
    }

    pub fn is_contained(&self, top_left: &MapPosition, size: &Self) -> bool {
        let (width, height) = (size.width as i32, size.height as i32);
        top_left.x >= 0
            || top_left.x + width <= self.width as i32
            || top_left.y >= 0
            || top_left.y + height <= self.height as i32
    }
}

impl Into<MapRectangle> for (usize, usize) {
    fn into(self) -> MapRectangle {
        MapRectangle::new(self.0, self.1)
    }
}

impl fmt::Display for MapRectangle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "MapPosition(w={}, h={})", self.width, self.height)
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

#[cfg(test)]
mod tests {
    use super::{MapPosition, MapRectangle};

    #[test]
    fn test_position_addition() {
        assert_eq!(
            MapPosition::new(3, 2) + MapPosition::new(-1, 12),
            MapPosition::new(2, 14),
        );
        assert_eq!(
            MapPosition::new(13, -5) + MapPosition::new(0, -1),
            MapPosition::new(13, -6),
        );
    }

    #[test]
    fn test_position_substraction() {
        assert_eq!(
            MapPosition::new(3, 2) - MapPosition::new(-1, 12),
            MapPosition::new(4, -10),
        );
        assert_eq!(
            MapPosition::new(13, -5) - MapPosition::new(0, -1),
            MapPosition::new(13, -4),
        );
    }

    #[test]
    fn test_position_offsetting() {
        assert_eq!(
            MapPosition::new(7, -3).with_offset(3, 3),
            MapPosition::new(10, 0),
        );
        assert_eq!(
            MapPosition::new(7, -3).with_x_offset(-1),
            MapPosition::new(6, -3),
        );
        assert_eq!(
            MapPosition::new(7, -3).with_y_offset(2),
            MapPosition::new(7, -1),
        );
    }

    #[test]
    fn test_rectangle_tuple() {
        assert_eq!(MapRectangle::new(120, 100).get_tuple(), (120, 100),);
    }
}
