use std::collections::HashMap;

use rand::Rng;

use super::{tiles::TILE_LOW_MASK, MapPosition, MapRectangle, Tile, TileMap};

mod effects;
mod toolbox;
mod utils;

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

pub struct BuildingConstructionInfo {
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
    /// Ignore any incurred cost.
    free: bool,
    /// Accumulated cost of the modifications.
    cost: u32,
    /// Set of modifications in the world, indexed by position.
    modifications: HashMap<MapPosition, Tile>,
}

impl ToolEffects {
    pub fn new(free: bool) -> Self {
        Self {
            free,
            cost: 0,
            modifications: HashMap::new(),
        }
    }

    #[must_use]
    pub fn add_cost(self, cost: u32) -> Self {
        if self.free {
            self.cost += cost;
        }
        self
    }

    #[must_use]
    pub fn add_modification(self, position: &MapPosition, tile: Tile) -> Self {
        self.set_map_value_at(position, tile);
        self
    }

    pub fn clear(&mut self) {
        self.cost = 0;
        self.modifications.clear();
    }

    /// Consume the given tool result to apply it to the current instance
    /// if it suceeded in order to continue modifications, or return the result as-is otherwise.
    pub fn chain_or_return(self, result: ToolResult) -> Option<ToolResult> {
        match result {
            ToolResult::Succeeded(other) => {
                self = other;
                None
            }
            _ => Some(result),
        }
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

pub fn apply_tool<R: Rng>(
    rng: &mut R,
    map: &TileMap,
    position: &MapPosition,
    tool: &EditingTool,
    auto_bulldoze: bool,
    animations_enabled: bool,
) -> Result<ToolEffects, String> {
    use EditingTool::*;

    // TODO: handle free tool / free terrain editing scenarios
    let effects = ToolEffects::new(false);
    // let result = match *tool {
    // };

    todo!()
}
