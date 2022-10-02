use std::cmp;

use crate::{
    map::{
        tiles::TILE_LOW_MASK, Map, MapClusteringStrategy, MapPosition, MapPositionOffset,
        MapRectangle, Tile, TileMap, TileType, WORLD_HEIGHT, WORLD_WIDTH,
    },
    utils::random::MicropolisRandom,
};

use super::{sprite::ActiveSpritesList, sprite::SpriteType, zoning::ZoneType};

/// Maximum number of map tiles to drive, looking for a destination.
const MAX_TRAFFIC_LOOKUP_DISTANCE: usize = 30;

const DRIVING_DONE_TARGET_LOW: [TileType; 3] =
    [TileType::CommercialBase, TileType::House, TileType::House];
const DRIVING_DONE_TARGET_HIGH: [TileType; 3] =
    [TileType::Nuclear, TileType::Port, TileType::CommercialBase];

const PERIMETER_ROAD_EDGES_X: [i32; 12] = [-1, 0, 1, 2, 2, 2, 1, 0, -1, -2, -2, -2];
const PERIMETER_ROAD_EDGES_Y: [i32; 12] = [-2, -2, -2, -1, 0, 1, 2, 2, 2, 1, 0, -1];

pub type TrafficDensityMap = Map<u8>;

impl TrafficDensityMap {
    pub fn density_map_with_dimensions(dimensions: &MapRectangle, default_value: u8) -> Self {
        TrafficDensityMap::with_data(
            vec![vec![default_value; dimensions.get_height() / 2]; dimensions.get_width() / 2],
            MapClusteringStrategy::BlockSize2,
        )
    }
}

/// Traffic simulation.
pub struct CityTraffic {
    density_map: TrafficDensityMap,
    positions_stack_pointer: usize,
    positions_stack: [MapPosition; MAX_TRAFFIC_LOOKUP_DISTANCE],
}

impl CityTraffic {
    pub fn from_map(map: &TileMap) -> Self {
        Self {
            density_map: TrafficDensityMap::density_map_with_dimensions(&map.bounds(), 0x00),
            positions_stack: [MapPosition::new(0, 0); MAX_TRAFFIC_LOOKUP_DISTANCE],
            positions_stack_pointer: 0,
        }
    }

    pub fn get_density_map(&self) -> &TrafficDensityMap {
        &self.density_map
    }

    pub fn get_density_map_mut(&mut self) -> &mut TrafficDensityMap {
        &mut self.density_map
    }

