pub mod budget;
pub mod evaluate;
pub mod meta;
pub mod power;
pub mod reports;
pub mod simulation;

use power::CityPower;
use rand::rngs::OsRng;

use crate::map::{Map, MapRectangle, TileMap, TileType};

pub enum CityInitializationState {
    Initialized = 0,
    JustCreated = 1,
    JustLoaded = 2,
}

pub struct CityPopulation {
    /// Number of people in residential zones.
    ///
    /// Depends on the level of zone development.
    residential: u32,
    /// Number of people in commercial zones.
    ///
    /// Depends on the level of zone development.
    commercial: u32,
    /// Number of people in industrial zones.
    ///
    /// Depends on the level of zone development.
    industrial: u32,
    /// Total population.
    ///
    /// Formula = (residential population) / 8 + (commercial population) + (industrial population).
    total: u32,
}

/// A Micropolis city.
pub struct City {
    rng: OsRng,
    /// Status of the city's initialization (`initSimLoad` in the C++ code).
    init_status: CityInitializationState,
    /// TileMap describing the city and its surroundings.
    map: TileMap,
    /// Name of the city.
    name: String,
    /// Starting year of the city.
    starting_year: i16,
    /// City time counter, incremented once every 16 runs through the simulator
    /// (at fast speed).
    ///
    /// A time unit is 7.6 days. 4 units per month, 48 units per year, relative
    /// to `starting_year`.
    city_time: u32,
    /// Number of road tiles in the game.
    ///
    /// Bridges count as 4 tiles, and high density traffic counts as 2.
    roads_total: u32,
    /// Number of rail tiles in the game.
    ///
    /// No penalty for bridges or high traffic density.
    rail_total: u32,
    /// Number of burning fires.
    fires_count: u32,
    /// Population counts.
    population: CityPopulation,
    /// Electricity simulation.
    power: CityPower,
}

impl City {
    pub fn new(name: String) -> Result<Self, String> {
        let map = Map::tilemap_with_dimensions(&MapRectangle::new(120, 100), TileType::Dirt)?; // TODO: loading
        let power = CityPower::from_map(&map);
        Ok(City {
            rng: OsRng,
            init_status: CityInitializationState::JustCreated,
            map,
            name,
            starting_year: 1900,
            city_time: 0,
            roads_total: 0,
            rail_total: 0,
            fires_count: 0,
            population: CityPopulation {
                residential: 0,
                commercial: 0,
                industrial: 0,
                total: 0,
            },
            power,
        })
    }
}
