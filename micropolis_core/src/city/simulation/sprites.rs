use crate::{
    city::sprite::{ActiveSpritesList, Sprite, SpriteType},
    map::{tiles::TILE_BULL_BIT, MapPosition, TileMap, TileType},
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
    make_sprite(rng, sprites, &SpriteType::Train, &sprite_position)
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
    make_sprite(rng, sprites, &SpriteType::Bus, &sprite_position)
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
    make_sprite(rng, sprites, &SpriteType::Ship, &sprite_position)
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
    make_sprite(rng, sprites, &SpriteType::Airplane, &sprite_position)
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
    make_sprite(rng, sprites, &SpriteType::Helicopter, &sprite_position)
}

// Ensure a tornado sprite exists.
pub fn make_tornado(
    rng: &mut MicropolisRandom,
    sprites: &mut ActiveSpritesList,
    map: &TileMap,
) -> Result<(), String> {
    if let Some(sprite) = sprites.get_sprite_mut(&SpriteType::Tornado) {
        sprite.count = 200;
        return Ok(());
    }

    let bounds = map.bounds();
    let sprite_position: MapPosition = (
        rng.get_random((bounds.get_width() << 4) as i16 - 800) + 400,
        rng.get_random((bounds.get_height() << 4) as i16 - 200) + 100,
    )
        .into();
    make_sprite(rng, sprites, &SpriteType::Tornado, &sprite_position)
}

/// Start a new monster sprite.
///
/// TODO: make monster over land, because it disappears if it's made over water.
/// Better yet make monster not disappear for a while after it's created,
/// over land or water. Should never disappear prematurely.
pub fn make_monster(
    rng: &mut MicropolisRandom,
    sprites: &mut ActiveSpritesList,
    map: &TileMap,
    maximum_pollution_at: &MapPosition,
) -> Result<(), String> {
    let mut done = false;
    let mut position: MapPosition = (0, 0).into();

    if let Some(sprite) = sprites.get_sprite_mut(&SpriteType::Monster) {
        sprite.sound_count = 1;
        sprite.count = 1000;
        sprite.destination = (
            maximum_pollution_at.get_x() << 4,
            maximum_pollution_at.get_y() << 4,
        )
            .into();
        return Ok(());
    }

    let bounds = map.bounds();
    let river_tile_type_value = TileType::River.to_u16().unwrap();
    for z in 0..300 {
        position = (
            rng.get_random(bounds.get_width() as i16 - 20) + 10,
            rng.get_random(bounds.get_height() as i16 - 10) + 5,
        )
            .into();
        if let Some(tile) = map.get_tile_at(&position) {
            let tile_raw = tile.get_raw();
            if tile_raw == river_tile_type_value
                || tile_raw == river_tile_type_value + TILE_BULL_BIT
            {
                make_monster_at(rng, sprites, &position)?;
                done = true;
                break;
            }
        }
    }

    if done {
        Ok(())
    } else {
        make_monster_at(rng, sprites, &(60, 50).into())
    }
}

/// Start a new monster sprite at the given map tile.
fn make_monster_at(
    rng: &mut MicropolisRandom,
    sprites: &mut ActiveSpritesList,
    position: &MapPosition,
) -> Result<(), String> {
    let monster_position: MapPosition =
        ((position.get_x() << 4) + 48, position.get_y() << 4).into();
    make_sprite(rng, sprites, &SpriteType::Monster, &monster_position)?;
    // TODO: sendMessage(MESSAGE_MONSTER_SIGHTED, x + 5, y, true, true)
    Ok(())
}

/// Make a sprite either by re-using the old one, or by making a new one.
pub fn make_sprite(
    rng: &mut MicropolisRandom,
    sprites: &mut ActiveSpritesList,
    kind: &SpriteType,
    position: &MapPosition,
) -> Result<(), String> {
    if let Some(sprite) = sprites.get_sprite_mut(kind) {
        kind.init_sprite(rng, sprite, None);
    } else {
        let sprite = Sprite::new(rng, "".into(), kind, *position, None)?;
        sprites.add_sprite(sprite);
    }
    Ok(())
}
