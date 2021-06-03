use std::collections::HashMap;

use toolbox::{
    tool_build_building, tool_bulldozer, tool_forest, tool_land, tool_network, tool_park,
    tool_rail, tool_road, tool_water, tool_wire,
};

use crate::{city::budget::MoneyValue, utils::random::MicropolisRandom};

use super::{buildings::BuildingType, tiles::TILE_LOW_MASK, MapPosition, Tile, TileMap};

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
    pub fn cost(self) -> MoneyValue {
        match self {
            Residential => 100,
            Commercial => 100,
            Industrial => 100,
            FireStation => 500,
            PoliceStation => 500,
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

    // TODO: remove duplicate constants with BuildingType
    pub fn size(self) -> u16 {
        match self {
            Residential => 3,
            Commercial => 3,
            Industrial => 3,
            FireStation => 3,
            PoliceStation => 3,
            Wire => 1,
            Bulldozer => 1,
            Railroad => 1,
            Road => 1,
            Stadium => 4,
            Park => 1,
            Seaport => 4,
            CoalPower => 4,
            NuclearPower => 4,
            Airport => 6,
            Network => 1,
            Water => 1,
            Land => 1,
            Forest => 1,
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

impl ToolResult {
    pub fn is_success(&self) -> bool {
        match self {
            &ToolResult::Succeeded(_) => true,
            _ => false,
        }
    }

    pub fn effects(self) -> Option<ToolEffects> {
        match self {
            ToolResult::Succeeded(effects) => Some(effects),
            _ => None,
        }
    }
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

    pub fn add_cost(mut self, cost: u32) -> Self {
        if !self.free {
            self.cost += cost;
        }
        self
    }

    pub fn add_modification(mut self, position: &MapPosition, tile: Tile) -> Self {
        self.set_map_value_at(position, tile);
        self
    }

    pub fn clear(&mut self) {
        self.cost = 0;
        self.modifications.clear();
    }

    /// Consume the given tool result to apply it to the current instance
    /// if it suceeded in order to continue modifications, or return the result as-is otherwise.
    pub fn chain_or_return(&mut self, result: ToolResult) -> Option<ToolResult> {
        match result {
            ToolResult::Succeeded(other) => {
                self.free = self.free || other.free;
                self.cost += if other.free { 0 } else { other.cost };
                self.modifications = {
                    let mut hm = self.modifications.clone();
                    for (position, tile) in other.modifications.iter() {
                        hm.insert(*position, tile.clone());
                    }
                    hm
                };
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
            == self.modifications.len()
        {
            return None;
        }

        // send the notification
        // TODO: messaging system

        self.clear();
        Some(self.cost)
    }

    /// Apply the modifications if there are enough funds.
    pub fn modify_world_if_enough_money(
        &mut self,
        map: &mut TileMap,
        total_funds: u32,
    ) -> (bool, Option<u32>) {
        if self.cost < total_funds {
            (false, None)
        } else {
            let cost = self.modify_world(map);
            (true, cost)
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
            .or_else(|| map.get_tile_at(position).cloned())
    }

    /// Set a new map value at the given position.
    fn set_map_value_at(&mut self, position: &MapPosition, tile: Tile) {
        println!("{:?}=>{}", tile, tile.is_tree());
        self.modifications.insert(*position, tile);
    }
}

pub fn tool_down(
    rng: &mut MicropolisRandom,
    map: &mut TileMap,
    position: &MapPosition,
    tool: &EditingTool,
    auto_bulldoze: bool,
    animations_enabled: bool,
    total_funds: u32,
) -> Result<(), String> {
    Ok(
        match apply_tool(
            rng,
            map,
            position,
            tool,
            auto_bulldoze,
            animations_enabled,
            total_funds,
        )? {
            ToolResult::NeedBulldoze => {
                // TODO: send MESSAGE_BULLDOZE_AREA_FIRST
                // TODO: play interface sound "UhUh" at (x << 4, y << 4
                // TODO: played sound should only be heard by the calling user
            }
            ToolResult::NoMoney => {
                // TODO: send MESSAGE_NOT_ENOUGH_FUNDS
                // TODO: play interface sound "Sorry" at (x << 4, y << 4
                // TODO: played sound should only be heard by the calling user
            }
            _ => {}
        },
    )
}

pub fn apply_tool(
    rng: &mut MicropolisRandom,
    map: &mut TileMap,
    position: &MapPosition,
    tool: &EditingTool,
    auto_bulldoze: bool,
    animations_enabled: bool,
    total_funds: u32,
) -> Result<ToolResult, String> {
    use EditingTool::*;

    // TODO: handle free tool / free terrain editing scenarios
    let effects = ToolEffects::new(false);
    let result = match *tool {
        Residential => apply_build_building(
            map,
            position,
            BuildingType::Residential,
            effects,
            auto_bulldoze,
        ),
        Commercial => apply_build_building(
            map,
            position,
            BuildingType::Commercial,
            effects,
            auto_bulldoze,
        ),
        Industrial => apply_build_building(
            map,
            position,
            BuildingType::Industrial,
            effects,
            auto_bulldoze,
        ),
        FireStation => apply_build_building(
            map,
            position,
            BuildingType::FireStation,
            effects,
            auto_bulldoze,
        ),
        PoliceStation => apply_build_building(
            map,
            position,
            BuildingType::PoliceStation,
            effects,
            auto_bulldoze,
        ),
        Wire => tool_wire(map, position, effects, auto_bulldoze),
        Bulldozer => tool_bulldozer(
            rng,
            map,
            position,
            effects,
            auto_bulldoze,
            animations_enabled,
        ),
        Railroad => tool_rail(map, position, effects, auto_bulldoze),
        Road => tool_road(map, position, effects, auto_bulldoze),
        Stadium => {
            apply_build_building(map, position, BuildingType::Stadium, effects, auto_bulldoze)
        }
        Park => tool_park(rng, map, position, effects),
        Seaport => {
            apply_build_building(map, position, BuildingType::Seaport, effects, auto_bulldoze)
        }
        CoalPower => apply_build_building(
            map,
            position,
            BuildingType::CoalPowerPlant,
            effects,
            auto_bulldoze,
        ),
        NuclearPower => apply_build_building(
            map,
            position,
            BuildingType::NuclearPowerPlant,
            effects,
            auto_bulldoze,
        ),
        Airport => {
            apply_build_building(map, position, BuildingType::Airport, effects, auto_bulldoze)
        }
        Network => tool_network(map, position, effects),
        Water => tool_water(map, position, effects),
        Land => tool_land(
            rng,
            map,
            position,
            effects,
            auto_bulldoze,
            animations_enabled,
        ),
        Forest => tool_forest(
            rng,
            map,
            position,
            effects,
            auto_bulldoze,
            animations_enabled,
        ),
    }?;

    match result.clone() {
        ToolResult::Succeeded(mut chained_effects) => {
            if chained_effects
                .modify_world_if_enough_money(map, total_funds)
                .0
            {
                Ok(result)
            } else {
                Ok(ToolResult::NoMoney)
            }
        }
        _ => Ok(result),
    }
}

fn apply_build_building(
    map: &TileMap,
    center: &MapPosition,
    building: BuildingType,
    mut effects: ToolEffects,
    auto_bulldoze: bool,
) -> Result<ToolResult, String> {
    tool_build_building(map, center, effects, &building.info()?, auto_bulldoze)
}
