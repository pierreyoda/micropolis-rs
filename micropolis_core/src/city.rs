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
pub mod zoning;

use budget::MoneyValue;
use population::CityPopulation;
use power::CityPower;
use simulation::Simulation;
use sprite::ActiveSpritesList;

use crate::{
    game::{GameLevelDifficulty, GameScenario},
    map::{animations::TileMapAnimator, Map, MapRectangle, TileMap, TileType},
    utils::random::MicropolisRandom,
};

use self::{disasters::CityDisasters, traffic::CityTraffic};

pub enum CityInitializationState {
    Initialized = 0,
    JustCreated = 1,
    JustLoaded = 2,
}

/// A Micropolis city.
pub struct City {
    /// Stateful PseudoRandom Number Generator.
    rng: MicropolisRandom,
    /// Active sprites register.
    sprites: ActiveSpritesList,
    /// Status of the city's initialization (`initSimLoad` in the C++ code).
    init_status: CityInitializationState,
    /// Game difficulty.
    difficulty: GameLevelDifficulty,
    /// Game scenario.
    scenario: GameScenario,
    /// Current simulation speed. From 0 to 3.
    simulation_speed: u8,
    /// TileMap describing the city and its surroundings.
    map: TileMap,
    /// TileMap animator.
    map_animator: TileMapAnimator,
    /// Name of the city.
    name: String,
    /// Starting year of the city.
    starting_year: i16,
    /// Cash flow of the city.
    cash_flow: MoneyValue,
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
    /// Disasters simulation,
    disasters: CityDisasters,
    /// Population counts.
    population: CityPopulation,
    /// Electricity simulation.
    power: CityPower,
    /// Traffic simulation.
    traffic: CityTraffic,
    /// Global simulation.
    sim: Simulation,
}

impl City {
    pub fn new(name: String, scenario: GameScenario) -> Result<Self, String> {
        let map = Map::tilemap_with_dimensions(&MapRectangle::new(120, 100), TileType::Dirt)?; // TODO: loading
        let population = CityPopulation::from_map(&map);
        let power = CityPower::from_map(&map);
        let traffic = CityTraffic::from_map(&map);
        let sim = Simulation::new(&map);
        Ok(City {
            rng: MicropolisRandom::from_random_system_seed(),
            sprites: ActiveSpritesList::new(),
            init_status: CityInitializationState::JustCreated,
            difficulty: GameLevelDifficulty::Normal,
            scenario: GameScenario::None,
            simulation_speed: 0,
            map,
            map_animator: TileMapAnimator::load()?,
            name,
            starting_year: 1900,
            cash_flow: 0,
            city_time: 0,
            roads_total: 0,
            rail_total: 0,
            fires_count: 0,
            disasters: CityDisasters::new(&scenario),
            population,
            power,
            traffic,
            sim,
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

    pub fn get_simulation_speed(&self) -> u8 {
        self.simulation_speed
    }

    /// See Micropolis::setSpeed in `utilities.cpp`.
    pub fn set_simulation_speed(&mut self, speed: u8) {
        self.simulation_speed = if speed > 3 { 3 } else { speed }
        // TODO: pause handling
    }

    pub fn total_funds(&self) -> MoneyValue {
        todo!()
    }

    pub fn evaluate(&self) -> Result<(), String> {
        todo!()
    }
}
