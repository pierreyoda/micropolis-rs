use std::{
    cmp::max,
    cmp::min,
    fmt,
    ops::{Add, AddAssign, Div, Mul, Shl, Shr, Sub, SubAssign},
};

use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::abs;
use num_traits::{FromPrimitive as FromPrimitiveTrait, ToPrimitive as ToPrimitiveTrait};
use serde::{Deserialize, Serialize};

use crate::utils::random::MicropolisRandom;

const DIRECTION_GD_TAB: [usize; 13] = [0, 3, 2, 1, 3, 4, 5, 7, 6, 5, 7, 8, 1];

/// Represents a position on a 2D map.
///
/// Uses signed integers to allow for offsetting.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
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

    pub fn new_random(rng: &mut MicropolisRandom, bounds: &MapRectangle) -> Self {
        Self {
            x: rng.get_random(bounds.width as i16) as i32,
            y: rng.get_random(bounds.height as i16) as i32,
        }
    }

    pub fn get_x(&self) -> i32 {
        self.x
    }
    pub fn get_y(&self) -> i32 {
        self.y
    }

    pub fn set_x(&mut self, x: i32) {
        self.x = x;
    }
    pub fn set_y(&mut self, y: i32) {
        self.y = y;
    }

    pub fn as_tuple(&self) -> (i32, i32) {
        (self.x, self.y)
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

    pub fn unitary(&self, neutral_value: i32) -> MapPosition {
        Self {
            x: if self.x > 0 {
                1
            } else if self.x < 0 {
                -1
            } else {
                neutral_value
            },
            y: if self.y > 0 {
                1
            } else if self.y < 0 {
                -1
            } else {
                neutral_value
            },
        }
    }

    pub fn absolute(&self) -> MapPosition {
        Self {
            x: abs(self.x),
            y: abs(self.y),
        }
    }

    pub fn minimum_axis(&self) -> i32 {
        min(self.x, self.y)
    }
    pub fn maximum_axis(&self) -> i32 {
        max(self.x, self.y)
    }

    pub fn axis_equalities_with(&self, other: &MapPosition) -> (bool, bool) {
        (self.x == other.x, self.y == other.y)
    }

    /// Compute the Manhattan distance between the two positions.
    pub fn distance_with(&self, other: &MapPosition) -> u32 {
        let diff = (*self - *other).absolute();
        (diff.x + diff.y) as u32
    }

    /// Compute the distance & direction to get from this position to the given destination.
    pub fn direction_towards(&self, destination: &MapPosition) -> (u32, MapPositionOffset) {
        let diff = *destination - *self;
        let mut z = match diff.as_tuple() {
            (x, y) if x < 0 => {
                if y < 0 {
                    11
                } else {
                    8
                }
            }
            (_, y) => {
                if y < 0 {
                    2
                } else {
                    5
                }
            }
        };

        let (diff_x, diff_y) = diff.absolute().as_tuple();
        z = if diff_x * 2 < diff_y {
            z + 1
        } else if diff_y * 2 < diff_y {
            // TODO: always false!
            z - 1
        } else if z < 0 || z > 12 {
            0
        } else {
            z
        };

        (
            (diff_x + diff_y) as u32,
            MAP_POSITION_DIRECTIONS[DIRECTION_GD_TAB[z]],
        )
    }
}

impl From<(i16, i16)> for MapPosition {
    fn from(val: (i16, i16)) -> Self {
        MapPosition {
            x: val.0 as i32,
            y: val.1 as i32,
        }
    }
}

impl From<(i32, i32)> for MapPosition {
    fn from(val: (i32, i32)) -> Self {
        MapPosition { x: val.0, y: val.1 }
    }
}

impl From<(usize, usize)> for MapPosition {
    fn from(val: (usize, usize)) -> Self {
        MapPosition {
            x: val.0 as i32,
            y: val.1 as i32,
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

impl AddAssign for MapPosition {
    fn add_assign(&mut self, rhs: MapPosition) {
        self.x += rhs.x;
        self.y += rhs.y;
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

impl SubAssign for MapPosition {
    fn sub_assign(&mut self, rhs: MapPosition) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl Shl<usize> for MapPosition {
    type Output = Self;

    fn shl(self, rhs: usize) -> Self::Output {
        Self {
            x: self.x << rhs,
            y: self.y << rhs,
        }
    }
}

impl Shr<usize> for MapPosition {
    type Output = Self;

    fn shr(self, rhs: usize) -> Self::Output {
        Self {
            x: self.x >> rhs,
            y: self.y >> rhs,
        }
    }
}

impl Mul<i32> for MapPosition {
    type Output = Self;

    fn mul(self, rhs: i32) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl Div<i32> for MapPosition {
    type Output = Self;

    fn div(self, rhs: i32) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl fmt::Display for MapPosition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "MapPosition(x={}, y={})", self.x, self.y)
    }
}

/// Describes the width and height of a rectangle section of a Metropolis city.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
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
#[derive(Clone, Copy, Debug, PartialEq, Eq, FromPrimitive, ToPrimitive, Serialize, Deserialize)]
pub enum MapPositionOffset {
    None = 0,
    NorthWest = 8,
    North = 1,
    NorthEast = 2,
    East = 3,
    SouthEast = 4,
    South = 5,
    SouthWest = 6,
    West = 7,
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
        self.rotated_45_times(1)
    }

    /// Get the direction rotated by 45 degrees `x` times.
    pub fn rotated_45_times(&self, count: u8) -> MapPositionOffset {
        use MapPositionOffset::*;
        let direction_number = *self as u8;
        match 1 + ((direction_number - 1 + count) & 7) {
            0 => None,
            1 => North,
            2 => NorthEast,
            3 => East,
            4 => SouthEast,
            5 => South,
            6 => SouthWest,
            7 => West,
            8 => NorthWest,
            _ => unreachable!(),
        }
    }

    /// Get the direction rotated by 90 degrees clock-wise.
    pub fn rotated_90(&self) -> MapPositionOffset {
        use MapPositionOffset::*;
        match self {
            None => None,
            NorthWest => NorthEast,
            North => East,
            NorthEast => SouthEast,
            East => South,
            SouthEast => SouthWest,
            South => West,
            SouthWest => NorthWest,
            West => North,
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

    pub fn from_usize(value: usize) -> Option<Self> {
        FromPrimitiveTrait::from_usize(value)
    }

    pub fn to_usize(&self) -> Option<usize> {
        match self {
            MapPositionOffset::None => None,
            _ => ToPrimitiveTrait::to_usize(self),
        }
    }

    pub fn is_cardinal(&self) -> bool {
        use MapPositionOffset::*;
        matches!(self, North | East | South | West)
    }
}

#[cfg(test)]
mod tests {
    use super::{MapPosition, MapPositionOffset, MapRectangle};

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

    #[test]
    fn test_cardinal_directions() {
        assert_eq!(MapPositionOffset::North.is_cardinal(), true);
        assert_eq!(MapPositionOffset::East.is_cardinal(), true);
        assert_eq!(MapPositionOffset::South.is_cardinal(), true);
        assert_eq!(MapPositionOffset::West.is_cardinal(), true);
        assert_eq!(MapPositionOffset::NorthEast.is_cardinal(), false);
        assert_eq!(MapPositionOffset::NorthWest.is_cardinal(), false);
        assert_eq!(MapPositionOffset::SouthEast.is_cardinal(), false);
        assert_eq!(MapPositionOffset::SouthWest.is_cardinal(), false);
    }
}
