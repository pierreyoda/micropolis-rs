use std::collections::HashMap;

use crate::utils::{random_in_range, Percentage};

use super::CityPopulation;

/// Number of problems to complain abount.
const COMPLAINTS_COUNT: usize = 4;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum CityVotingProblem {
    None,
    Crime,
    Pollution,
    Housing,
    Taxes,
    Traffic,
    Unemployment,
    Fire,
}

use rand::Rng;
use CityVotingProblem::*;

pub const CITY_VOTING_PROBLEMS: [CityVotingProblem; 7] = [
    Crime,
    Pollution,
    Housing,
    Taxes,
    Traffic,
    Unemployment,
    Fire,
];

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

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CityScore {
    current: u16,
    delta: u16,
}

pub struct CityEvaluator {
    /// Current class of the city, determined by population.
    class: CityClass,
    /// Number of votes for each problem.
    problems_votes: HashMap<CityVotingProblem, u32>,
    /// Order of taken problems, in decreasing order of priority.
    problems_order: [CityVotingProblem; COMPLAINTS_COUNT],
    /// Percentage of people who think the mayor is doing a good job.
    mayor_approval_rate: Percentage,
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
        }
    }

    pub fn perform_evaluation(&self) {
        todo!()
    }

    pub fn get_accessed_value() -> u32 {
        todo!()
    }

    pub fn compute_score(&self) -> CityScore {
        todo!()
    }

    pub fn survey_mayor_approval<R: Rng>(&mut self, rng: &mut R, score: CityScore) {
        let city_score = score.current as i32;
        self.mayor_approval_rate = Percentage::from_integer(
            (0..100)
                .filter(|_| random_in_range(rng, 0, 1000) < city_score)
                .count() as u8,
        )
        .unwrap()
    }

    pub fn compute_city_population(&mut self) {
        todo!()
    }

    pub fn classify_city(population: &CityPopulation) -> CityClass {
        match population.total {
            n if n <= 2000 => CityClass::Village,
            n if n <= 10000 => CityClass::Town,
            n if n <= 50000 => CityClass::City,
            n if n <= 100000 => CityClass::Capital,
            n if n <= 500000 => CityClass::Metropolis,
            _ => CityClass::Megalopolis,
        }
    }

    pub fn prioritize_problems(&mut self) {
        todo!()
    }

    pub fn vote_on_problems(&mut self) {
        todo!()
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

    fn compute_traffic_average(&self) -> u16 {
        todo!()
    }

    fn compute_unemployment(&self) -> u16 {
        todo!()
    }

    fn get_fire_severity(&self) -> u16 {
        todo!()
    }
}
