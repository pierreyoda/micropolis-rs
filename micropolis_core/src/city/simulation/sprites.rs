use crate::{
    city::sprite::{ActiveSpritesList, Sprite, SpriteType},
    map::MapPosition,
    utils::random::MicropolisRandom,
};

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
