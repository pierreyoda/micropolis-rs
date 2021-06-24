use crate::map::MapPosition;

#[derive(Clone, Debug, PartialEq, Default)]
pub struct SimulationStatistics {
    /// Number of road tiles in the game.
    ///
    /// Bridges count as 4 tiles, and high-density traffic counts as 2 tiles.
    pub road_total: u16,
    /// Total number of rails.
    ///
    /// No penalty for bridges or high-traffic density.
    pub rail_total: u16,
    /// Police station population.
    pub police_station_count: u16,
    /// Fire station population.
    pub fire_station_count: u16,
    /// Seaport station population.
    pub seaport_count: u16,
    /// Airport station population.
    pub airport_count: u16,
    /// Stadium population.
    pub stadium_count: u16,
    /// Average crime.
    ///
    /// Affected by land value, population density, police station distance.
    pub average_crime: u16,
    /// Coordinates of the most criminal area. Not used.
    pub maximum_crime_at: MapPosition,
    /// Average pollution.
    ///
    /// Affected - effectively - by traffic, fire, radioactivity, industrial zones,
    /// seports, airports and power plants.
    pub average_pollution: u16,
    /// Coordinates of the most polluted area (for the monster).
    pub maximum_pollution_at: MapPosition,
    /// Average land value.
    ///
    /// Affected by distance from city center, development dencity, pollution and crime.
    pub average_land_value: u16,
}
