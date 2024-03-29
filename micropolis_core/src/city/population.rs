use crate::map::{Map, MapClusteringStrategy, MapPosition, MapRectangle, TileMap};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CityClass {
    /// Population <= 2k citizens.
    Village,
    /// Population <= 10k citizens.
    Town,
    /// Population <= 50k citizens.
    City,
    /// Population <= 100k citizens.
    Capital,
    /// Population <= 500k citizens.
    Metropolis,
    /// Population > 500k citizens.
    Megalopolis,
}

impl CityClass {
    pub fn from_total_population(total_population: i64) -> Self {
        match total_population {
            n if n <= 2000 => CityClass::Village,
            n if n <= 10000 => CityClass::Town,
            n if n <= 50000 => CityClass::City,
            n if n <= 100000 => CityClass::Capital,
            n if n <= 500000 => CityClass::Metropolis,
            _ => CityClass::Megalopolis,
        }
    }
}

pub type PopulationDensityMap = Map<u8>;

impl PopulationDensityMap {
    pub fn with_dimensions(dimensions: &MapRectangle, default_value: u8) -> Self {
        Self::with_data(
            vec![vec![default_value; dimensions.get_height()]; dimensions.get_width()],
            MapClusteringStrategy::BlockSize2,
        )
    }
}

// TODO: dedicated struct for valves?
pub struct CityPopulation {
    /// Population density map.
    density_map: Map<u8>,
    /// Number of people in residential zones.
    ///
    /// Depends on the level of zone development.
    residential: u16,
    /// Block residential growth?
    residential_cap: bool,
    residential_valve: i16,
    /// Number of people in commercial zones.
    ///
    /// Depends on the level of zone development.
    commercial: u16,
    /// Block commercial growth?
    commercial_cap: bool,
    commercial_valve: i16,
    /// Number of people in industrial zones.
    ///
    /// Depends on the level of zone development.
    industrial: u16,
    /// Block industrial growth?
    industrial_cap: bool,
    industrial_valve: i16,
    /// Total city population.
    ///
    /// Formula = (residential population) / 8 + (commercial population) + (industrial population).
    total: i64,
    /// Change in the total city population.
    total_delta: i64,
}

impl CityPopulation {
    pub fn from_map(map: &TileMap) -> Self {
        Self {
            density_map: PopulationDensityMap::with_dimensions(&map.bounds(), 0),
            residential: 0,
            residential_cap: false,
            residential_valve: 0,
            commercial: 0,
            commercial_cap: false,
            commercial_valve: 0,
            industrial: 0,
            industrial_cap: false,
            industrial_valve: 0,
            total: 0,
            total_delta: 0,
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
    pub fn get_residential_valve(&self) -> i16 {
        self.commercial_valve
    }
    pub fn is_residential_capped(&self) -> bool {
        self.residential_cap
    }

    pub fn get_commercial(&self) -> u16 {
        self.commercial
    }
    pub fn get_commercial_valve(&self) -> i16 {
        self.commercial_valve
    }
    pub fn is_commercial_capped(&self) -> bool {
        self.commercial_cap
    }

    pub fn get_industrial(&self) -> u16 {
        self.industrial
    }
    pub fn get_industrial_valve(&self) -> i16 {
        self.industrial_valve
    }
    pub fn is_industrial_capped(&self) -> bool {
        self.industrial_cap
    }

    pub fn total_population(&self) -> i64 {
        self.total
    }

    pub fn delta_population(&self) -> i64 {
        self.total_delta
    }

    /// Update the city total population and classification.
    pub fn update(&mut self) -> CityClass {
        let mut old_total = self.total;
        self.total = self.compute_total_population();
        if old_total == -1 {
            old_total = self.total;
        }

        self.total_delta = self.total - old_total;
        CityClass::from_total_population(self.total)
    }

    fn compute_total_population(&self) -> i64 {
        (self.residential as i64 + (self.commercial as i64 + self.industrial as i64 * 8)) * 20
    }
}
