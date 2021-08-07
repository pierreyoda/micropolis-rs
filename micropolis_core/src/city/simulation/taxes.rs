use crate::city::budget::MoneyValue;

use super::statistics::SimulationStatistics;

// TODO: break out into individual user-configurable parameters
const R_LEVELS: [f64; 3] = [0.7, 0.9, 1.2];
const F_LEVELS: [f64; 3] = [1.4, 1.2, 0.8];

// TODO: break out into individual configurable parameters
pub const TAX_TABLE: [i16; 21] = [
    200, 150, 120, 100, 80, 50, 30, 0, -10, -40, -100, -150, -200, -250, -300, -350, -400, -450,
    -500, -550, -600,
];

pub const EXTERNAL_MARKET_PARAMETERS_TABLE: [f64; 3] = [1.2, 1.1, 0.98];

#[derive(Clone, Debug, PartialEq, Default)]
pub struct SimulationTaxes {
    /// City tax rate.
    pub city_tax: u16,
    /// City tax rate - average.
    pub city_tax_average: u16,
    /// Tax port flag.
    ///
    /// TODO: apparently never used. Remove if true.
    pub tax_flag: bool,
}

impl SimulationTaxes {
    /// Collect city taxes.
    ///
    /// @bug Function seems to be doing different things depending on
    ///      Micropolis::totalPop value. With an non-empty city it does fund
    ///      calculations. For an empty city, it immediately sets effects of
    ///      funding, which seems inconsistent at least, and may be wrong
    ///
    /// @bug If Micropolis::taxFlag is set, no variable is touched which seems
    ///      non-robust at least
    pub fn collect_taxes(&mut self, cash_flow: &mut MoneyValue, statistics: &SimulationStatistics) {
        *cash_flow = 0;

        // TODO: apparently tax_flag is never set to true so this always run
    }
}
