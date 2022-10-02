use std::cmp;
use std::collections::HashMap;

use crate::{
    map::{Map, WORLD_HEIGHT, WORLD_WIDTH},
    utils::{random::MicropolisRandom, Percentage},
};

use super::{
    power::CityPower, simulation::statistics::SimulationStatistics, traffic::CityTraffic,
    CityPopulation,
};

/// Number of problems to complain abount.
const COMPLAINTS_COUNT: usize = 4;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum CityVotingProblem {
    None,
    Crime,
    Pollution,
    HousingCosts,
    Taxes,
    Traffic,
    Unemployment,
    Fires,
}

use CityVotingProblem::*;

pub const CITY_VOTING_PROBLEMS: [CityVotingProblem; 7] = [
    Crime,
    Pollution,
    HousingCosts,
    Taxes,
    Traffic,
    Unemployment,
    Fires,
];

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CityScore {
    current: u16,
    delta: u16,
}

pub struct CityEvaluator {
    /// City score.
    ///
    /// Affected by average of problems, residential cap, commercial cap,
    /// industrial cap, road effect, police effect, fire effect,
    /// residential valve, commercial valve, industrial valve, city
    /// population, delta city population, fires, tax rate, and un-powered
    /// zones.
    score: CityScore,
    /// City assessed value.
    assessed_value: u32,
    /// Average traffic.
    traffic_average: u16,
    /// Number of votes for each problem.
    problems_votes: HashMap<CityVotingProblem, u32>,
    /// Order of taken problems, in decreasing order of priority.
    problems_order: [CityVotingProblem; COMPLAINTS_COUNT],
    /// Percentage of people who think the mayor is doing a good job.
    mayor_approval_rate: Percentage,
    /// Should the evaluation window be shown to the user?
    evaluation_updated: bool,
}

impl CityEvaluator {
    pub fn new() -> Self {
        let mut problems_votes = HashMap::new();
        for problem in &CITY_VOTING_PROBLEMS {
            problems_votes.insert(problem.clone(), 0);
        }
        Self {
            class: CityClass::Village,
            problems_votes,
            problems_order: [None, None, None, None],
            mayor_approval_rate: Percentage::from_integer(0).unwrap(),
            evaluation_updated: false,
        }
    }

    /// Initialize evaluation variables.
    fn init(&mut self) {
        self.mayor_approval_rate = Percentage::from_integer(0).unwrap();
        self.score = CityScore {
            current: 500,
            delta: 0,
        };
    }

    /// Evaluate the city.
    pub fn perform_evaluation(
        &mut self,
        rng: &mut MicropolisRandom,
        population: &mut CityPopulation,
        statistics: &SimulationStatistics,
        power: &CityPower,
    ) {
        if population.total_population() > 0 {
            let problems_table = HashMap::new();

            self.assessed_value = Self::compute_accessed_value(power, statistics);
            population.update();
            self.prioritize_problems(&problems_table);
            self.score = self.compute_score();
            self.vote_on_problems(rng);
            self.change_evaluation();
        } else {
            self.init();
            self.mayor_approval_rate = Percentage::from_integer(0).unwrap(); // arbitrary number when no population
            self.change_evaluation();
        }
    }

    fn change_evaluation(&mut self) {}

    /// Access the value of the city.
    pub fn compute_accessed_value(power: &CityPower, statistics: &SimulationStatistics) -> u32 {
        let mut z = statistics.road_total as u32 * 5;
        z += (statistics.rail_total as u32) * 10;
        z += (statistics.police_station_count as u32) * 1000;
        z += (statistics.fire_station_count as u32) * 1000;
        z += (statistics.hospital_count as u32) * 400;
        z += (statistics.stadium_count as u32) * 3000;
        z += (statistics.seaport_count as u32) * 5000;
        z += (statistics.airport_count as u32) * 10000;
        z += (power.coal_generators_count as u32) * 3000;
        z += (power.nuclear_generators_count as u32) * 6000;

        z * 1000
    }

    /// Compute the total score of the city.
    pub fn compute_score(&self) -> CityScore {
        todo!()
    }

    pub fn survey_mayor_approval(&mut self, rng: &mut MicropolisRandom, score: CityScore) {
        let city_score = score.current as i32;
        self.mayor_approval_rate = Percentage::from_integer(
            (0..100)
                .filter(|_| (rng.get_random(1000) as i32) < city_score)
                .count() as u8,
        )
        .unwrap()
    }

    /// Evaluate problems of the city, take votes, and decide which are the most
    /// important ones.
    pub fn prioritize_problems(&mut self) {
        todo!()
    }

    /// Assess how well the mayor is doing.
    pub fn vote_on_problems(&mut self, rng: &mut MicropolisRandom) {
        for z in (0..100) {
            if (rng.get_random(1000) as u16) < self.score.current {
                self.mayor_approval_rate.increment();
            }
        }
    }

    pub fn count_problems(&self) -> usize {
        self.problems_order
            .iter()
            .filter(|problem| *problem != &None)
            .count()
    }

    pub fn get_problem_from_priority(&self, priority_index: usize) -> Option<&CityVotingProblem> {
        if priority_index >= COMPLAINTS_COUNT {
            self.problems_order.get(priority_index)
        } else {
            Option::None
        }
    }

    /// Get the number of votes to solve the worst (by priority index) problem.
    pub fn get_problem_votes(&self, priority_index: usize) -> Option<u32> {
        self.problems_votes
            .get(self.get_problem_from_priority(priority_index)?)
            .cloned()
    }

    /// Compute the average traffic in the city.
    fn compute_traffic_average(&self, land_value_map: &mut Map<u8>, traffic: &CityTraffic) -> u16 {
        let mut traffic_total = 0;
        let mut count = 1;

        let traffic_density_map = traffic.get_density_map();
        let land_value_map_blocksize = land_value_map.get_clustering_strategy().block_size();
        for x in (0..WORLD_WIDTH).step_by(land_value_map_blocksize) {
            for y in (0..WORLD_HEIGHT).step_by(land_value_map_blocksize) {
                let position = (x, y).into();
                if let Some(land_value) = land_value_map.get_tile_at(&position) {
                    if *land_value > 0 {
                        traffic_total += traffic_density_map.get_tile_at(&position).unwrap_or(&0);
                        count += 1;
                    }
                }
            }
        }

        (((traffic_total / count) as f64) * 2.4).floor() as u16
    }

    /// Compute the severity of unemployment in the city.
    fn compute_unemployment(&self, population: &CityPopulation) -> u16 {
        let b = (population.get_commercial() + population.get_industrial()) * 8;
        if b == 0 {
            return 0;
        }
        // Total people / working ratio. At least 1.
        let r = population.get_residential() as f64 / b as f64;

        let unemployment = ((r - 1f64) * 255f64).floor() as u16;
        cmp::min(unemployment, 255)
    }

    /// Compute the severity of fire in the city.
    fn get_fire_severity(&self, statistics: &SimulationStatistics) -> u16 {
        cmp::min(statistics.fire_station_count * 5, 255)
    }
}
