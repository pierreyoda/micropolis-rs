use crate::map::{
    Map, MapClusteringStrategy, MapPosition, MapPositionOffset, MapRectangle, TileMap,
    MAP_POSITION_DIRECTIONS, WORLD_HEIGHT, WORLD_WIDTH,
};

/// Size of the power stack.
const POWER_STACK_SIZE: usize = (WORLD_WIDTH * WORLD_HEIGHT) / 4;

/// Number of tiles that a coal power plant can supply power to.
const COAL_POWER_STRENGTH: usize = 700;

/// Number of tiles that a nuclear power plant can supply power to.
const NUCLEAR_POWER_STRENGTH: usize = 2000;

type PowerMap = Map<u8>;

impl PowerMap {
    pub fn powermap_with_dimensions(dimensions: &MapRectangle, default_value: u8) -> Self {
        PowerMap::with_data(
            vec![vec![default_value; dimensions.get_height()]; dimensions.get_width()],
            MapClusteringStrategy::BlockSize1,
        )
    }
}

pub struct CityPower {
    pub(crate) power_grid_map: PowerMap,
    /// Number of coal power plants on the map.
    pub(crate) coal_generators_count: usize,
    /// Number of nuclear power plants on the map.
    pub(crate) nuclear_generators_count: usize,
    /// Stack used to find powered tiles by tracking conductive tiles.
    ///
    /// Points to top-most item in the power stack.
    power_stack_pointer: usize,
    /// Stack of map positions for traversing setting the power grid.
    power_stack: [MapPosition; POWER_STACK_SIZE],
    /// Number of powered tiles in all zones.
    powered_zone_count: i16,
    /// Number of unpowered tiles in all zones.
    unpowered_zone_count: i16,
}

impl CityPower {
    pub fn from_map(map: &TileMap) -> Self {
        CityPower {
            power_grid_map: PowerMap::powermap_with_dimensions(&map.bounds(), 0x00),
            coal_generators_count: 0,
            nuclear_generators_count: 0,
            power_stack_pointer: 0,
            power_stack: [MapPosition::new(0, 0); POWER_STACK_SIZE],
            // TODO: tracking of these values
            powered_zone_count: 0,
            unpowered_zone_count: 0,
        }
    }

    pub fn get_powered_zone_count(&self) -> i16 {
        return self.powered_zone_count;
    }

    pub fn get_unpowered_zone_count(&self) -> i16 {
        return self.unpowered_zone_count;
    }

    /// Push the given position onto the power stack if there is room.
    pub fn push_power_stack(&mut self, position: MapPosition) {
        if self.power_stack_pointer < (POWER_STACK_SIZE - 2) {
            self.power_stack_pointer += 1;
            self.power_stack[self.power_stack_pointer] = position;
        }
    }

    /// Pull a position from the power stack.
    pub fn pull_power_stack(&mut self) -> Option<MapPosition> {
        assert!(self.power_stack_pointer > 0);
        self.power_stack_pointer -= 1;
        self.power_stack.get(self.power_stack_pointer + 1).cloned()
    }

    /// Checks at the given position for a power-less conducting tile in the given direction.
    pub fn test_for_conductive(
        &self,
        map: &TileMap,
        position: &MapPosition,
        direction: &MapPositionOffset,
    ) -> bool {
        if let Some(moved_position) = direction.apply_with_bounds(position, &map.bounds()) {
            if let Some(tile) = map.get_tile_at(&moved_position) {
                if tile.is_conductive() {
                    if let Some(power_tile) = self.power_grid_map.get_tile_at(&moved_position) {
                        return *power_tile == 0x00;
                    }
                }
            }
        }
        false
    }

    /// Scans the map for powered tiles, and copy them to the `power_grid_map`.
    ///
    /// Also warns the user about using too much power.
    pub fn do_power_scan(&mut self, map: &TileMap) {
        let map_bounds = map.bounds();
        // clear power map
        self.power_grid_map = PowerMap::powermap_with_dimensions(&map_bounds, 0x00);

        // combined coal+nuclear power plants deliverable power
        let max_power = self.coal_generators_count * COAL_POWER_STRENGTH
            + self.nuclear_generators_count * NUCLEAR_POWER_STRENGTH;
        let mut power_count: usize = 0;

        while self.power_stack_pointer > 0 {
            let mut position = self.pull_power_stack().unwrap();
            let mut direction = MapPositionOffset::None;
            'inner: loop {
                power_count += 1;
                if power_count > max_power {
                    // TODO: send MESSAGE_NOT_ENOUGH_POWER
                    return;
                }
                if direction != MapPositionOffset::None {
                    position = direction.apply_with_bounds(&position, &map_bounds).unwrap();
                }
                self.power_grid_map.set_tile_at(&position, 0x01);
                let mut connections_count: usize = 0;
                for current_direction in &MAP_POSITION_DIRECTIONS {
                    if connections_count >= 2 {
                        break;
                    }
                    if self.test_for_conductive(map, &position, current_direction) {
                        connections_count += 1;
                        direction = current_direction.clone();
                    }
                }
                if connections_count > 1 {
                    self.push_power_stack(position);
                } else if connections_count == 0 {
                    break 'inner;
                }
            }
        }
    }
}
