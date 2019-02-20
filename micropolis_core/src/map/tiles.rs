use std::rc::Rc;

use super::tiles_type::TileType;
use super::MapRect;

#[derive(Clone, Debug)]
pub struct BuildingInfo {
    size: MapRect,
}

impl BuildingInfo {
    pub fn get_size(&self) -> &MapRect {
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

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Tile(i32);

impl Tile {
    pub fn get_type(&self) -> TileType {
        // TODO:
        TileType::Dirt
    }
}
