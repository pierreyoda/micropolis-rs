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

pub fn random_river_direction(rng: &mut MicropolisRandom) -> MapPositionOffset {
    use MapPositionOffset::*;
    match 1 + 2 * rng.get_random(3) {
        1 => North,
        2 => NorthEast,
        3 => East,
        4 => SouthEast,
        5 => South,
        6 => SouthWest,
        7 => West,
        8 => NorthWest,
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

/// Put the given tile type on the map terrain, **if possible**.
pub fn put_tile_on_terrain(
    terrain: &mut TileMap,
    new_tile_type: TileType,
    at: &MapPosition,
) -> Result<(), String> {
    if new_tile_type == TileType::Dirt {
        return Ok(());
    }
    let row = terrain.data.get_mut(at.x as usize);
    if row.is_none() {
        return Ok(());
    }

    let tile_option = row.unwrap().get_mut(at.y as usize);
    if tile_option.is_none() {
        return Ok(());
    }

    let tile = tile_option.unwrap();
    match tile.get_type() {
        Some(TileType::Dirt) => tile.set_type(new_tile_type),
        Some(TileType::River) if new_tile_type != TileType::Channel => Ok(()),
        Some(TileType::Channel) => Ok(()),
        _ => tile.set_type(new_tile_type),
    }
}
