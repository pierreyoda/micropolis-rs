use std::collections::HashMap;

use super::{tiles::TILE_LOW_MASK, MapPosition, MapRectangle, Tile, TileMap};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ConnectTileCommand {
    /// Fix zone (connect wire, road and rail).
    Fix,
    /// Bulldoze and fix zone.
    Bulldoze,
    /// Lay road and fix zone.
    Road,
    /// Lay rail and fix zone.
    Rail,
    /// Lay wire and fix zone.
    Wire,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum EditingTool {
    Residential,
    Commercial,
    Industrial,
    FireStation,
    PoliceStation,
    Query,
    Wire,
    Bulldozer,
    Railroad,
    Road,
    Stadium,
    Park,
    Seaport,
    CoalPower,
    NuclearPower,
    Airport,
    Network,
    Water,
    Land,
    Forest,
}

impl EditingTool {
    pub fn cost(&self) -> u32 {
        match *self {
            Residential => 100,
            Commercial => 100,
            Industrial => 100,
            FireStation => 500,
            PoliceStation => 500,
            Query => 0,
            Wire => 5,
            Bulldozer => 1,
            Railroad => 20,
            Road => 10,
            Stadium => 5000,
            Park => 10,
            Seaport => 3000,
            CoalPower => 3000,
            NuclearPower => 5000,
            Airport => 10000,
            Network => 100,
            Water => 0,
            Land => 0,
            Forest => 0,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ToolResult {
    /// User has not enough money for tool.
    NoMoney,
    /// Clear the area first.
    NeedBulldoze,
    /// Cannot build here.
    Failed,
    /// Build succeeded.
    Succeeded(ToolEffects),
}

struct BuildingConstructionInfo {
    /// Number of tiles.
    size: MapRectangle,
    /// Tile value at top-left in the map.
    base_tile: Tile,
    /// Tool needed for making the building.
    tool: EditingTool,
    /// Name of the tool needed for making the building.
    tool_name: String,
    /// Building has animated tiles?
    building_is_animated: bool,
}

/// Structure for storing effects of applying a tool to the world.
///
/// When applying a tool, two things change:
/// - The world map.
/// - The funds of the player.
/// - Messages sent to the player and the front-end.
/// - Sounds played for the player.
///
/// The funds gives a decision problem. To decide whether the tool can be
/// applied, you need to know the cost. To know the cost you need to know the
/// exact changes being made.
/// The simplest way to compute the exact changes is to simply apply the tool to
/// the world. This holds especially when tools get stacked on top of each
/// other.
///
/// This class provides an easy way out, greatly simplifying the problem.
/// All tools do not modify the world directly, but instead put their results
/// in an instance of this class, thus collecting all the modifications.
/// After the whole operation is 'done', the `ToolEffects` instance can tell the
/// precise cost and what has been changed in the world. At that moment, the
/// yes/no decision can be made, and the effects can be copied to the real map
/// and funds.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ToolEffects {
    /// Accumulated cost of the modifications.
    cost: u32,
    /// Set of modifications in the world, indexed by position.
    modifications: HashMap<MapPosition, Tile>,
}

impl ToolEffects {
    pub fn new() -> Self {
        Self {
            cost: 0,
            modifications: HashMap::new(),
        }
    }

    pub fn add_cost(self, cost: u32) -> Self {
        self.cost += cost;
        self
    }

    pub fn add_modification(self, position: &MapPosition, tile: Tile) -> Self {
        self.set_map_value_at(position, tile);
        self
    }

    pub fn clear(&mut self) {
        self.cost = 0;
        self.modifications.clear();
    }

    /// Perform the effects stored in the structure to the simulation world.
    ///
    /// Returns the total cost of the operation.
    #[must_use]
    pub fn modify_world(&mut self, map: &mut TileMap) -> Option<u32> {
        // modify the world
        if self
            .modifications
            .iter()
            .map(|(position, tile)| map.set_tile_at(position, tile.clone()))
            .filter(|modified| !*modified)
            .count()
            > 0
        {
            return None;
        }

        // send the notification
        // TODO: messaging system

        self.clear();
        Some(self.cost)
    }

    /// Apply the modifications if there are enough funds.
    pub fn modify_world_if_enough_money(&self, map: &mut TileMap, total_funds: u32) -> bool {
        if self.cost < total_funds {
            false
        } else {
            self.modify_world(map);
            true
        }
    }

    /// Get the tile at the given position.
    pub fn get_map_tile_at(&self, map: &TileMap, position: &MapPosition) -> Option<Tile> {
        Tile::from_raw(self.get_map_value_at(map, position)?.get_raw() & TILE_LOW_MASK).ok()
    }

    /// Get a map value from the world at the given position.
    ///
    /// Unlike the simulation world, this method takes modifications made
    /// previously by other tools into account.
    pub fn get_map_value_at(&self, map: &TileMap, position: &MapPosition) -> Option<Tile> {
        self.modifications
            .get(position)
            .cloned()
            .or(map.get_tile_at(position).cloned())
    }

    /// Set a new map value at the given position.
    fn set_map_value_at(&mut self, position: &MapPosition, tile: Tile) {
        self.modifications.insert(position.clone(), tile);
    }
}

/// Collection of building utilities.
pub mod utilities {
    use rand::Rng;

    use crate::{
        map::generator::MapGenerator, map::tiles::TILE_ANIM_BIT, map::tiles::TILE_BULL_BIT,
        map::tiles::TILE_BURN_BIT, map::tiles::TILE_BURN_BULL_BIT, map::tiles::TILE_CONDUCT_BIT,
        map::MapPosition, map::Tile, map::TileMap, map::TileType, utils::random_in_range,
    };

    use super::{EditingTool, ToolEffects, ToolResult};

    /// Put down a park at the given position.
    pub fn put_down_park<R: Rng>(
        rng: &mut R,
        map: &TileMap,
        position: &MapPosition,
        effects: ToolEffects,
    ) -> Result<ToolResult, String> {
        let value: u16 = random_in_range(rng, 0, 4);
        let tile_raw = TILE_BURN_BIT
            | TILE_BULL_BIT
            | match value {
                4 => TileType::FOUNTAIN.to_u16().unwrap() | TILE_ANIM_BIT,
                _ => value + TileType::Woods2.to_u16().unwrap(),
            };

        match effects.get_map_value_at(map, position) {
            Some(tile) if tile.is_dirt() => Ok(ToolResult::NeedBulldoze),
            _ => Ok(ToolResult::Succeeded(
                effects
                    .add_cost(EditingTool::Park.cost())
                    .add_modification(position, Tile::from_raw(tile_raw)?),
            )),
        }
    }

    /// Put down a communication network.
    pub fn put_down_network(
        map: &TileMap,
        position: &MapPosition,
        effects: ToolEffects,
    ) -> Result<ToolResult, String> {
        let mut tile = effects
            .get_map_tile_at(map, position)
            .ok_or(format!("Cannot read tile at {}", position))?;
        if !tile.is_dirt() {
            if is_tile_auto_bulldozable(&tile).unwrap() {
                effects = effects
                    .add_cost(EditingTool::Bulldozer.cost())
                    .add_modification(position, Tile::from_type(TileType::Dirt)?);
                tile = Tile::from_type(TileType::Dirt).unwrap();
            } else {
                return Ok(ToolResult::NeedBulldoze);
            }
        }

        Ok(ToolResult::Succeeded(
            effects
                .add_cost(EditingTool::Network.cost())
                .add_modification(
                    position,
                    Tile::from_raw(
                        TileType::INDBASE2.to_u16().unwrap()
                            | TILE_CONDUCT_BIT
                            | TILE_BURN_BIT
                            | TILE_BULL_BIT
                            | TILE_ANIM_BIT,
                    )?,
                ),
        ))
    }

    /// Put down a water tile.
    pub fn put_down_water(
        map: &TileMap,
        position: &MapPosition,
        effects: ToolEffects,
    ) -> Result<ToolResult, String> {
        let tile = effects
            .get_map_tile_at(map, position)
            .ok_or(format!("Cannot read tile at {}", position))?;
        if tile.is_of_type(&TileType::River) {
            Ok(ToolResult::Failed)
        } else {
            Ok(ToolResult::Succeeded(
                effects
                    .add_cost(EditingTool::Water.cost())
                    .add_modification(position, Tile::from_type(TileType::River)?),
            ))
        }
    }

    /// Put down a land tile.
    pub fn put_down_land(
        map: &TileMap,
        position: &MapPosition,
        effects: ToolEffects,
    ) -> Result<ToolResult, String> {
        let tile = effects
            .get_map_tile_at(map, position)
            .ok_or(format!("Cannot read tile at {}", position))?;
        if tile.is_dirt() {
            Ok(ToolResult::Failed)
        } else {
            Ok(ToolResult::Succeeded(
                effects
                    .add_cost(EditingTool::Land.cost())
                    .add_modification(position, Tile::from_type(TileType::Dirt)?),
            ))
        }
    }

    const FOREST_DX: [i32; 8] = [-1, 0, 1, -1, 1, -1, 0, 1];
    const FOREST_DY: [i32; 8] = [-1, -1, -1, 0, 0, 1, 1, 1];

    /// Put down a forest.
    pub fn put_down_forest(
        map: &TileMap,
        position: &MapPosition,
        effects: ToolEffects,
    ) -> Result<ToolResult, String> {
        effects = effects.add_modification(
            position,
            Tile::from_raw(TileType::Woods.to_u16().unwrap() | TILE_BURN_BULL_BIT)?,
        );

        for i in 0..8 {
            if map.in_bounds(
                &(*position
                    + MapPosition::new(*FOREST_DX.get(i).unwrap(), *FOREST_DY.get(i).unwrap())),
            ) {
                effects = MapGenerator::smooth_trees_at(map, position, effects, true)?;
            }
        }

        Ok(ToolResult::Succeeded(
            effects.add_cost(EditingTool::Forest.cost()),
        ))
    }

    /// Check and connect a new zone around the border.
    pub fn check_border() {}

    /// Computes the size of the zone that the tile belongs to, or 0 if
    /// unknown tile value.
    pub fn compute_size(tile: &Tile) -> Option<u8> {
        match (*tile.get_type())?.to_u16()? {
            t if (t >= offseted_raw_type(&TileType::ResidentialBase, -1)?
                && t <= offseted_raw_type(&TileType::PortBase, -1)?)
                || (t >= offseted_raw_type(&TileType::LastPowerPlant, 1)?
                    && t <= offseted_raw_type(&TileType::PoliceStation, 4)?)
                || (t >= TileType::Church1Base.to_u16()?
                    && t <= TileType::Church7Last.to_u16()?) =>
            {
                Some(3)
            }
            t if (t >= TileType::PortBase.to_u16()? && t <= TileType::LastPort.to_u16()?)
                || (t >= TileType::CoalBase.to_u16()?
                    && t <= TileType::LastPowerPlant.to_u16()?)
                || (t >= TileType::StadiumBase.to_u16()?
                    && t <= TileType::LastZone.to_u16()?) =>
            {
                Some(4)
            }
            _ => Some(0),
        }
    }

    /// Compute where the "center" - at (1, 1) - of the zone is, depending on where
    /// the user clicked.
    ///
    /// Only inner tiles are recognized, and possibly not even complete (ie. stadium
    /// while game is playing).
    ///
    /// Returns the corrected position and the size of the zone clicked at
    /// (or 0 if cliked outside zone).
    pub fn check_big_zone(tile: &Tile) -> Option<(MapPosition, u8)> {
        match (*tile.get_type())?.to_u16()? {
            t if [
                offseted_raw_type(&TileType::PowerPlant, 0)?,
                offseted_raw_type(&TileType::Port, 0)?,
                offseted_raw_type(&TileType::Nuclear, 0)?,
                offseted_raw_type(&TileType::Stadium, 0)?,
            ]
            .contains(&t) =>
            {
                Some((MapPosition::new(0, 0), 4))
            }
            t if [
                offseted_raw_type(&TileType::PowerPlant, 0)?,
                offseted_raw_type(&TileType::CoalSmoke3, 0)?,
                offseted_raw_type(&TileType::CoalSmoke3, 1)?,
                offseted_raw_type(&TileType::CoalSmoke3, 2)?,
                offseted_raw_type(&TileType::Port, 1)?,
                offseted_raw_type(&TileType::Nuclear, 1)?,
                offseted_raw_type(&TileType::Stadium, 1)?,
            ]
            .contains(&t) =>
            {
                Some((MapPosition::new(-1, 0), 4))
            }
            t if [
                offseted_raw_type(&TileType::PowerPlant, 4)?,
                offseted_raw_type(&TileType::Port, 4)?,
                offseted_raw_type(&TileType::Nuclear, 4)?,
                offseted_raw_type(&TileType::Stadium, 4)?,
            ]
            .contains(&t) =>
            {
                Some((MapPosition::new(0, -1), 4))
            }
            t if [
                offseted_raw_type(&TileType::PowerPlant, 5)?,
                offseted_raw_type(&TileType::Port, 5)?,
                offseted_raw_type(&TileType::Nuclear, 5)?,
                offseted_raw_type(&TileType::Stadium, 5)?,
            ]
            .contains(&t) =>
            {
                Some((MapPosition::new(-1, -1), 4))
            }
            t if [offseted_raw_type(&TileType::Airport, 0)?].contains(&t) => {
                Some((MapPosition::new(0, 0), 6))
            }
            t if [offseted_raw_type(&TileType::Airport, 1)?].contains(&t) => {
                Some((MapPosition::new(-1, 0), 6))
            }
            t if [offseted_raw_type(&TileType::Airport, 2)?].contains(&t) => {
                Some((MapPosition::new(-2, 0), 6))
            }
            t if [offseted_raw_type(&TileType::Airport, 3)?].contains(&t) => {
                Some((MapPosition::new(-3, 0), 6))
            }
            t if [offseted_raw_type(&TileType::Airport, 6)?].contains(&t) => {
                Some((MapPosition::new(0, -1), 6))
            }
            t if [offseted_raw_type(&TileType::Airport, 7)?].contains(&t) => {
                Some((MapPosition::new(-1, -1), 6))
            }
            t if [offseted_raw_type(&TileType::Airport, 8)?].contains(&t) => {
                Some((MapPosition::new(-2, -1), 6))
            }
            t if [offseted_raw_type(&TileType::Airport, 9)?].contains(&t) => {
                Some((MapPosition::new(-3, -1), 6))
            }
            t if [offseted_raw_type(&TileType::Airport, 12)?].contains(&t) => {
                Some((MapPosition::new(0, -2), 6))
            }
            t if [offseted_raw_type(&TileType::Airport, 13)?].contains(&t) => {
                Some((MapPosition::new(-1, -2), 6))
            }
            t if [offseted_raw_type(&TileType::Airport, 14)?].contains(&t) => {
                Some((MapPosition::new(-2, -2), 6))
            }
            t if [offseted_raw_type(&TileType::Airport, 15)?].contains(&t) => {
                Some((MapPosition::new(-3, -2), 6))
            }
            t if [offseted_raw_type(&TileType::Airport, 18)?].contains(&t) => {
                Some((MapPosition::new(0, -3), 6))
            }
            t if [offseted_raw_type(&TileType::Airport, 19)?].contains(&t) => {
                Some((MapPosition::new(-1, -3), 6))
            }
            t if [offseted_raw_type(&TileType::Airport, 20)?].contains(&t) => {
                Some((MapPosition::new(-2, -3), 6))
            }
            t if [offseted_raw_type(&TileType::Airport, 21)?].contains(&t) => {
                Some((MapPosition::new(-3, -3), 6))
            }
            _ => Some((MapPosition::new(0, 0), 0)),
        }
    }

    /// Can the tile be auto-bulldozed?
    ///
    /// Called `tally` in the C++ codebase.
    pub fn is_tile_auto_bulldozable(tile: &Tile) -> Option<bool> {
        let raw = tile.get_raw();
        Some(
            (raw >= TileType::FirstRiverEdge.to_u16()? && raw <= TileType::LastRubble.to_u16()?)
                || (raw >= TileType::HorizontalPower.to_u16()? + 2
                    && raw <= TileType::HorizontalPower.to_u16()? + 12)
                || (raw >= TileType::TINYEXP.to_u16()?
                    && raw <= TileType::LASTTINYEXP.to_u16()? + 2),
        )
    }

    fn offseted_raw_type(tile_type: &TileType, offset: i8) -> Option<u16> {
        Some((tile_type.to_u16()? as i32 + offset as i32) as u16)
    }
}
