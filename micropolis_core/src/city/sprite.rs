use rand::Rng;

use crate::{
    map::{MapPosition, MapRectangle, TileMap, TileType, WORLD_HEIGHT, WORLD_WIDTH},
    utils::random_in_range,
};

use super::traffic::TrafficDensityMap;

const TRAIN_CX: [i32; 4] = [0, 16, 0, -16];
const TRAIN_CY: [i32; 4] = [-16, 0, 16, 0];
const TRAIN_DX: [i32; 5] = [0, 4, 0, -4, 0];
const TRAIN_DY: [i32; 5] = [-4, 0, 4, 0, 0];
const TRAIN_PIC2: [u32; 5] = [1, 2, 1, 2, 5];

const HELI_CDX: [i32; 9] = [0, 0, 3, 5, 3, 0, -3, -5, -3];
const HELI_CDY: [i32; 9] = [0, -5, -3, 0, 3, 5, 3, 0, -3];

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SpriteType {
    Train,
    Helicopter,
    Airplane,
    Ship,
    Monster,
    Tornado,
    Explosion,
    Bus,
}

impl SpriteType {
    pub fn init_sprite<R: Rng>(
        &self,
        rng: &mut R,
        sprite: Sprite,
        max_pollution_at: &MapPosition,
    ) -> Sprite {
        use SpriteType::*;
        match self {
            &Train => {
                sprite.size = (32, 32).into();
                sprite.offset = (32, -16).into();
                sprite.hot_offset = (40, -8).into();
                sprite.frame = 1;
                sprite.direction = 4;
            }
            &Ship => {
                sprite.size = (48, 48).into();
                sprite.offset = (32, -16).into();
                sprite.hot_offset = (48, 0).into();
                sprite.frame = match sprite.position.as_tuple() {
                    (x, _) if x < (4 << 4) => 3,
                    (x, _) if x >= (((WORLD_WIDTH - 4) as i32) << 4) => 7,
                    (_, y) if y < (4 << 4) => 5,
                    (_, y) if y >= (((WORLD_HEIGHT - 4) as i32) << 4) => 1,
                    _ => 3,
                };
                sprite.direction = 10;
                sprite.new_direction = sprite.frame as usize;
                sprite.count = 1;
            }
            &Monster => {
                sprite.size = (48, 48).into();
                sprite.offset = (24, 0).into();
                sprite.hot_offset = (40, 16).into();
                sprite.frame = match sprite.position.as_tuple() {
                    (x, y) if x > (((WORLD_WIDTH as i32) << 4) / 2) => {
                        if y > (((WORLD_HEIGHT) as i32) << 4) / 2 {
                            10
                        } else {
                            7
                        }
                    }
                    (_, y) if y > (((WORLD_HEIGHT as i32) << 4) / 2) => 1,
                    _ => 4,
                };
                sprite.count = 1000;
                sprite.destination = *max_pollution_at << 4;
                sprite.origin = sprite.position;
            }
            &Helicopter => {
                sprite.size = (32, 32).into();
                sprite.offset = (32, -16).into();
                sprite.hot_offset = (40, -8).into();
                sprite.frame = 5;
                sprite.count = 1500;
                sprite.destination = MapPosition::new(
                    random_in_range(rng, 0, ((WORLD_WIDTH as i32) << 4) - 1),
                    random_in_range(rng, 0, ((WORLD_HEIGHT as i32) << 4) - 1),
                );
                sprite.origin = sprite.position - (30, 0).into();
            }
            &Airplane => {
                sprite.size = (48, 48).into();
                sprite.offset = (24, 0).into();
                sprite.hot_offset = (48, 16).into();
                if sprite.position.get_x() > ((WORLD_WIDTH as i32) - 20) << 4 {
                    sprite.position = sprite.position - (100 + 48, 0).into();
                    sprite.destination = sprite.position - (200, 0).into();
                    sprite.frame = 7;
                } else {
                    sprite.destination = sprite.position + (200, 0).into();
                    sprite.frame = 11;
                }
                sprite.destination.set_y(sprite.position.get_y());
            }
            &Tornado => {
                sprite.size = (48, 48).into();
                sprite.offset = (24, 0).into();
                sprite.hot_offset = (40, 36).into();
                sprite.count = 200;
                sprite.frame = 1;
            }
            &Explosion => {
                sprite.size = (48, 48).into();
                sprite.offset = (24, 0).into();
                sprite.hot_offset = (40, 16).into();
                sprite.frame = 1;
            }
            &Bus => {
                sprite.size = (32, 32).into();
                sprite.offset = (30, -18).into();
                sprite.hot_offset = (40, -8).into();
                sprite.direction = 1;
                sprite.frame = 1;
            }
        }
        sprite
    }

