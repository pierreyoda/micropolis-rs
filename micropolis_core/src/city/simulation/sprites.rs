use crate::{
    city::sprite::{ActiveSpritesList, Sprite, SpriteType},
    map::{MapPosition, TileMap, TileType},
    utils::random::MicropolisRandom,
};

const TRAIN_GROOVE_X: i32 = -39;
const TRAIN_GROOVE_Y: i32 = 6;
const BUS_GROOVE_X: i32 = -39;
const BUS_GROOVE_Y: i32 = 6;

/// Try to start a new train sprite at the given map tile.
pub fn generate_train(
    rng: &mut MicropolisRandom,
    sprites: &mut ActiveSpritesList,
    position: &MapPosition,
    total_population: u32,
) -> Result<(), String> {
    if total_population <= 20
        || sprites.get_sprite(&SpriteType::Train).is_some()
        || rng.get_random(25) != 0
    {
        return Ok(());
    }

    let sprite_position: MapPosition = (
        (position.get_x() << 4) + TRAIN_GROOVE_X,
        (position.get_y() << 4) + TRAIN_GROOVE_Y,
    )
        .into();
    make_sprite(rng, sprites, &SpriteType::Train, &sprite_position)?;
    Ok(())
}

/// Try to start a new bus sprite at the given map tile.
pub fn generate_bus(
    rng: &mut MicropolisRandom,
    sprites: &mut ActiveSpritesList,
    position: &MapPosition,
) -> Result<(), String> {
    if sprites.get_sprite(&SpriteType::Bus).is_some() || rng.get_random(25) != 0 {
        return Ok(());
    }

    let sprite_position: MapPosition = (
        (position.get_x() << 4) + BUS_GROOVE_X,
        (position.get_y() << 4) + BUS_GROOVE_Y,
    )
        .into();
    make_sprite(rng, sprites, &SpriteType::Bus, &sprite_position)?;
    Ok(())
}

/// Try to construct a new ship sprite.
pub fn generate_ship(
    rng: &mut MicropolisRandom,
    sprites: &mut ActiveSpritesList,
    map: &TileMap,
) -> Result<(), String> {
    let map_size = map.bounds();

    if rng.get_random_16() & 0x03 != 0x00 {
        for x in 4..(map_size.get_width() - 2) {
            let position: MapPosition = (x, 0).into();
            if let Some(tile) = map.get_tile_at(&position) {
                if tile.get_type() == &Some(TileType::Channel) {
                    make_ship_at(rng, sprites, &position)?;
                    return Ok(());
                }
            }
        }
    }

    if rng.get_random_16() & 0x03 != 0x00 {
        for y in 1..(map_size.get_height() - 2) {
            let position: MapPosition = (0, y).into();
            if let Some(tile) = map.get_tile_at(&position) {
                if tile.get_type() == &Some(TileType::Channel) {
                    make_ship_at(rng, sprites, &position)?;
                    return Ok(());
                }
            }
        }
    }

    if rng.get_random_16() & 0x03 != 0x00 {
        for x in 4..(map_size.get_width() - 2) {
            let position: MapPosition = (x, map_size.get_height() - 1).into();
            if let Some(tile) = map.get_tile_at(&position) {
                if tile.get_type() == &Some(TileType::Channel) {
                    make_ship_at(rng, sprites, &position)?;
                    return Ok(());
                }
            }
        }
    }

    if rng.get_random_16() & 0x03 != 0x00 {
        for y in 1..(map_size.get_height() - 2) {
            let position: MapPosition = (map_size.get_width() - 1, y).into();
            if let Some(tile) = map.get_tile_at(&position) {
                if tile.get_type() == &Some(TileType::Channel) {
                    make_ship_at(rng, sprites, &position)?;
                    return Ok(());
                }
            }
        }
    }

    Ok(())
}

fn make_ship_at(
    rng: &mut MicropolisRandom,
    sprites: &mut ActiveSpritesList,
    position: &MapPosition,
) -> Result<(), String> {
    let sprite_position: MapPosition =
        ((position.get_x() << 4) - (48 - 1), (position.get_y() << 4)).into();
    make_sprite(rng, sprites, &SpriteType::Ship, &sprite_position)?;
    Ok(())
}

/// Ensure an airplane sprite exists.
///
/// If it does not exist, create one at the given coordinates.
pub fn generate_plane(
    rng: &mut MicropolisRandom,
    sprites: &mut ActiveSpritesList,
    position: &MapPosition,
) -> Result<(), String> {
    if sprites.get_sprite(&SpriteType::Airplane).is_some() {
        return Ok(());
    }

    let sprite_position: MapPosition =
        ((position.get_x() << 4) + 48, (position.get_y() << 4) + 12).into();
    make_sprite(rng, sprites, &SpriteType::Airplane, &sprite_position)?;
    Ok(())
}

/// Ensure a helicopter sprite exists.
///
/// If it does not exist, create one at the given coordinates.
pub fn generate_copter(
    rng: &mut MicropolisRandom,
    sprites: &mut ActiveSpritesList,
    position: &MapPosition,
) -> Result<(), String> {
    if sprites.get_sprite(&SpriteType::Helicopter).is_some() {
        return Ok(());
    }

    let sprite_position: MapPosition =
        ((position.get_x() << 4), (position.get_y() << 4) + 30).into();
    make_sprite(rng, sprites, &SpriteType::Helicopter, &sprite_position)?;
    Ok(())
}

/// Make a sprite either by re-using the old one, or by making a new one.
pub fn make_sprite(
    rng: &mut MicropolisRandom,
    sprites: &mut ActiveSpritesList,
    kind: &SpriteType,
    position: &MapPosition,
) -> Result<Sprite, String> {
    if let Some(sprite) = sprites.get_sprite_mut(kind) {
        kind.init_sprite(rng, *sprite, None)
    } else {
        let sprite = Sprite::new(rng, "".into(), kind, *position, None)?;
        sprites.add_sprite(sprite);
        Ok(sprite)
    }
}
