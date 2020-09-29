use super::{tools::EditingTool, MapRectangle, Tile, TileType};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BuildingInfo {
    /// Tiles footprint.
    size: MapRectangle,
    /// Tile value at top-left in the map.
    base_tile: Tile,
    /// Tool needed for making the building.
    tool: EditingTool,
    /// Name of the tool needed for making the building.
    tool_name: String,
    /// Building has animated tiles?
    is_animated: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BuildingType {
    Residential,
    Commercial,
    Industrial,
    PoliceStation,
    FireStation,
    Stadium,
    CoalPowerPlant,
    NuclearPowerPlant,
    Seaport,
    Airport,
}

impl BuildingType {
    pub fn info(&self) -> Result<BuildingInfo, String> {
        use BuildingType::*;

        Ok(match *self {
            Residential => BuildingInfo {
                size: (3, 3).into(),
                base_tile: Tile::from_type(TileType::ResidentialBase)?,
                tool: EditingTool::Residential,
                tool_name: "Res".into(),
                is_animated: false,
            },
            Commercial => BuildingInfo {
                size: (3, 3).into(),
                base_tile: Tile::from_type(TileType::CommercialBase)?,
                tool: EditingTool::Commercial,
                tool_name: "Com".into(),
                is_animated: false,
            },
            Industrial => BuildingInfo {
                size: (3, 3).into(),
                base_tile: Tile::from_type(TileType::IndustrialBase)?,
                tool: EditingTool::Commercial,
                tool_name: "Ind".into(),
                is_animated: false,
            },
            PoliceStation => BuildingInfo {
                size: (3, 3).into(),
                base_tile: Tile::from_type(TileType::PoliceStationBase)?,
                tool: EditingTool::Commercial,
                tool_name: "Pol".into(),
                is_animated: false,
            },
            FireStation => BuildingInfo {
                size: (3, 3).into(),
                base_tile: Tile::from_type(TileType::FireStationBase)?,
                tool: EditingTool::FireStation,
                tool_name: "Fire".into(),
                is_animated: false,
            },
            Stadium => BuildingInfo {
                size: (4, 4).into(),
                base_tile: Tile::from_type(TileType::StadiumBase)?,
                tool: EditingTool::Stadium,
                tool_name: "Stad".into(),
                is_animated: false,
            },
            CoalPowerPlant => BuildingInfo {
                size: (4, 4).into(),
                base_tile: Tile::from_type(TileType::CoalBase)?,
                tool: EditingTool::CoalPower,
                tool_name: "Coal".into(),
                is_animated: false,
            },
            NuclearPowerPlant => BuildingInfo {
                size: (4, 4).into(),
                base_tile: Tile::from_type(TileType::NuclearBase)?,
                tool: EditingTool::NuclearPower,
                tool_name: "Nuc".into(),
                is_animated: true,
            },
            Seaport => BuildingInfo {
                size: (4, 4).into(),
                base_tile: Tile::from_type(TileType::PortBase)?,
                tool: EditingTool::Seaport,
                tool_name: "Seap".into(),
                is_animated: false,
            },
            Airport => BuildingInfo {
                size: (6, 6).into(),
                base_tile: Tile::from_type(TileType::AirportBase)?,
                tool: EditingTool::Airport,
                tool_name: "Airp".into(),
                is_animated: false,
            },
        })
    }
}
