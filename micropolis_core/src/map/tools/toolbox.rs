use crate::{
    map::buildings::BuildingInfo, map::connect::TileMapConnector, map::tiles::TILE_ANIM_BIT,
    map::tiles::TILE_BULL_BIT, map::tiles::TILE_BURN_BIT, map::tiles::TILE_CONDUCT_BIT,
    map::tiles::TILE_LOW_MASK, map::tiles::TILE_ZONE_BIT, map::MapPosition, map::Tile,
    map::TileMap, map::TileType, utils::random::MicropolisRandom,
};

use super::{
    effects::build_building, effects::put_down_forest, effects::put_down_land,
    effects::put_down_park, effects::put_down_water, effects::put_rubble, utils::check_big_zone,
    utils::compute_size, utils::is_tile_auto_bulldozable, ConnectTileCommand, EditingTool,
    ToolEffects, ToolResult,
};

/// Apply bulldozer tool (manual, with explosion animation for buildings).
pub(super) fn tool_bulldozer(
    rng: &mut MicropolisRandom,
    map: &TileMap,
    position: &MapPosition,
    mut effects: ToolEffects,
    auto_bulldoze: bool,
    animations_enabled: bool,
) -> Result<ToolResult, String> {
    if !map.in_bounds(position) {
        return Ok(ToolResult::Failed);
    }

    let tile_raw = effects
        .get_map_tile_at(map, position)
        .ok_or(format!("toool_bulldozer: cannot read tile at {}", position))?
        .get_raw()
        & TILE_LOW_MASK;
    let tile = Tile::from_raw(tile_raw)?;
    let (delta, zone_size) = if tile_raw & TILE_ZONE_BIT != 0x00 {
        (
            MapPosition::new(0, 0),
            compute_size(&tile).ok_or(format!(
                "tool_bulldozer: cannot compute size for tile {}",
                tile
            ))?,
        )
    } else {
        check_big_zone(&tile).ok_or(format!(
            "tool_bulldozer: cannot check big zone for tile {}",
            tile
        ))?
    };

    if zone_size == 0 {
        // invalid zone
        if let Some(result) = effects.chain_or_return(
            if tile.is_any_of_types(&[TileType::River, TileType::RiverEdge, TileType::Channel]) {
                let result = TileMapConnector::connect_tile(
                    map,
                    position,
                    &ConnectTileCommand::Bulldoze,
                    effects.clone(),
                    auto_bulldoze,
                )?;
                if result.is_success()
                    && effects.get_map_tile_at(map, position).ok_or(format!(
                        "tool_bulldozer: cannot read effects tile at {}",
                        position
                    ))? != tile
                {
                    ToolResult::Succeeded(result.effects().unwrap().add_cost(5))
                } else {
                    result
                }
            } else {
                TileMapConnector::connect_tile(
                    map,
                    position,
                    &ConnectTileCommand::Bulldoze,
                    effects.clone(),
                    auto_bulldoze,
                )?
            },
        ) {
            return Ok(result);
        }
    } else {
        effects = effects.add_cost(EditingTool::Bulldozer.cost());
        let center = *position + delta;

        match zone_size {
            3 => {
                // TODO: trigger sound Explosion-High in channel city at position
                effects = put_rubble(
                    rng,
                    map,
                    &center.with_offset(-1, -1),
                    3,
                    effects,
                    animations_enabled,
                )?;
            }
            4 => {
                // TODO: trigger sound Explosion-Low in channel city at position
                effects = put_rubble(
                    rng,
                    map,
                    &center.with_offset(-1, -1),
                    4,
                    effects,
                    animations_enabled,
                )?;
            }
            6 => {
                // TODO: trigger sound Explosion-High in channel city at position
                // TODO: trigger sound Explosion-Low in channel city at position
                effects = put_rubble(
                    rng,
                    map,
                    &center.with_offset(-1, -1),
                    6,
                    effects,
                    animations_enabled,
                )?;
            }
            _ => unreachable!(""),
        }
    }

    // TODO: send "didtool" message "Dozr, (x, y)"

    Ok(ToolResult::Succeeded(effects))
}

/// Build arbitrary infrastructure at the given position.
fn tool_build_wrapper<F: FnMut(ToolEffects) -> Result<ToolResult, String>>(
    map: &TileMap,
    position: &MapPosition,
    mut effects: ToolEffects,
    mut apply: F,
    tool_message_id: &str,
) -> Result<ToolResult, String> {
    if !map.in_bounds(position) {
        return Ok(ToolResult::Failed);
    }
    if let Some(result) = effects.chain_or_return(apply(effects.clone())?) {
        return Ok(result);
    }
    // TODO: send "didtool" message "(tool_message_id), (x, y)"
    let id = tool_message_id.clone();
    Ok(ToolResult::Succeeded(effects))
}

/// Build a road at the given position.
pub(super) fn tool_road(
    map: &TileMap,
    position: &MapPosition,
    effects: ToolEffects,
    auto_bulldoze: bool,
) -> Result<ToolResult, String> {
    tool_build_wrapper(
        map,
        position,
        effects,
        |mut e| {
            TileMapConnector::connect_tile(
                map,
                position,
                &ConnectTileCommand::Road,
                e,
                auto_bulldoze,
            )
        },
        "Road",
    )
}

