use std::cmp::{max, min};

use crate::{
    city::{budget::MoneyValue, population::CityPopulation},
    utils::clamp,
};

use super::statistics::SimulationStatistics;

/// Number of history entries.
const HISTORY_LENGTH: usize = 480;
/// Number of miscellaneous history entries.
const MISC_HISTORY_LENGTH: usize = 240;

const RESIDENTIAL_POPULATION_DENOMINATOR: u16 = 8;

pub struct CitySimulationCensus {
    /// Residential population history.
    residential_history: Vec<u16>,
    /// Commercial population history.
    commercial_history: Vec<u16>,
    /// Industrial population history.
    industrial_history: Vec<u16>,
    /// Money history.
    money_history: Vec<u16>,
    /// Pollution history.
    pollution_history: Vec<u16>,
    /// Crime history.
    crime_history: Vec<u16>,
    /// Memory used to save miscellaneous game values in the save file.
    misc_history: Vec<u16>,
    /// 10 year residential history maximum value.
    residential_history_10_max: u16,
    /// 120 year residential history maximum value.
    residential_history_120_max: u16,
    /// 10 year commercial history maximum value.
    commercial_history_10_max: u16,
    /// 120 year commercial history maximum value.
    commercial_history_120_max: u16,
    /// 10 year industrial history maximum value.
    industrial_history_10_max: u16,
    /// 120 year industrial history maximum value.
    industrial_history_120_max: u16,
    /// Census changed flag.
    ///
    /// If true, need to redraw census-dependent stuff.
    census_changed: bool,
    /// Need hospital?
    ///
    /// 0 if no, 1 if yes, -1 if too many.
    need_hospital: i8,
    /// Need church?
    ///
    /// 0 if no, 1 if yes, -1 if too many.
    need_church: i8,
}

impl CitySimulationCensus {
    pub fn new() -> Self {
        Self {
            residential_history: vec![0; HISTORY_LENGTH],
            commercial_history: vec![0; HISTORY_LENGTH],
            industrial_history: vec![0; HISTORY_LENGTH],
            money_history: vec![0; HISTORY_LENGTH],
            pollution_history: vec![0; HISTORY_LENGTH],
            crime_history: vec![0; HISTORY_LENGTH],
            misc_history: vec![0; MISC_HISTORY_LENGTH],
            residential_history_10_max: 0,
            residential_history_120_max: 0,
            commercial_history_10_max: 0,
            commercial_history_120_max: 0,
            industrial_history_10_max: 0,
            industrial_history_120_max: 0,
            census_changed: false,
            need_hospital: 0,
            need_church: 0,
        }
    }

    /// Take monthly snapshots of all relevant data for the historic graphs.
    ///
    /// Also update variables that control building new churches and hospitals.
    pub fn take_monthly_census(
        &mut self,
        population: &CityPopulation,
        statistics: &mut SimulationStatistics,
        cash_flow: MoneyValue,
    ) {
        self.residential_history_10_max = 0;
        self.commercial_history_10_max = 0;
        self.industrial_history_10_max = 0;

        let mut i = 118;
        while i >= 0 {
            let x = i as usize;

            self.residential_history_10_max =
                max(self.residential_history_10_max, self.residential_history[x]);
            self.commercial_history_10_max =
                max(self.commercial_history_10_max, self.commercial_history[x]);
            self.industrial_history_10_max =
                max(self.industrial_history_10_max, self.industrial_history[x]);

            self.residential_history[x + 1] = self.residential_history[x];
            self.commercial_history[x + 1] = self.commercial_history[x];
            self.industrial_history[x + 1] = self.industrial_history[x];
            self.crime_history[x + 1] = self.crime_history[x];
            self.pollution_history[x + 1] = self.pollution_history[x];
            self.money_history[x + 1] = self.money_history[x];

            i -= 1;
        }

        self.residential_history[0] =
            population.get_residential() / RESIDENTIAL_POPULATION_DENOMINATOR;
        self.commercial_history[0] = population.get_commercial();
        self.industrial_history[0] = population.get_industrial();

        statistics.crime_ramp += (statistics.average_crime - statistics.crime_ramp) / 4;
        self.crime_history[0] = min(statistics.crime_ramp, 255);

        statistics.pollution_ramp += (statistics.average_pollution - statistics.pollution_ramp) / 4;
        self.pollution_history[0] = min(statistics.pollution_ramp, 255);

        let x = ((cash_flow / 20) + 128) as u16;
        self.money_history[0] = clamp(x, 0, 255);

        self.on_census_changed();

        let residential_population_scaled = population.get_residential() >> 8;
        self.need_hospital =
            match statistics.hospital_count as i32 - residential_population_scaled as i32 {
                delta if delta < 0 => 1,
                delta if delta > 0 => -1,
                _ => 0,
            };

        let faithful_population = residential_population_scaled + statistics.faith;
        self.need_church = match statistics.church_count as i32 - faithful_population as i32 {
            delta if delta < 0 => 1,
            delta if delta > 0 => -1,
            _ => 0,
        };
    }

    /// Take yearly census.
    pub fn take_yearly_census(&mut self, population: &CityPopulation) {
        self.residential_history_120_max = 0;
        self.commercial_history_120_max = 0;
        self.industrial_history_120_max = 0;

        let mut x = 238;
        while x >= 120 {
            self.residential_history_120_max = max(
                self.residential_history_120_max,
                self.residential_history[x],
            );
            self.commercial_history_120_max =
                max(self.commercial_history_120_max, self.commercial_history[x]);
            self.industrial_history_120_max =
                max(self.industrial_history_120_max, self.industrial_history[x]);

            self.residential_history[x + 1] = self.residential_history[x];
            self.commercial_history[x + 1] = self.commercial_history[x];
            self.industrial_history[x + 1] = self.industrial_history[x];
            self.crime_history[x + 1] = self.crime_history[x];
            self.pollution_history[x + 1] = self.pollution_history[x];
            self.money_history[x + 1] = self.money_history[x];

            x -= 1;
        }

        self.residential_history[120] =
            population.get_residential() / RESIDENTIAL_POPULATION_DENOMINATOR;
        self.commercial_history[120] = population.get_commercial();
        self.industrial_history[120] = population.get_industrial();
        self.crime_history[120] = self.crime_history[0];
        self.pollution_history[120] = self.pollution_history[0];
        self.money_history[120] = self.money_history[0];

        self.on_census_changed();
    }

    /// Set the flag that graph data has been changed and graphs should be updated.
    fn on_census_changed(&mut self) {
        self.census_changed = true;
    }
}
