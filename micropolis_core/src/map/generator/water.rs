use crate::{
    map::{
        tiles::TILE_LOW_MASK,
        tiles_type::{WOODS_HIGH, WOODS_LOW},
        MapPosition, MapPositionOffset, TileMap, TileType,
    },
    utils::random::MicropolisRandom,
};

use super::{
    constants::{
        BLOB_RIVER_BIG, BLOB_RIVER_SMALL, SMOOTH_RIVER_EDGES_TABLE, SMOOTH_TILES_DX,
        SMOOTH_TILES_DY,
    },
    utils::{put_tile_on_terrain, random_river_direction},
};

/// Generate a random number of lakes, depending on `level_lakes`.
pub fn make_lakes(rng: &mut MicropolisRandom, level_lakes: i16, map: &mut TileMap) {
    let mut remaining_lakes = if level_lakes < 0 {
        rng.get_random(10)
    } else {
        level_lakes / 2
    };

    let map_size = map.bounds();
    while remaining_lakes > 0 {
        let x = 10 + rng.get_random(map_size.width as i16 - 21);
        let y = 10 + rng.get_random(map_size.height as i16 - 20);
        make_single_lake(rng, map, (x as i32, y as i32).into());
        remaining_lakes -= 1;
    }
}

/// Generate a single lake around the given rough position.
fn make_single_lake(rng: &mut MicropolisRandom, terrain: &mut TileMap, at: MapPosition) {
    let mut num_plops = 2 + rng.get_random(12);
    while num_plops > 0 {
        let plop_position = at
            + (
                (rng.get_random(12) - 6) as i32,
                (rng.get_random(12) - 6) as i32,
            )
                .into();

        if rng.get_random(4) != 0 {
            plop_small_river(terrain, &plop_position)
        } else {
            plop_big_river(terrain, &plop_position)
        };
        num_plops -= 1;
    }
}

/// Put down a big diamond-like shaped river, where `base` is the top-left position of the blob.
pub fn plop_big_river(terrain: &mut TileMap, base: &MapPosition) {
    for x in 0..9 {
        for y in 0..9 {
            let position = *base + (x, y).into();
            if !terrain.in_bounds(&position) {
                continue;
            }
            put_tile_on_terrain(terrain, BLOB_RIVER_BIG[y][x].clone(), &position)
                .expect("MapGenerator.plop_big_river set tile error");
        }
    }
}

/// Put down a small diamond-like shaped river, where `base` is the top-left position of the blob.
pub fn plop_small_river(terrain: &mut TileMap, base: &MapPosition) {
    for x in 0..6 {
        for y in 0..6 {
            let position = *base + (x, y).into();
            if !terrain.in_bounds(&position) {
                continue;
            }
            put_tile_on_terrain(terrain, BLOB_RIVER_SMALL[y][x].clone(), &position)
                .expect("MapGenerator.plop_small_river set tile error");
        }
    }
}

pub fn make_rivers(
    rng: &mut MicropolisRandom,
    level_river_curves: i16,
    terrain: &mut TileMap,
    start: &MapPosition,
) {
    let mut global_direction = random_river_direction(rng);
    make_big_river(
        rng,
        level_river_curves,
        terrain,
        start,
        &global_direction,
        &global_direction,
    );

    global_direction = global_direction.rotated_180();
    let local_direction = make_big_river(
        rng,
        level_river_curves,
        terrain,
        start,
        &global_direction,
        &global_direction,
    );

    global_direction = random_river_direction(rng);
    make_small_river(
        rng,
        level_river_curves,
        terrain,
        start,
        &global_direction,
        &local_direction,
    );
}