/// Build a railroad at the given position.
pub(super) fn tool_rail(
    map: &TileMap,
    position: &MapPosition,
    mut effects: ToolEffects,
    auto_bulldoze: bool,
) -> Result<ToolResult, String> {
    tool_build_wrapper(
        map,
        position,
        effects,
        |mut e| {
            TileMapConnector::connect_tile(
                map,
                position,
                &ConnectTileCommand::Rail,
                e,
                auto_bulldoze,
            )
        },
        "Rail",
    )
}

/// Build a wire at the given position.
pub(super) fn tool_wire(
    map: &TileMap,
    position: &MapPosition,
    mut effects: ToolEffects,
    auto_bulldoze: bool,
) -> Result<ToolResult, String> {
    tool_build_wrapper(
        map,
        position,
        effects,
        |mut e| {
            TileMapConnector::connect_tile(
                map,
                position,
                &ConnectTileCommand::Wire,
                e,
                auto_bulldoze,
            )
        },
        "Wire",
    )
}

/// Build at park.
pub(super) fn tool_park(
    rng: &mut MicropolisRandom,
    map: &TileMap,
    center: &MapPosition,
    effects: ToolEffects,
) -> Result<ToolResult, String> {
    tool_build_wrapper(
        map,
        center,
        effects,
        |mut e| put_down_park(rng, map, center, e),
        "Park",
    )
}

/// Build a communication network.
pub(super) fn tool_network(
    map: &TileMap,
    position: &MapPosition,
    effects: ToolEffects,
) -> Result<ToolResult, String> {
    tool_build_wrapper(
        map,
        position,
        effects,
        |mut e| put_down_network(map, position, e),
        "Park",
    )
}

/// Put down a communication network.
fn put_down_network(
    map: &TileMap,
    position: &MapPosition,
    mut effects: ToolEffects,
) -> Result<ToolResult, String> {
    let mut tile = effects
        .get_map_tile_at(map, position)
        .ok_or(format!("Cannot read tile at {}", position))?;
    if !tile.is_dirt() && is_tile_auto_bulldozable(&tile).unwrap() {
        effects = effects
            .add_cost(EditingTool::Bulldozer.cost())
            .add_modification(position, Tile::from_type(TileType::Dirt)?);
        tile = Tile::from_type(TileType::Dirt).unwrap();
    }

    if !tile.is_of_type(&TileType::Dirt) {
        Ok(ToolResult::NeedBulldoze)
    } else {
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
}

/// Place water at the give position.
pub(super) fn tool_water(
    map: &TileMap,
    position: &MapPosition,
    mut effects: ToolEffects,
) -> Result<ToolResult, String> {
    tool_build_wrapper(
        map,
        position,
        effects,
        |mut e| put_down_water(map, position, e),
        "Net",
    )
}

/// Place land at the give position.
pub(super) fn tool_land(
    rng: &mut MicropolisRandom,
    map: &TileMap,
    position: &MapPosition,
    effects: ToolEffects,
    auto_bulldoze: bool,
    animations_enabled: bool,
) -> Result<ToolResult, String> {
    tool_build_wrapper(
        map,
        position,
        effects,
        |mut e| {
            if let Some(result) = e.chain_or_return(tool_bulldozer(
                rng,
                map,
                position,
                e.clone(),
                auto_bulldoze,
                animations_enabled,
            )?) {
                return Ok(result);
            }
            put_down_land(map, position, e)
        },
        "Land",
    )
}

/// Place a forest at the given position.
pub(super) fn tool_forest(
    rng: &mut MicropolisRandom,
    map: &TileMap,
    position: &MapPosition,
    mut effects: ToolEffects,
    auto_bulldoze: bool,
    animations_enabled: bool,
) -> Result<ToolResult, String> {
    tool_build_wrapper(
        map,
        position,
        effects,
        |mut e| {
            let tile = e.get_map_value_at(map, position).ok_or(format!(
                "tool_forest: cannot read effects tile at {}",
                position
            ))?;
            if tile.is_tree() {
                // nothing to do
                return Ok(ToolResult::Succeeded(e));
            }

            if !tile.is_dirt() {
                // TODO: bulldozer should be free in terrain mode or from a free tool
                if let Some(result) = e.chain_or_return(tool_bulldozer(
                    rng,
                    map,
                    position,
                    e.clone(),
                    auto_bulldoze,
                    animations_enabled,
                )?) {
                    return Ok(result);
                }
            }

            let tile_refreshed = e.get_map_value_at(map, position).ok_or(format!(
                "tool_forest: cannot read effects tile at {}",
                position
            ))?;
            if tile_refreshed.is_dirt() {
                put_down_forest(map, position, e)
            } else {
                // auto-bulldozing not allowed or impossible
                Ok(ToolResult::Failed)
            }
        },
        "Forest",
    )
}

/// Build at building.
pub(super) fn tool_build_building(
    map: &TileMap,
    center: &MapPosition,
    mut effects: ToolEffects,
    building_info: &BuildingInfo,
    auto_bulldoze: bool,
) -> Result<ToolResult, String> {
    tool_build_wrapper(
        map,
        center,
        effects,
        |mut e| build_building(map, center, &building_info, e, auto_bulldoze),
        &*building_info.tool_name,
    )
}
