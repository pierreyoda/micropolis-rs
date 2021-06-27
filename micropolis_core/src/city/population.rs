use crate::map::{Map, MapClusteringStrategy, MapPosition, MapRectangle, TileMap};

pub type PopulationDensityMap = Map<u8>;

impl PopulationDensityMap {
    pub fn with_dimensions(dimensions: &MapRectangle, default_value: u8) -> Self {
        Self::with_data(
            vec![vec![default_value; dimensions.get_height()]; dimensions.get_width()],
            MapClusteringStrategy::BlockSize2,
        )
    }
}

pub struct CityPopulation {
    /// Population density map.
    density_map: Map<u8>,
    /// Number of people in residential zones.
    ///
    /// Depends on the level of zone development.
    residential: u16,
    /// Number of people in commercial zones.
    ///
    /// Depends on the level of zone development.
    commercial: u16,
    /// Number of people in industrial zones.
    ///
    /// Depends on the level of zone development.
    industrial: u16,
    /// Total population.
    ///
    /// Formula = (residential population) / 8 + (commercial population) + (industrial population).
    total: u32,
}

impl CityPopulation {
    pub fn from_map(map: &TileMap) -> Self {
        Self {
            density_map: PopulationDensityMap::with_dimensions(&map.bounds(), 0),
            residential: 0,
            commercial: 0,
            industrial: 0,
            total: 0,
        }
    }

    pub fn get_density_map(&self) -> &Map<u8> {
        &self.density_map
    }
    pub fn get_density_map_mut(&mut self) -> &mut Map<u8> {
        &mut self.density_map
    }

    /// Get the population density at the position between (0, 0) and
    /// (WORLD_WIDTH / 2, WORLD_HEIGHT / 2).
    pub fn get_density_at(&self, at: &MapPosition) -> u8 {
        *self.density_map.get_tile_at(at).unwrap_or(&0)
    }

    /// Set the population density at the position between (0, 0) and
    /// (WORLD_WIDTH / 2, WORLD_HEIGHT / 2).
    pub fn set_density_at(&mut self, at: &MapPosition, density: u8) -> bool {
        self.density_map.set_tile_at(at, density)
    }

    pub fn get_residential(&self) -> u16 {
        self.residential
    }

    pub fn get_commercial(&self) -> u16 {
        self.commercial
    }

    pub fn get_industrial(&self) -> u16 {
        self.industrial
    }

    pub fn total_population(&self) -> u32 {
        self.total
    }
}
