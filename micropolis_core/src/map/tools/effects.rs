use rand::Rng;

use crate::{
    map::{
        generator::MapGenerator, tiles::TILE_ANIM_BIT, tiles::TILE_BULL_BIT, tiles::TILE_BURN_BIT,
        tiles::TILE_BURN_BULL_BIT, tiles::TILE_CONDUCT_BIT, tiles::TILE_ZONE_BIT, MapPosition,
        MapRectangle, Tile, TileMap, TileType,
    },
    utils::random_in_range,
};

use super::{
    utils::check_border, utils::is_tile_auto_bulldozable, BuildingInfo, EditingTool, ToolEffects,
    ToolResult,
};

/// Make a square of rubble tiles of the given size.
#[must_use]
pub(super) fn put_rubble<R: Rng>(
    rng: &mut R,
    map: &TileMap,
    anchor: &MapPosition,
    size: i8,
    effects: ToolEffects,
    animations_enabled: bool,
) -> Result<ToolEffects, String> {
    for x in anchor.x..anchor.x + size as i32 {
        for y in anchor.y..anchor.y + size as i32 {
            let current_position = MapPosition::new(x, y);
            if !map.in_bounds(&current_position) {
                continue;
            }
            let tile = effects
                .get_map_tile_at(map, &current_position)
                .ok_or(format!(
                    "put_rubble cannot read effects tile at {}",
                    current_position
                ))?;
            if tile.is_any_of_types(&[TileType::Radioactive, TileType::Dirt]) {
                continue;
            }
            effects = effects.add_modification(
                &current_position,
                Tile::from_raw(
                    if animations_enabled {
                        TileType::TINYEXP.to_u16().unwrap() + random_in_range(rng, 0, 2)
                    } else {
                        TileType::SOMETINYEXP.to_u16().unwrap()
                    } | TILE_ANIM_BIT
                        | TILE_BULL_BIT,
                )?,
            );
        }
    }
    Ok(effects)
}

/// Put down a park at the given position.
pub(super) fn put_down_park<R: Rng>(
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

/// Put down a water tile.
pub(super) fn put_down_water(
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
pub(super) fn put_down_land(
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
pub(super) fn put_down_forest(
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
            &(*position + MapPosition::new(*FOREST_DX.get(i).unwrap(), *FOREST_DY.get(i).unwrap())),
        ) {
            effects = MapGenerator::smooth_trees_at(map, position, effects, true)?;
        }
    }

    Ok(ToolResult::Succeeded(
        effects.add_cost(EditingTool::Forest.cost()),
    ))
}

/// Prepare the site where a building is about to be put down,.
///
/// This function performs some basic sanity checks, and implements
/// the auto-bulldoze functionality to prepare the site.
fn prepare_bulding_site(
    map: &TileMap,
    position: &MapPosition,
    size: &MapRectangle,
    effects: ToolEffects,
    auto_bulldoze: bool,
) -> Result<ToolResult, String> {
    let (width, height) = (size.width as i8, size.height as i8);

    // check that the entire site fits on the map
    if !map.bounds().is_contained(&position, size) {
        return Ok(ToolResult::Failed);
    }

    // ensure that all the tiles are clear, auto-bulldoze if allowed and possible
    for dy in 0..height {
        for dx in 0..width {
            let current_position = position.with_offset(dx, dy);
            let current_tile = effects
                .get_map_tile_at(map, &current_position)
                .ok_or(format!("Cannot read tile at {}", current_position))?;
            if current_tile.is_of_type(&TileType::Dirt) {
                continue; // tile is buildable
            }
            if !auto_bulldoze || is_tile_auto_bulldozable(&current_tile) != Some(true) {
                return Ok(ToolResult::NeedBulldoze);
            }
            effects = effects
                .add_cost(EditingTool::Bulldozer.cost())
                .add_modification(&current_position, Tile::from_type(TileType::Dirt)?);
        }
    }

    Ok(ToolResult::Succeeded(effects))
}

/// Put down a new building, starting at the given position (left, top).
///
/// Building information:
/// - base_tile
///   Tile value to use at the top-left position.
/// - animation_flag
///   Set animation flag at relative position (1, 2).
fn put_down_building(
    map: &TileMap,
    position: &MapPosition,
    building_info: &BuildingInfo,
    effects: ToolEffects,
) -> Result<ToolResult, String> {
    let (width, height) = (
        building_info.size.width as i8,
        building_info.size.height as i8,
    );
    let mut tile = building_info.base_tile.clone();
    for dy in 0..height {
        for dx in 0..width {
            let tile_raw = building_info.base_tile.get_raw()
                | TILE_BURN_BIT
                | TILE_CONDUCT_BIT
                | match (dx, dy) {
                    (1, 1) => TILE_ZONE_BIT,
                    (1, 2) if building_info.is_animated => TILE_ANIM_BIT,
                    _ => 0x00,
                };
            effects.add_modification(&position.with_offset(dx, dy), Tile::from_raw(tile_raw)?);
            tile.set_raw(tile.get_raw() + 1);
        }
    }

    Ok(ToolResult::Succeeded(effects))
}

/// Build a new building, with the given position being its 'center' tile.
pub(super) fn build_building(
    map: &TileMap,
    center: &MapPosition,
    building_info: &BuildingInfo,
    effects: ToolEffects,
    auto_bulldoze: bool,
) -> Result<ToolResult, String> {
    // compute top-left 'anchor'
    let anchor = center.with_offset(-1, -1);
    // prepare building site
    if let Some(prepareResult) = effects.chain_or_return(prepare_bulding_site(
        map,
        &anchor,
        &building_info.size,
        effects,
        auto_bulldoze,
    )?) {
        return Ok(prepareResult);
    }
    // put down the building
    if let Some(buildResult) =
        effects.chain_or_return(put_down_building(map, &anchor, &building_info, effects)?)
    {
        return Ok(buildResult);
    }
    // update surrounding connections
    if let Some(connectResult) = effects.chain_or_return(check_border(
        map,
        &anchor,
        &building_info.size,
        effects,
        auto_bulldoze,
    )?) {
        return Ok(connectResult);
    }
    // all good!
    Ok(ToolResult::Succeeded(
        effects.add_cost(building_info.tool.cost()),
    ))
}