/// Make a big river.
///
/// Parameters:
/// - `global_direction` is the global direction of the river
/// - `local_direction` is the local direction of the terrain
///
/// Returns the last local river direction.
fn make_big_river(
    rng: &mut MicropolisRandom,
    level_river_curves: i16,
    terrain: &mut TileMap,
    start: &MapPosition,
    global_direction: &MapPositionOffset,
    local_direction: &MapPositionOffset,
) -> MapPositionOffset {
    let (rate1, rate2) = match level_river_curves {
        level if level < 0 => (100, 200),
        level => (10 + level, 100 + level),
    };

    let mut position = *start;
    let mut last_local_direction = *local_direction;
    while terrain.in_bounds(&MapPosition {
        x: position.x + 4,
        y: position.y + 4,
    }) {
        plop_big_river(terrain, &position);
        if rng.get_random(rate1) < 10 {
            last_local_direction = *global_direction;
        } else {
            if rng.get_random(rate2) > 90 {
                last_local_direction = last_local_direction.rotated_45();
            }
            if rng.get_random(rate2) > 90 {
                last_local_direction = last_local_direction.rotated_45_times(7);
            }
        }
        position = last_local_direction.apply(&position);
    }
    last_local_direction
}

// TODO: factorize code with make_big_river (macro/closures)
fn make_small_river(
    rng: &mut MicropolisRandom,
    level_river_curves: i16,
    terrain: &mut TileMap,
    start: &MapPosition,
    global_direction: &MapPositionOffset,
    local_direction: &MapPositionOffset,
) -> MapPositionOffset {
    let (rate1, rate2) = match level_river_curves {
        level if level < 0 => (100, 200),
        level => (10 + level, 100 + level),
    };

    let mut position = *start;
    let mut last_local_direction = *local_direction;
    while terrain.in_bounds(&MapPosition {
        x: position.x + 3,
        y: position.y + 3,
    }) {
        plop_small_river(terrain, &position);
        if rng.get_random(rate1) < 10 {
            last_local_direction = *global_direction;
        } else {
            if rng.get_random(rate2) > 90 {
                last_local_direction = last_local_direction.rotated_45();
            }
            if rng.get_random(rate2) > 90 {
                last_local_direction = last_local_direction.rotated_45_times(7);
            }
        }
        position = local_direction.apply(&position);
    }
    last_local_direction
}

pub fn smooth_rivers(rng: &mut MicropolisRandom, terrain: &mut TileMap) -> Result<(), String> {
    let map_size = terrain.bounds();
    let dirt_type_raw = TileType::Dirt
        .to_u16()
        .ok_or("Dirt tile type raw conversion error")?;
    let river_type_raw = TileType::River
        .to_u16()
        .ok_or("River tile type raw conversion error")?;
    for x in 0..map_size.width {
        for y in 0..map_size.height {
            {
                // avoid immutable / mutable borrow conflict
                // TODO: find better way
                let tile = terrain
                    .data
                    .get(x)
                    .ok_or(format!(
                        "MapGenerator.smooth_rivers map X overflow at {}",
                        x
                    ))?
                    .get(y)
                    .ok_or(format!(
                        "MapGenerator.smooth_rivers map Y overflow at {}",
                        y
                    ))?;
                if tile.get_type() != &Some(TileType::RiverEdge) {
                    continue;
                }
            }

            let position: MapPosition = (x as i32, y as i32).into();
            let mut bit_index = 0;

            for i in 0..4 {
                bit_index <<= 1;
                let temp_position = position + (SMOOTH_TILES_DX[i], SMOOTH_TILES_DY[i]).into();
                if !terrain.in_bounds(&temp_position) {
                    continue;
                }
                if let Some(temp_tile) = terrain.get_tile_at(&temp_position) {
                    let temp_tile_type_raw = temp_tile.get_type_raw() & TILE_LOW_MASK;
                    if temp_tile_type_raw == dirt_type_raw {
                        continue;
                    }
                    if !(WOODS_LOW..=WOODS_HIGH).contains(&temp_tile_type_raw) {
                        bit_index += 1;
                    }
                }
            }

            if let Some(tile) = terrain.get_tile_mut_at(&position) {
                let mut tile_raw = SMOOTH_RIVER_EDGES_TABLE[bit_index & 0x000F];
                if tile_raw != river_type_raw && rng.get_random(1) != 0 {
                    tile_raw += 1;
                }
                tile.set_raw(tile_raw);
            }
        }
    }
    Ok(())
}
