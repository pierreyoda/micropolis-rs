use crate::map::{connect::TileMapConnector, MapPosition, MapRectangle, Tile, TileMap, TileType};

use super::{ConnectTileCommand, ToolEffects, ToolResult};

/// Check and connect a new zone around the border.
pub(super) fn check_border(
    map: &TileMap,
    position: &MapPosition,
    zone_size: &MapRectangle,
    effects: ToolEffects,
    auto_bulldoze: bool,
) -> Result<ToolResult, String> {
    let (zone_width, zone_height) = (zone_size.width as i8, zone_size.height as i8);

    for top in 0..zone_width {
        if let Some(result) = effects.chain_or_return(TileMapConnector::connect_tile(
            map,
            &position.with_offset(top, -1),
            &ConnectTileCommand::Fix,
            effects,
            auto_bulldoze,
        )?) {
            return Ok(result);
        }
    }

    for left in 0..zone_height {
        if let Some(result) = effects.chain_or_return(TileMapConnector::connect_tile(
            map,
            &position.with_offset(-1, left),
            &ConnectTileCommand::Fix,
            effects,
            auto_bulldoze,
        )?) {
            return Ok(result);
        }
    }

    for bottom in 0..zone_width {
        if let Some(result) = effects.chain_or_return(TileMapConnector::connect_tile(
            map,
            &position.with_offset(bottom, zone_height),
            &ConnectTileCommand::Fix,
            effects,
            auto_bulldoze,
        )?) {
            return Ok(result);
        }
    }

    for right in 0..zone_height {
        if let Some(result) = effects.chain_or_return(TileMapConnector::connect_tile(
            map,
            &position.with_offset(zone_width, right),
            &ConnectTileCommand::Fix,
            effects,
            auto_bulldoze,
        )?) {
            return Ok(result);
        }
    }

    Ok(ToolResult::Succeeded(effects))
}

/// Computes the size of the zone that the tile belongs to, or 0 if
/// unknown tile value.
pub(super) fn compute_size(tile: &Tile) -> Option<u8> {
    match (*tile.get_type())?.to_u16()? {
        t if (t >= offseted_raw_type(&TileType::ResidentialBase, -1)?
            && t <= offseted_raw_type(&TileType::PortBase, -1)?)
            || (t >= offseted_raw_type(&TileType::LastPowerPlant, 1)?
                && t <= offseted_raw_type(&TileType::PoliceStation, 4)?)
            || (t >= TileType::Church1Base.to_u16()? && t <= TileType::Church7Last.to_u16()?) =>
        {
            Some(3)
        }
        t if (t >= TileType::PortBase.to_u16()? && t <= TileType::LastPort.to_u16()?)
            || (t >= TileType::CoalBase.to_u16()? && t <= TileType::LastPowerPlant.to_u16()?)
            || (t >= TileType::StadiumBase.to_u16()? && t <= TileType::LastZone.to_u16()?) =>
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
pub(super) fn check_big_zone(tile: &Tile) -> Option<(MapPosition, u8)> {
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
pub(super) fn is_tile_auto_bulldozable(tile: &Tile) -> Option<bool> {
    let raw = tile.get_raw();
    Some(
        (raw >= TileType::FirstRiverEdge.to_u16()? && raw <= TileType::LastRubble.to_u16()?)
            || (raw >= TileType::HorizontalPower.to_u16()? + 2
                && raw <= TileType::HorizontalPower.to_u16()? + 12)
            || (raw >= TileType::TINYEXP.to_u16()? && raw <= TileType::LASTTINYEXP.to_u16()? + 2),
    )
}

pub(super) fn offseted_raw_type(tile_type: &TileType, offset: i8) -> Option<u16> {
    Some((tile_type.to_u16()? as i32 + offset as i32) as u16)
}
