pub mod budget;
pub mod disasters;
pub mod evaluate;
pub mod meta;
pub mod population;
pub mod power;
pub mod reports;
pub mod simulation;
pub mod sprite;
pub mod traffic;

use budget::MoneyValue;
use population::CityPopulation;
use power::CityPower;
use rand::rngs::OsRng;
use simulation::Simulation;

use crate::map::{animations::TileMapAnimator, Map, MapRectangle, TileMap, TileType};

pub enum CityInitializationState {
    Initialized = 0,
    JustCreated = 1,
    JustLoaded = 2,
}

/// A Micropolis city.
pub struct City {
    rng: OsRng,
    /// Status of the city's initialization (`initSimLoad` in the C++ code).
    init_status: CityInitializationState,
    /// TileMap describing the city and its surroundings.
    map: TileMap,
    /// TileMap animator.
    map_animator: TileMapAnimator,
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
    /// Global simulation.
    sim: Simulation,
    /// Electricity simulation.
    power: CityPower,
}

impl City {
    pub fn new(name: String) -> Result<Self, String> {
        let map = Map::tilemap_with_dimensions(&MapRectangle::new(120, 100), TileType::Dirt)?; // TODO: loading
        let population = CityPopulation::from_map(&map);
        let power = CityPower::from_map(&map);
        Ok(City {
            rng: OsRng,
            init_status: CityInitializationState::JustCreated,
            map,
            map_animator: TileMapAnimator::load()?,
            name,
            starting_year: 1900,
            city_time: 0,
            roads_total: 0,
            rail_total: 0,
            fires_count: 0,
            population,
            sim: Simulation::new(),
            power,
        })
    }

    pub fn get_map(&self) -> &TileMap {
        &self.map
    }
    pub fn get_map_mut(&mut self) -> &mut TileMap {
        &mut self.map
    }

    pub fn get_sim(&self) -> &Simulation {
        &self.sim
    }
    pub fn get_sim_mut(&mut self) -> &mut Simulation {
        &mut self.sim
    }

    pub fn invalidate_map(&mut self) {
        self.sim.on_map_updated();
    }

    pub fn total_funds(&self) -> MoneyValue {
        todo!()
    }
}
