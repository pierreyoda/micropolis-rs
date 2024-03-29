use std::collections::HashMap;
use std::{cmp, thread::current};

use crate::{
    city::simulation::parameters::{MAX_FIRE_EFFECT, MAX_ROAD_EFFECT},
    map::{Map, WORLD_HEIGHT, WORLD_WIDTH},
    utils::{clamp, random::MicropolisRandom, Percentage},
};

use super::{
    population::CityClass,
    power::CityPower,
    simulation::{
        parameters::{SimulationParameters, MAX_POLICE_EFFECT},
        statistics::SimulationStatistics,
        taxes::SimulationTaxes,
    },
    traffic::CityTraffic,
    CityPopulation,
};

/// Number of problems to complain about.
const COMPLAINTS_COUNT: u8 = 4;

const PROBLEMS_COUNT: u8 = 7;
const PROBLEMS_NUMBER: u8 = 10;

/// Problems in the city where citizens vote on.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum CityVotingProblem {
    Crime = 0,
    Pollution = 1,
    HousingCosts = 2,
    Taxes = 3,
    Traffic = 4,
    Unemployment = 5,
    Fires = 6,
    None,
}

impl CityVotingProblem {
    pub fn from_u8(value: u8) -> Option<CityVotingProblem> {
        match value {
            0 => Some(Self::Crime),
            1 => Some(Self::Pollution),
            2 => Some(Self::HousingCosts),
            3 => Some(Self::Taxes),
            4 => Some(Self::Traffic),
            5 => Some(Self::Unemployment),
            6 => Some(Self::Fires),
            _ => Option::None,
        }
    }

    pub fn to_u8(&self) -> u8 {
        match self {
            Self::Crime => 0,
            Self::Pollution => 1,
            Self::HousingCosts => 2,
            Self::Taxes => 3,
            Self::Traffic => 4,
            Self::Unemployment => 5,
            Self::Fires => 6,
            _ => unreachable!(),
        }
    }
}

use CityVotingProblem::*;

pub const CITY_VOTING_PROBLEMS: [CityVotingProblem; PROBLEMS_COUNT as usize] = [
    Crime,
    Pollution,
    HousingCosts,
    Taxes,
    Traffic,
    Unemployment,
    Fires,
];

/// City score.
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct CityScore {
    /// Affected by average of problems, residential cap, commercial cap,
    /// industrial cap, road effect, police effect, fire effect,
    /// residential valve, commercial valve, industrial valve, city
    /// population, delta city population, fires, tax rate, and unpowered
    /// zones.
    current: u16,
    /// Change in city score.
    delta: u16,
}

/// City population for scoring.
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct CityScoringPopulation {
    /// Depends on residential, commercial and industrial populations.
    current: u16,
    /// Change in city population.
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
    /// City scoring population.
    population: CityScoringPopulation,
    /// City assessed value.
    ///
    ///
    // Depends on total number of roads, rails, police stations,
    // fire stations, hospitals, stadiums, seaports, airports,
    // coal and nuclear plants.
    assessed_value: u32,
    /// City class, affected by the city population.
    class: CityClass,
    /// Average traffic.
    ///
    /// Depends on average traffic density of tiles with non-zero land value.
    traffic_average: u16,
    /// Number of votes for each problem.
    problems_votes: HashMap<u8, u16>,
    /// Order of taken problems, in decreasing order of priority.
    problems_order: [CityVotingProblem; COMPLAINTS_COUNT as usize],
    /// Percentage of people who think the mayor is doing a good job.
    mayor_approval_rate: Percentage,
    /// Should the evaluation window be shown to the user?
    evaluation_updated: bool,
}

impl CityEvaluator {
    pub fn new() -> Self {
        Self {
            score: CityScore::default(),
            population: CityScoringPopulation::default(),
            assessed_value: 0,
            class: CityClass::Village,
            traffic_average: 0,
            problems_votes: HashMap::new(),
            problems_order: [None, None, None, None],
            mayor_approval_rate: Percentage::from_integer(0).unwrap(),
            evaluation_updated: false,
        }
    }

    /// Initialize evaluation variables.
    fn init(&mut self) {
        self.score = CityScore {
            current: 500,
            delta: 0,
        };
        self.population = CityScoringPopulation {
            current: 0,
            delta: 0,
        };
        self.assessed_value = 0;
        self.class = CityClass::Village;

        let mut problems_votes = HashMap::with_capacity(CITY_VOTING_PROBLEMS.len());
        for problem in &CITY_VOTING_PROBLEMS {
            problems_votes.insert(problem.to_u8(), 0);
        }
        self.problems_votes = problems_votes;

        for i in 0..COMPLAINTS_COUNT {
            self.problems_order[i as usize] = CityVotingProblem::None; // TODO: check if same behavior
        }

        self.mayor_approval_rate = Percentage::from_integer(0).unwrap();
    }