    /// Spawn traffic starting from the road tile a the given position.
    ///
    /// Returns true if a connection was found.
    pub fn spawn_traffic_at(
        &mut self,
        rng: &mut MicropolisRandom,
        map: &TileMap,
        at: &MapPosition,
        destination_zone: &ZoneType,
        sprites: &mut ActiveSpritesList,
    ) -> Result<bool, String> {
        let position = *at;
        if self.try_driving_to(rng, map, &position, destination_zone)? {
            self.add_to_traffic_density_map(rng, map, sprites)?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// Find a connection over a road from the given position.
    ///
    /// Returns Some(true) if a connection was found, Some(false) if not and None if
    /// no connection to a road was found.
    pub fn spawn_traffic(
        &mut self,
        rng: &mut MicropolisRandom,
        map: &TileMap,
        starting_at: &MapPosition,
        destination_zone: &ZoneType,
        sprites: &mut ActiveSpritesList,
    ) -> Result<Option<bool>, String> {
        self.positions_stack_pointer = 0;

        let position = *starting_at;
        if Self::find_perimeter_road(map, &position)?.0 {
            if self.try_driving_to(rng, map, &position, destination_zone)? {
                self.add_to_traffic_density_map(rng, map, sprites)?;
                return Ok(Some(true));
            }
            return Ok(Some(false));
        }
        Ok(None)
    }

    /// Update the traffic density map from the positions stack.
    fn add_to_traffic_density_map(
        &mut self,
        rng: &mut MicropolisRandom,
        map: &TileMap,
        sprites: &mut ActiveSpritesList,
    ) -> Result<(), String> {
        while self.positions_stack_pointer > 0 {
            let position = self.stack_pop_position();
            if let Some(tile_value) = map
                .get_tile_at(&position)
                .map(|t| t.get_raw() & TILE_LOW_MASK)
            {
                if tile_value >= TileType::HorizontalBridge.to_u16().unwrap()
                    && tile_value < TileType::HorizontalPower.to_u16().unwrap()
                {
                    // update traffic density
                    let mut traffic = self.density_map.get_tile_at(&position).ok_or(format!(
                        "CityTraffic::add_to_traffic_density_map cannot get density value at {}",
                        position
                    ))? + 50;
                    traffic = cmp::min(traffic, 240);
                    self.density_map.set_tile_at(&position, traffic);

                    // check for heavy traffic
                    if traffic >= 240 && rng.get_random(5) == 0 {
                        let traffic_max = position;
                        // direct helicopter towards heavy traffic
                        if let Some(sprite) = sprites.get_sprite_mut(&SpriteType::Helicopter) {
                            if sprite.control == -1 {
                                sprite.destination = traffic_max * 16;
                            }
                        }
                    }
                }
            }
        }

        Ok(())
    }

    fn stack_push_position(&mut self, position: &MapPosition) {
        self.positions_stack_pointer += 1;
        assert!(self.positions_stack_pointer < MAX_TRAFFIC_LOOKUP_DISTANCE);
        self.positions_stack[self.positions_stack_pointer] = position.clone();
    }

    fn stack_pop_position(&mut self) -> MapPosition {
        assert!(self.positions_stack_pointer > 0);
        self.positions_stack_pointer -= 1;
        self.positions_stack[self.positions_stack_pointer]
    }

    fn can_drive_to(&self, from: &MapPosition, towards_zone: &ZoneType) -> Result<bool, String> {
        let mut current_position = from;
        for dist in 0..MAX_TRAFFIC_LOOKUP_DISTANCE {}
        todo!()
    }

    /// Find a connection to a road at the given perimeter position.
    pub fn find_perimeter_road(
        map: &TileMap,
        position: &MapPosition,
    ) -> Result<(bool, MapPosition), String> {
        for z in 0..12 {
            let t = *position + (PERIMETER_ROAD_EDGES_X[z], PERIMETER_ROAD_EDGES_Y[z]).into();
            if let Some(tile) = map.get_tile_at(&position) {
                if tile.is_driveable() {
                    return Ok((true, t));
                }
            }
        }
        Ok((false, position.clone()))
    }

    /// Try to drive to the given destination.
    fn try_driving_to(
        &mut self,
        rng: &mut MicropolisRandom,
        map: &TileMap,
        from: &MapPosition,
        destination_zone: &ZoneType,
    ) -> Result<bool, String> {
        let mut previous_direction = MapPositionOffset::None;
        let mut current_position = *from;

        for mut distance in 0..MAX_TRAFFIC_LOOKUP_DISTANCE {
            let direction =
                Self::try_random_driving(rng, map, &current_position, &previous_direction)?;
            if direction != MapPositionOffset::None {
                // road found
                current_position = direction.apply(&current_position);
                previous_direction = previous_direction.rotated_180();

                if distance & 0x01 != 0x00 {
                    self.stack_push_position(&current_position);
                }
                if Self::is_driving_done(map, &current_position, destination_zone)? {
                    return Ok(true);
                }
            } else {
                if self.positions_stack_pointer > 0 {
                    // dead end: backup
                    self.positions_stack_pointer -= 1;
                    distance += 3;
                } else {
                    return Ok(false);
                }
            }
        }

        // exceeded MAX_TRAFFIC_LOOKUP_DISTANCE
        Ok(false)
    }

    /// Try to drive one tile in a random direction.
    fn try_random_driving(
        rng: &mut MicropolisRandom,
        map: &TileMap,
        from: &MapPosition,
        from_direction: &MapPositionOffset,
    ) -> Result<MapPositionOffset, String> {
        let mut directions = [MapPositionOffset::None; 4];

        // find connections from current position
        let mut direction = MapPositionOffset::North;
        let mut count = 0;
        for i in 0..4 {
            if direction != *from_direction {
                let adjacent_tile = map.get_neighboring_tile_at(
                    from,
                    &direction,
                    Tile::from_type(TileType::Dirt).unwrap(),
                );
                if adjacent_tile.is_driveable() {
                    // found a road in an allowed direction
                    directions[i] = direction;
                    count += 1;
                } else {
                    directions[i] = MapPositionOffset::None;
                }
            } else {
                directions[i] = MapPositionOffset::None;
            }

            direction = direction.rotated_90();
        }

        // dead-end?
        if count == 0 {
            return Ok(MapPositionOffset::None);
        }

        // only one choice
        if count == 1 {
            for i in 0..4 {
                if directions[i] != MapPositionOffset::None {
                    return Ok(directions[i]);
                }
            }
        }

        // more than one choice: draw a random number
        let mut i: usize = (rng.get_random_16() as usize) & 0x03;
        while directions[i] == MapPositionOffset::None {
            i = (i + 1) & 0x03;
        }
        Ok(directions[i])
    }

    /// Has the journey arrived at its destination?
    fn is_driving_done(
        map: &TileMap,
        position: &MapPosition,
        destination_zone: &ZoneType,
    ) -> Result<bool, String> {
        let destination_zone_index = destination_zone.to_usize().unwrap();
        let (target_low, target_high) = (
            DRIVING_DONE_TARGET_LOW[destination_zone_index]
                .to_usize()
                .unwrap(),
            DRIVING_DONE_TARGET_HIGH[destination_zone_index]
                .to_usize()
                .unwrap(),
        );

        let get_low_tile_value = |map: &TileMap, pos: &MapPosition| -> Result<usize, String> {
            Ok(map
                .get_tile_at(&pos)
                .ok_or(format!(
                    "CityTraffic::is_driving_done cannot get tile map at {}",
                    pos
                ))
                .map(|t| t.get_raw() & TILE_LOW_MASK)? as usize)
        };

        let (x, y) = position.as_tuple();
        if y > 0 {
            let z = get_low_tile_value(map, &(x, y - 1).into())?;
            if z >= target_low && z <= target_high {
                return Ok(true);
            }
        }
        if x < WORLD_WIDTH as i32 - 1 {
            let z = get_low_tile_value(map, &(x + 1, y).into())?;
            if z >= target_low && z <= target_high {
                return Ok(true);
            }
        }
        if y < WORLD_HEIGHT as i32 - 1 {
            let z = get_low_tile_value(map, &(x, y + 1).into())?;
            if z >= target_low && z <= target_high {
                return Ok(true);
            }
        }
        if x > 0 {
            let z = get_low_tile_value(map, &(x - 1, y).into())?;
            if z >= target_low && z <= target_high {
                return Ok(true);
            }
        }

        Ok(false)
    }
}
