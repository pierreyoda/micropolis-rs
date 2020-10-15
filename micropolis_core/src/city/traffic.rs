use rand::Rng;

use crate::{
    map::{
        Map, MapClusteringStrategy, MapPosition, MapPositionOffset, MapRectangle, Tile, TileMap,
        TileType,
    },
    utils::random_in_range,
};

use super::zoning::ZoneType;

/// Maximum number of map tiles to drive, looking for a destination.
const MAX_TRAFFIC_LOOKUP_DISTANCE: usize = 30;

pub type TrafficDensityMap = Map<u8>;

impl TrafficDensityMap {
    pub fn density_map_with_dimensions(dimensions: &MapRectangle, default_value: u8) -> Self {
        TrafficDensityMap::with_data(
            vec![vec![default_value; dimensions.get_height()]; dimensions.get_width()],
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
    pub fn new(map: &TileMap) -> Self {
        Self {
            density_map: TrafficDensityMap::density_map_with_dimensions(&map.bounds(), 0x00),
            positions_stack: [MapPosition::new(0, 0); MAX_TRAFFIC_LOOKUP_DISTANCE],
            positions_stack_pointer: 0,
        }
    }

    pub fn spawn_traffic_at(
        &self,
        position: &MapPosition,
        destination_zone: &ZoneType,
    ) -> Result<bool, String> {
        todo!()
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

    pub fn can_drive_to(
        &self,
        from: &MapPosition,
        towards_zone: &ZoneType,
    ) -> Result<bool, String> {
        let mut current_position = from;
        for dist in 0..MAX_TRAFFIC_LOOKUP_DISTANCE {}
        todo!()
    }

    /// Try to drive to the given destination.
    fn try_driving_to<R: Rng>(
        rng: &mut R,
        map: &TileMap,
        from: &MapPosition,
        destination_zone: &ZoneType,
    ) -> Result<bool, String> {
        let mut previous_direction = MapPositionOffset::None;
        let mut current_position = from.clone();

        for distance in 0..MAX_TRAFFIC_LOOKUP_DISTANCE {
            let direction =
                Self::try_random_driving(rng, map, &current_position, &previous_direction)?;
            if direction != MapPositionOffset::None {
                // road found
                current_position = direction.apply(&current_position);
                previous_direction = previous_direction.rotated_180();
            }
        }

        // exceeded MAX_TRAFFIC_LOOKUP_DISTANCE
        Ok(false)
    }

    /// Try to drive one tile in a random direction.
    fn try_random_driving<R: Rng>(
        rng: &mut R,
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
                    &Tile::from_type(TileType::Dirt).unwrap(),
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
        let mut i: usize = random_in_range(rng, 0, u16::MAX as usize) & 0x03;
        while directions[i] == MapPositionOffset::None {
            i = (i + 1) & 0x03;
        }
        Ok(directions[i])
    }

    fn is_driving_done(
        map: &TileMap,
        position: &MapPosition,
        destination_zone: &ZoneType,
    ) -> Result<bool, String> {
        todo!()
    }
}
