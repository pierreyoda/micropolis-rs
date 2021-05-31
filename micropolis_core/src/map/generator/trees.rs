use crate::{
    map::{
        tiles::{TILE_BLBNBIT_MASK, TILE_BURN_BULL_BIT},
        tools::ToolEffects,
        MapPosition, Tile, TileMap, TileType,
    },
    utils::random::MicropolisRandom,
};

use super::{
    constants::{SMOOTH_FOREST_EDGES_TABLE, SMOOTH_TILES_DX, SMOOTH_TILES_DY},
    utils::random_direction,
};

pub fn make_forests(
    rng: &mut MicropolisRandom,
    level_trees: i16,
    terrain: &mut TileMap,
) -> Result<(), String> {
    let amount = match level_trees {
        level if level < 0 => 50 + rng.get_random(100),
        level => 3 + level,
    };
    let map_size = terrain.bounds();
    for _ in 0..amount {
        let x = rng.get_random(map_size.width as i16 - 1);
        let y = rng.get_random(map_size.height as i16 - 1);
        splash_trees(rng, level_trees, terrain, &(x, y).into());
    }

    smooth_trees(terrain)?;
    smooth_trees(terrain)?; // TODO: why the repetition ?

    Ok(())
}

/// Splash a bunch of trees near the given position.
///
/// The amount of trees generated depends on `level_trees`.
/// Note: trees are not smoothed.
///
/// TODO: function generates trees even if `level_trees` is 0 (original bug).
fn splash_trees(
    rng: &mut MicropolisRandom,
    level_trees: i16,
    terrain: &mut TileMap,
    at: &MapPosition,
) {
    let mut trees_count = match level_trees {
        level if level < 0 => 50 + rng.get_random(150),
        level => 50 + rng.get_random(100 + level * 2),
    };

    let mut tree_position = *at;
    let woods_type_raw = TileType::Woods.to_u16().unwrap();
    while trees_count > 0 {
        let direction = random_direction(rng);
        tree_position = direction.apply(&tree_position);

        if !terrain.in_bounds(&tree_position) {
            return;
        }
        if let Some(tile) = terrain.get_tile_mut_at(&tree_position) {
            if tile.get_type() == &Some(TileType::Dirt) {
                tile.set_raw(woods_type_raw | TILE_BLBNBIT_MASK);
            }
        }
        trees_count -= 1;
    }
}

fn smooth_trees(terrain: &mut TileMap) -> Result<(), String> {
    let map_size = terrain.bounds();
    let dirt_type_raw = TileType::Dirt
        .to_u16()
        .ok_or("Dirt tile type raw conversion error")?;
    let woods_type_raw = TileType::Woods
        .to_u16()
        .ok_or("Woods tile type raw conversion error")?;
    for x in 0..map_size.width {
        for y in 0..map_size.height {
            {
                // avoid immutable / mutable borrow conflict
                // TODO: find better way
                let tile = terrain
                    .data
                    .get_mut(x)
                    .ok_or(format!("MapGenerator.smooth_trees map overflow at X={}", x,))?
                    .get_mut(y)
                    .ok_or(format!("MapGenerator.smooth_trees map overflow at Y={}", y))?;
                if !tile.is_tree() {
                    continue;
                }
            }
            let mut bit_index = 0;
            for i in 0..4 {
                bit_index <<= 1;
                let temp_position = MapPosition {
                    x: x as i32 + SMOOTH_TILES_DX[i],
                    y: y as i32 + SMOOTH_TILES_DY[i],
                };
                if !map_size.is_inside(&temp_position) {
                    continue;
                }
                let temp_tile = terrain
                    .data
                    .get(temp_position.x as usize)
                    .ok_or(format!(
                        "MapGenerator.smooth_trees map X overflow at temp X={}",
                        temp_position.x
                    ))?
                    .get(temp_position.y as usize)
                    .ok_or(format!(
                        "MapGenerator.smooth_trees map Y overflow at temp Y={}",
                        temp_position.y
                    ))?;
                if temp_tile.is_tree() {
                    bit_index += 1;
                }
            }
            let tile = terrain.data.get_mut(x).unwrap().get_mut(y).unwrap();
            let mut tile_raw = SMOOTH_FOREST_EDGES_TABLE[bit_index & 0x000F];
            if tile_raw == dirt_type_raw {
                tile.set_type_raw(tile_raw);
            } else {
                if tile_raw != woods_type_raw && (x + y) & 0x1 == 0x1 {
                    tile_raw -= 8;
                }
                tile.set_raw(TILE_BLBNBIT_MASK | tile_raw);
            }
        }
    }
    Ok(())
}

pub fn smooth_trees_at(
    terrain: &TileMap,
    position: &MapPosition,
    effects: ToolEffects,
    preserve: bool,
) -> Result<ToolEffects, String> {
    if !effects
        .get_map_value_at(terrain, position)
        .ok_or(format!(
            "MapGenerator.smooth_trees_at cannot read effects tile value at {}",
            position
        ))?
        .is_tree()
    {
        return Ok(effects);
    }

    let mut bit_index: u16 = 0;
    for z in 0..4 {
        bit_index <<= 0x01;
        if terrain
            .in_bounds(&(*position + MapPosition::new(SMOOTH_TILES_DX[z], SMOOTH_TILES_DY[z])))
        {
            bit_index += 1;
        }
    }

    let table_index = (bit_index & 0x0F) as usize;
    let temp = *SMOOTH_FOREST_EDGES_TABLE.get(table_index).ok_or(format!(
        "MapGenerator.smooth_trees_at SMOOTH_FOREST_EDGES_TABLE overflow: {}",
        table_index
    ))?;
    match temp {
        0 => Ok(effects.add_modification(
            position,
            Tile::from_raw(
                TILE_BURN_BULL_BIT
                    | if TileType::from_u16(temp).ok_or(format!(
                        "MapGenerator.smooth_trees_at cannot create tile from {}",
                        temp
                    ))? != TileType::Woods
                        && (position.x + position.y) & 0x01 != 0x00
                    {
                        temp - 8
                    } else {
                        temp
                    },
            )?,
        )),
        _ => Ok(if preserve {
            effects
        } else {
            effects.add_modification(position, Tile::from_raw(temp)?)
        }),
    }
}