    pub fn update_sprite<R: Rng>(
        &self,
        rng: &mut R,
        map: &TileMap,
        sprite: &mut Sprite,
        sprite_cycle: u16,
    ) -> Result<(), String> {
        use SpriteType::*;
        match self {
            &Train => self.update_train(rng, map, sprite, sprite_cycle),
            &Helicopter => self.update_helicopter(rng, map, sprite, sprite_cycle),
        }
    }

    fn update_train<R: Rng>(
        &self,
        rng: &mut R,
        map: &TileMap,
        sprite: &mut Sprite,
        sprite_cycle: u16,
    ) -> Result<(), String> {
        if sprite.frame == 3 || sprite.frame == 4 {
            sprite.frame = *TRAIN_PIC2.get(sprite.direction as usize).unwrap();
        }

        sprite.position += MapPosition::new(TRAIN_DX[sprite.direction], TRAIN_DY[sprite.direction]);

        if sprite_cycle & 0x03 != 0 {
            return Ok(());
        }

        let direction = (random_in_range(rng, 0, u16::MAX) & 0x03) as usize;
        for z in direction..(direction + 4) {
            let direction2: usize = z & 0x03;
            if sprite.direction != 4 && direction2 == ((sprite.direction + 2) & 0x03) {
                continue;
            }

            let map_at = sprite.position + (TRAIN_CX[direction2] + 48, TRAIN_CY[direction2]).into();
            let char = map.get_tile_char_at(&map_at).ok_or(format!(
                "SpriteType.update_train: cannot get tile char at {}",
                map_at
            ))?;
            if (char >= TileType::UnderwaterHorizontalRail.to_u16().unwrap()
                && char <= TileType::VerticalRailRoad.to_u16().unwrap())
                || char == TileType::RailVerticalPowerHorizontal.to_u16().unwrap()
                || char == TileType::RailHorizontalPowerVertical.to_u16().unwrap()
            {
                sprite.frame = if sprite.direction != direction2 && sprite.direction != 4 {
                    if sprite.direction + direction2 == 3 {
                        3
                    } else {
                        4
                    }
                } else {
                    TRAIN_PIC2[direction2]
                };

                if char == TileType::HorizontalRailRoad.to_u16().unwrap()
                    || char == TileType::VerticalRailRoad.to_u16().unwrap()
                {
                    sprite.frame = 5;
                }

                sprite.direction = direction2;
                return Ok(());
            }
        }

        if sprite.direction == 4 {
            sprite.frame = 0;
        } else {
            sprite.direction = 4;
        }

        Ok(())
    }