    /// Evaluate the city.
    pub fn perform_evaluation(
        &mut self,
        rng: &mut MicropolisRandom,
        land_value_map: &Map<u8>,
        parameters: &SimulationParameters,
        population: &mut CityPopulation,
        statistics: &SimulationStatistics,
        taxes: &SimulationTaxes,
        power: &CityPower,
        traffic: &CityTraffic,
    ) {
        if population.total_population() > 0 {
            let mut problems_table = HashMap::with_capacity(PROBLEMS_NUMBER as usize);
            for z in 0..PROBLEMS_NUMBER {
                problems_table.insert(z, 0u16);
            }

            self.assessed_value = Self::compute_accessed_value(power, statistics);
            self.class = population.update();
            self.prioritize_problems(
                rng,
                &mut problems_table,
                land_value_map,
                population,
                statistics,
                taxes,
                traffic,
            );
            self.score = self.compute_score(
                population,
                &problems_table,
                parameters,
                statistics,
                taxes,
                power,
            );
            self.vote_on_mayor(rng);
            self.change_evaluation();
        } else {
            self.init();
            self.mayor_approval_rate = Percentage::from_integer(50).unwrap(); // arbitrary number when no population
            self.change_evaluation();
        }
    }

    fn change_evaluation(&mut self) {
        self.evaluation_updated = true;
    }

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
    pub fn compute_score(
        &self,
        population: &CityPopulation,
        problems_table: &HashMap<u8, u16>,
        parameters: &SimulationParameters,
        statistics: &SimulationStatistics,
        taxes: &SimulationTaxes,
        power: &CityPower,
    ) -> CityScore {
        let mut x = 0;
        let last_city_score = self.score.current;

        for z in 0..PROBLEMS_COUNT {
            // add the 7 problems
            x += problems_table
                .get(&z)
                .expect("CityEvaluator.compute_score: problems_table was properly filled.");
        }

        x = x / 3;
        x = cmp::min(x, 256);
        let mut partial_score = clamp((256 - x) * 4, 0, 1000) as i32;

        if population.is_residential_capped() {
            partial_score = (partial_score as f64 * 0.85).floor() as i32;
        }
        if population.is_commercial_capped() {
            partial_score = (partial_score as f64 * 0.85).floor() as i32;
        }
        if population.is_industrial_capped() {
            partial_score = (partial_score as f64 * 0.85).floor() as i32;
        }

        if parameters.get_road_effect() < MAX_ROAD_EFFECT {
            partial_score -= MAX_ROAD_EFFECT as i32 - parameters.get_road_effect() as i32;
        }
        if parameters.get_police_effect() < MAX_POLICE_EFFECT {
            // 10.0001 = 10000.1 / 1000, 1/10.0001 is about 0.1
            partial_score = (partial_score as f64
                * (0.9
                    + (parameters.get_police_effect() as f64
                        / (10.0001 * MAX_POLICE_EFFECT as f64))))
                .floor() as i32;
        }
        if parameters.get_fire_effect() < MAX_FIRE_EFFECT {
            // 10.0001 = 10000.1 / 1000, 1/10.0001 is about 0.1
            partial_score = (partial_score as f64
                * (0.9
                    + (parameters.get_fire_effect() as f64 / (10.0001 * MAX_FIRE_EFFECT as f64))))
                .floor() as i32;
        }

        if population.get_residential_valve() < -1000 {
            partial_score = (partial_score as f64 * 0.85).floor() as i32;
        }
        if population.get_commercial_valve() < -1000 {
            partial_score = (partial_score as f64 * 0.85).floor() as i32;
        }
        if population.get_industrial_valve() < -1000 {
            partial_score = (partial_score as f64 * 0.85).floor() as i32;
        }

        let mut sm = 1.0;
        let (population, delta_population) =
            (population.total_population(), population.delta_population());
        if population == 0 || delta_population == 0 {
            // there is nobody or no migration happened
            sm = 1.0;
        } else if delta_population == population {
            // city sprang into existence or doubled in size
            sm == 1.0;
        } else if delta_population > 0 {
            sm = 1.0 + delta_population as f64 / population as f64;
        } else if delta_population < 0 {
            sm = 0.95 + population as f64 / (population as f64 - delta_population as f64);
        }
        partial_score = ((partial_score as f64) * sm).floor() as i32;

        partial_score -= Self::get_fire_severity(statistics) as i32 - taxes.city_tax as i32; // fires and taxes decrease the score

        let tm = power.get_unpowered_zone_count() + power.get_powered_zone_count(); // decreasing score for unpowered zones
        if tm > 0 {
            partial_score = (partial_score as f64
                * (power.get_powered_zone_count() as f64 / tm as f64))
                .floor() as i32;
        }

        partial_score = clamp(partial_score, 0, 1000);

        let last_city_score = self.score.current;
        let city_score = ((self.score.current as i32 + partial_score) / 2) as u16;
        CityScore {
            current: city_score,
            delta: city_score - last_city_score,
        }
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
    pub fn prioritize_problems(
        &mut self,
        rng: &mut MicropolisRandom,
        problems_table: &mut HashMap<u8, u16>,
        land_value_map: &Map<u8>,
        population: &CityPopulation,
        statistics: &SimulationStatistics,
        taxes: &SimulationTaxes,
        traffic: &CityTraffic,
    ) {
        let mut problems_taken = HashMap::new();
        for z in 0..PROBLEMS_NUMBER {
            problems_taken.insert(z, false);
            problems_table.insert(z, 0);
        }

        problems_table.insert(CityVotingProblem::Crime as u8, statistics.average_crime);
        problems_table.insert(
            CityVotingProblem::Pollution as u8,
            statistics.average_pollution,
        );
        problems_table.insert(
            CityVotingProblem::HousingCosts as u8,
            statistics.average_land_value * 7 / 10,
        );
        problems_table.insert(CityVotingProblem::Taxes as u8, taxes.city_tax * 10);
        problems_table.insert(
            CityVotingProblem::Traffic as u8,
            Self::compute_traffic_average(land_value_map, traffic),
        );
        problems_table.insert(
            CityVotingProblem::Unemployment as u8,
            Self::compute_unemployment(population),
        );
        problems_table.insert(
            CityVotingProblem::Fires as u8,
            Self::get_fire_severity(statistics),
        );
        self.vote_on_problems(rng, problems_table);

        for z in 0..COMPLAINTS_COUNT {
            // Find biggest problem not taken yet
            let (mut max_votes, mut best_problem_index) = (0, PROBLEMS_COUNT);
            for i in 0..PROBLEMS_COUNT {
                let problem = CityVotingProblem::from_u8(i).expect(
                    "CityEvaluator.prioritize_problems: can convert u8 to CityVotingProblem.",
                );
                if let Some(votes) = self.problems_votes.get(&problem.to_u8()) {
                    if *votes > max_votes && !problems_taken.get(&i).expect("CityEvaluator.prioritize_problems: problems_table was properly initialized.") {
                        best_problem_index = i;
                        max_votes = *votes;
                    }
                }
            }

            if let Some(best_problem) = CityVotingProblem::from_u8(best_problem_index) {
                self.problems_order[z as usize] = best_problem;
                problems_taken.insert(best_problem_index, true);
            } else {
                // No problem found: repeating the procedure will give the same result
                // TODO: optimize by filling all remaining entries, and breaking out
                self.problems_order[z as usize] = None;
            }
        }
    }

    /// Vote on the problems of the city.
    fn vote_on_problems(
        &mut self,
        rng: &mut MicropolisRandom,
        problems_table: &mut HashMap<u8, u16>,
    ) {
        for z in 0..PROBLEMS_NUMBER {
            self.problems_votes.insert(z, 0);
        }

        let (mut voting_on_problem_number, mut votes_count, mut loops_counter) = (0, 0, 0);
        while votes_count < 100 && loops_counter < 600 {
            if (rng.get_random(300) as u16)
                < *problems_table.get(&voting_on_problem_number).expect(
                    "CityEvaluator.vote_on_problems: problems_table must be initialized properly.",
                )
            {
                let problem_votes = self
                    .problems_votes
                    .get_mut(&voting_on_problem_number)
                    .unwrap();
                *problem_votes += 1;
                votes_count += 1;
            }

            voting_on_problem_number += 1;
            if voting_on_problem_number > PROBLEMS_NUMBER {
                voting_on_problem_number = 0;
            }

            loops_counter += 1;
        }
    }

    /// Vote on how well the mayor is doing.
    fn vote_on_mayor(&mut self, rng: &mut MicropolisRandom) {
        self.mayor_approval_rate = Percentage::from_integer(0).unwrap();
        for z in 0..100 {
            if (rng.get_random(1000) as u16) < self.score.current {
                self.mayor_approval_rate.increment();
            }
        }
    }

    fn get_problem_from_priority(&self, priority_index: u8) -> Option<&CityVotingProblem> {
        if priority_index >= COMPLAINTS_COUNT {
            self.problems_order.get(priority_index as usize)
        } else {
            Option::None
        }
    }

    /// Get the number of votes to solve the worst (by priority index) problem.
    pub fn get_problem_votes(&self, priority_index: u8) -> Option<u16> {
        self.get_problem_from_priority(priority_index)
            .and_then(|problem| self.problems_votes.get(&problem.to_u8()))
            .cloned()
    }

    /// Compute the average traffic in the city.
    fn compute_traffic_average(land_value_map: &Map<u8>, traffic: &CityTraffic) -> u16 {
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
    fn compute_unemployment(population: &CityPopulation) -> u16 {
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
    fn get_fire_severity(statistics: &SimulationStatistics) -> u16 {
        cmp::min(statistics.fire_station_count * 5, 255)
    }
}
