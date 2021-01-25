#[derive(Clone, Debug, PartialEq, Default)]
pub struct SimulationStatistics {
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
    /// Average pollution.
    ///
    /// Affected - effectively - by traffic, fire, radioactivity, industrial zones,
    /// seports, airports and power plants.
    pub average_pollution: u16,
    /// Average land value.
    ///
    /// Affected by distance from city center, development dencity, pollution and crime.
    pub average_land_value: u16,
}
