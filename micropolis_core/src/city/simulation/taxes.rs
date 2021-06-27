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
