use crate::{
    map::{MapPosition, MapPositionOffset, TileMap, TileType},
    utils::random::MicropolisRandom,
};

pub fn random_direction(rng: &mut MicropolisRandom) -> MapPositionOffset {
    use MapPositionOffset::*;
    match rng.get_random(7) {
        0 => NorthWest,
        1 => North,
        2 => NorthEast,
        3 => East,
        4 => SouthEast,
        5 => South,
        6 => SouthWest,
        7 => West,
        _ => unreachable!(),
    }
}

pub fn random_straight_direction(rng: &mut MicropolisRandom) -> MapPositionOffset {
    use MapPositionOffset::*;
    match rng.get_random(3) {
        0 => North,
        1 => East,
        2 => South,
        3 => West,
        _ => unreachable!(),
    }
}

/// Put the given tile type on the map terrain.
pub fn put_tile_on_terrain(
    terrain: &mut TileMap,
    new_tile_type: TileType,
    at: &MapPosition,
) -> Result<(), String> {
    if new_tile_type == TileType::Dirt {
        return Ok(());
    }
    let row = terrain
        .data
        .get_mut(at.x as usize)
        .ok_or("MapGenerator.set_tile map X overflow")?;
    let tile = row
        .get_mut(at.y as usize)
        .ok_or("MapGenerator.set_tile map Y overflow")?;
    match tile.get_type() {
        Some(TileType::Dirt) => tile.set_type(new_tile_type),
        Some(TileType::River) if new_tile_type != TileType::Channel => Ok(()),
        Some(TileType::Channel) => Ok(()),
        _ => tile.set_type(new_tile_type),
    }
}