    fn update_helicopter<R: Rng>(
        &self,
        rng: &mut R,
        map: &TileMap,
        sprite: &mut Sprite,
        sprite_cycle: u16,
        list: &ActiveSpritesList,
        traffic_density_map: &TrafficDensityMap,
    ) -> Result<(), String> {
        if sprite.sound_count > 0 {
            sprite.sound_count -= 1;
        }

        if sprite.control < 0 {
            if sprite.count > 0 {
                sprite.count -= 1;
            }
            if sprite.count == 0 {
                // attract copter to monster so that it blows up more often
                if let Some(monster) = list.get_sprite(&SpriteType::Monster) {
                    sprite.destination = monster.position;
                } else {
                    // attract copter to tornado so that it blows up more often
                    if let Some(tornado) = list.get_sprite(&SpriteType::Tornado) {
                        sprite.destination = tornado.position;
                    } else {
                        sprite.destination = sprite.origin;
                    }
                }
            }

            // land
            let (dist, _) = sprite.position.direction_towards(&sprite.origin);
            if dist < 30 {
                sprite.frame = 0;
            }
        } else {
            let (dist, _) = sprite.position.direction_towards(&sprite.destination);
            if dist < 16 {
                sprite.destination = sprite.origin;
                sprite.control = -1;
            }
        }

        // send report
        if sprite.sound_count == 0 {
            let at = (sprite.position + (48, 0).into()) / 16;
            if map.in_bounds(&at) {
                let chopper_position = at + (1, 1).into();
                let traffic_density =
                    *traffic_density_map
                        .get_tile_at(&chopper_position)
                        .ok_or(format!(
                            "SpriteType::update_helicopter: cannot get traffic density at {}",
                            chopper_position
                        ))?;
                if traffic_density > 170 && random_in_range(rng, 0, u16::MAX) & 0x07 == 0 {
                    // TODO: sendMessage(HEAVY_TRAFFIC, chopper_position, picture=true)
                    // TODO: makeSound("city", "HeavyTraffic", chipper_position)
                    sprite.sound_count = 200;
                }
            }
        }

        let mut z = sprite.frame as usize;
        if sprite_cycle & 0x03 == 0x00 {
            let (_, dir) = sprite.position.direction_towards(&sprite.destination);
            z = Sprite::turn_towards(z, dir.to_usize().unwrap());
            sprite.frame = z as u32;
        }

        sprite.position += (HELI_CDX[z], HELI_CDY[z]).into();

        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
/// A Sprite is a moveable, animatable entity on the map.
pub struct Sprite {
    /// Entity type of the sprite.
    pub(crate) kind: SpriteType,
    /// Name of the sprite.
    pub(crate) name: String,
    /// Current frame (0 means non-active sprite).
    pub(crate) frame: u32,
    /// Position in **pixels**,
    pub(crate) position: MapPosition,
    /// Size in **pixels**.
    pub(crate) size: MapRectangle,
    /// Offset.
    pub(crate) offset: MapPosition,
    /// Offset of the hot-sport, relative to `position`.
    pub(crate) hot_offset: MapPosition,
    /// Origin.
    pub(crate) origin: MapPosition,
    /// Destination.
    pub(crate) destination: MapPosition,
    /// TODO: ?
    pub(crate) count: u32,
    /// TODO: ?
    pub(crate) sound_count: u32,
    /// Direction.
    pub(crate) direction: usize,
    /// New direction.
    pub(crate) new_direction: usize,
    /// TODO: ?
    pub(crate) step: u32,
    /// TODO: ?
    pub(crate) flag: u32,
    /// TODO: ?
    pub(crate) control: i32,
    /// TODO: ?
    pub(crate) turn: u32,
    /// Speed.
    pub(crate) speed: u32,
    /// Acceleration.
    pub(crate) acceleration: u32,
}

impl Sprite {
    pub fn new<R: Rng>(
        rng: &mut R,
        name: String,
        kind: SpriteType,
        position: MapPosition,
        max_pollution_at: &MapPosition,
    ) -> Self {
        kind.init_sprite(
            rng,
            Self {
                kind,
                name,
                frame: 0,
                position,
                size: (0, 0).into(),
                offset: (0, 0).into(),
                hot_offset: (0, 0).into(),
                origin: (0, 0).into(),
                destination: (0, 0).into(),
                count: 0,
                sound_count: 0,
                direction: 0,
                new_direction: 0,
                step: 0,
                flag: 0,
                control: -1,
                turn: 0,
                speed: 100,
                acceleration: 0,
            },
            max_pollution_at,
        )
    }

    pub fn is_in_bounds(&self, map: &TileMap) -> bool {
        let (x, y) = (self.position + self.hot_offset).as_tuple();
        x >= 0 && y >= 0 && x < ((WORLD_WIDTH as i32) << 4) && y < ((WORLD_HEIGHT as i32) << 4)
    }

    pub fn collides_with(&self, other: &Self) -> bool {
        self.frame != 0
            && other.frame != 0
            && (self.position + self.hot_offset).distance_with(&(other.position + other.hot_offset))
                < 30
    }

    pub fn turn_towards(present_direction: usize, destination_direction: usize) -> usize {
        match (present_direction, destination_direction) {
            (p, d) if p == d => p,
            (p, d) if p < d => {
                if (d - p) < 4 {
                    p + 1
                } else {
                    p - 1
                }
            }
            (p, d) if p >= d => {
                if (p - d) < 4 {
                    p - 1
                } else {
                    p + 1
                }
            }
            (p, _) if p > 8 => 1,
            (p, _) if p < 1 => 8,
            (p, _) => p,
        }
    }
}

pub struct ActiveSpritesList {
    sprite_cycle: u16,
    pool: Vec<Sprite>,
}

impl ActiveSpritesList {
    pub fn new() -> Self {
        Self {
            sprite_cycle: 0,
            pool: vec![],
        }
    }

    /// Add a new sprite to the pool.
    pub fn add_sprite(&mut self, sprite: Sprite) {
        self.pool.push(sprite);
    }

    /// Returns the sprite of the given type, if available and active.
    pub fn get_sprite(&self, kind: &SpriteType) -> Option<&Sprite> {
        self.pool.iter().find(|s| s.kind == *kind && s.frame != 0)
    }

    /// Returns the mutable sprite of the given type, if available and active.
    pub fn get_sprite_mut(&self, kind: &SpriteType) -> Option<&mut Sprite> {
        self.pool
            .iter_mut()
            .find(|s| s.kind == *kind && s.frame != 0)
    }

    /// Destroy all sprites by de-activating them all (by setting their `frame` to 0).
    pub fn destroy_all_sprites(&mut self) {
        for sprite in self.pool.iter_mut() {
            sprite.frame = 0;
        }
    }

    /// Destroy the sprite by taking it out of the active list.
    /// TODO: break the connection between any views that are following this sprite.
    pub fn destroy_sprite(&mut self, sprite: &Sprite) {
        self.pool.retain(|s| s != sprite);
    }
}
