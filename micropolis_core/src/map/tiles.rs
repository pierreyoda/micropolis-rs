use serde::Serialize;
use std::{fmt, rc::Rc};

use super::tiles_type::{TileType, WOODS_HIGH, WOODS_LOW};
use super::MapRectangle;

#[derive(Clone, Debug)]
pub struct BuildingInfo {
    size: MapRectangle,
}

impl BuildingInfo {
    pub fn get_size(&self) -> &MapRectangle {
        &self.size
    }
}

#[derive(Clone, Debug)]
pub struct TileSpec {
    name: String,
    bulldozable: bool,
    burnable: bool,
    conductive: bool,
    over_water: bool,
    /// TODO: ?
    zone: bool,
    building_info: Option<BuildingInfo>,
    parent: Option<Rc<TileSpec>>,
    parent_offset_x: Option<i8>,
    parent_offset_y: Option<i8>,
    images_keys: Vec<String>,
}

impl TileSpec {
    pub fn get_name(&self) -> &String {
        &self.name
    }
    pub fn can_bulldoze(&self) -> bool {
        self.bulldozable
    }
    pub fn can_burn(&self) -> bool {
        self.burnable
    }
    pub fn can_conduct(&self) -> bool {
        self.conductive
    }
    pub fn is_over_water(&self) -> bool {
        self.over_water
    }
    pub fn get_building_info(&self) -> &Option<BuildingInfo> {
        &self.building_info
    }
}

/// The tile has power if bit 15 is set.
pub const TILE_POWER_BIT: u16 = 0b_1000_0000_0000_0000;
/// The tile conducts electricity if bit 14 is set.
pub const TILE_CONDUCT_BIT: u16 = 0b_0100_0000_0000_0000;
/// The tile is burnable if bit 13 is set.
pub const TILE_BURN_BIT: u16 = 0b_0010_0000_0000_0000;
/// The tile is bulldozable if bit 12 is set.
pub const TILE_BULL_BIT: u16 = 0b_0001_0000_0000_0000;
pub const TILE_BURN_BULL_BIT: u16 = TILE_BURN_BIT | TILE_BULL_BIT;
pub const TILE_BURN_BULL_CONDUCT_BIT: u16 = TILE_CONDUCT_BIT | TILE_BURN_BULL_BIT;
/// The tile is animated if bit 11 is set.
pub const TILE_ANIM_BIT: u16 = 0b_0000_1000_0000_0000;
/// The tile is the center of its zone if bit 10 is set.
pub const TILE_ZONE_BIT: u16 = 0b_0000_0100_0000_0000;
/// TODO: woods level?
pub const TILE_BLBNBIT_MASK: u16 = TILE_BULL_BIT | TILE_BURN_BIT;
/// Bits containing the type of the tile.
pub const TILE_TYPE_MASK: u16 = 0b_0000_0011_1111_1111;
/// Bits containing the status of the tile.
pub const TILE_STATUS_MASK: u16 = TILE_TYPE_MASK ^ 0xFFFF;
/// Mask for the `MapTileCharacters` part of the tile.
pub const TILE_LOW_MASK: u16 = 0x03ff;

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct Tile {
    /// Raw integer describing the type and status of the tile.
    raw: u16,
    /// Cached tile type value.
    tile_type: Option<TileType>,
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Tile(type={:?}, raw={:0>4X})", self.tile_type, self.raw)
    }
}

impl Tile {
    pub fn from_raw(raw: u16) -> Result<Self, String> {
        let tile_type = TileType::from_u16(raw & TILE_TYPE_MASK)
            .ok_or(format!("Tile::from_raw cannot cast tile type from {}", raw))?;
        Ok(Self {
            raw,
            tile_type: Some(tile_type),
        })
    }

    pub fn from_type(tile_type: TileType) -> Result<Self, String> {
        match tile_type {
            TileType::Invalid => Err(format!("Tile::from_type invalid type '{:?}'", tile_type)),
            _ => Ok(Tile {
                raw: tile_type.to_u16().ok_or(format!(
                    "Tile::from_type cannot cast type '{:?}' to raw",
                    tile_type
                ))?,
                tile_type: Some(tile_type),
            }),
        }
    }

    pub fn get_raw(&self) -> u16 {
        self.raw
    }
    pub fn set_raw(&mut self, raw: u16) {
        self.tile_type = TileType::from_u16(raw & TILE_TYPE_MASK);
        self.raw = raw;
    }

    pub fn get_type(&self) -> &Option<TileType> {
        &self.tile_type
    }
    pub fn set_type(&mut self, tile_type: TileType) -> Result<(), String> {
        let type_raw = tile_type.to_u16().ok_or(format!(
            "Tile.set_type cannot cast type '{:?}' to raw",
            tile_type
        ))?;
        let status_raw = self.raw & TILE_STATUS_MASK;
        self.raw = status_raw | type_raw;
        self.tile_type = Some(tile_type);
        Ok(())
    }

    pub fn get_type_raw(&self) -> u16 {
        self.raw & TILE_TYPE_MASK
    }
    pub fn set_type_raw(&mut self, type_raw: u16) {
        let type_raw_filtered = type_raw & TILE_TYPE_MASK;
        let status_raw = self.raw | TILE_STATUS_MASK;
        self.raw = status_raw & type_raw_filtered;
        self.tile_type = TileType::from_u16(type_raw_filtered);
    }

    pub fn is_dirt(&self) -> bool {
        self.tile_type == Some(TileType::Dirt)
    }

    pub fn is_tree(&self) -> bool {
        let type_raw = self.get_type_raw();
        WOODS_LOW <= type_raw && type_raw <= WOODS_HIGH
    }

    pub fn is_of_type(&self, tile_type: &TileType) -> bool {
        self.tile_type == Some(*tile_type)
    }

    pub fn is_any_of_types(&self, tile_types: &[TileType]) -> bool {
        if let Some(tile_type) = self.tile_type {
            tile_types.iter().any(|t| *t == tile_type)
        } else {
            false
        }
    }

    pub fn is_conductive(&self) -> bool {
        self.get_type_raw() & TILE_POWER_BIT == TILE_POWER_BIT
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tile_masks() {
        assert_eq!(TILE_POWER_BIT, 0x8000);
        assert_eq!(TILE_CONDUCT_BIT, 0x4000);
        assert_eq!(TILE_BURN_BIT, 0x2000);
        assert_eq!(TILE_BULL_BIT, 0x1000);
        assert_eq!(TILE_ANIM_BIT, 0x800);
        assert_eq!(TILE_ZONE_BIT, 0x400);
        assert_eq!(TILE_BLBNBIT_MASK, 0x3000);
        assert_eq!(TILE_STATUS_MASK, 0xFC00);
        assert_eq!(TILE_TYPE_MASK, 0x03FF);
    }
}
