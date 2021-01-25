use std::sync::RwLockReadGuard;

use crate::utils::Percentage;

pub const MAX_ROAD_EFFECT: u64 = 32;
pub const MAX_FIRE_EFFECT: u64 = 1000;
pub const MAX_POLICE_EFFECT: u64 = 1000;

#[derive(Clone, Debug, PartialEq)]
pub struct SimulationParameters {
    /// Used to normalize residential population.
    residential_population_denominator: u32,
    /// Combination of the population birthrate (positive) and deathrate (positive).
    ///
    /// Always positive.
    birth_rate: Percentage,
    /// Maximum `labor_base` value.
    max_labor_base: Percentage,
    /// Minimum `projected_industrial_population` value.
    min_projected_industrial_population: Percentage,
    /// Default residential population ratio.
    default_residential_ratio: Percentage,
    /// Maximum residential population ratio.
    max_residential_ratio: Percentage,
    /// Maximum commercial population ratio.
    max_commercial_ratio: Percentage,
    /// Maximum industrial population ratio.
    max_industrial_ratio: Percentage,
    /// Maximum global tax rate.
    max_tax_rate: Percentage,
    /// Tax table scale.
    tax_table_scale: Percentage,
    /// Ratio of road spendings over funding, times `MAX_ROAD_EFFECT`.
    road_effect: u64,
    /// Ratio of fire spendings over funding, times `MAX_POLICE_EFFECT`.
    fire_effect: u64,
    /// Ratio of police spendings over funding, times `MAX_POLICE_EFFECT`.
    police_effect: u64,
}

impl Default for SimulationParameters {
    fn default() -> Self {
        Self {
            residential_population_denominator: 8,
            birth_rate: 0.02.into(),
            max_labor_base: 1.3.into(),
            min_projected_industrial_population: 5.0.into(),
            default_residential_ratio: 1.3.into(),
            max_residential_ratio: 2.0.into(),
            max_commercial_ratio: 2.0.into(),
            max_industrial_ratio: 2.0.into(),
            max_tax_rate: 20.0.into(),
            tax_table_scale: 600.0.into(),
            road_effect: MAX_ROAD_EFFECT,
            fire_effect: MAX_FIRE_EFFECT,
            police_effect: MAX_POLICE_EFFECT,
        }
    }
}

impl SimulationParameters {
    pub fn get_road_effect(&self) -> u64 {
        self.road_effect
    }

    pub fn get_fire_effect(&self) -> u64 {
        self.fire_effect
    }

    pub fn get_police_effect(&self) -> u64 {
        self.police_effect
    }
}

#[cfg(test)]
mod tests {
    use super::SimulationParameters;

    #[test]
    fn test_default_parameters() {
        let parameters = SimulationParameters::default();
        assert_eq!(parameters.residential_population_denominator, 8);
        assert_eq!(parameters.birth_rate.value(), 0.02);
        assert_eq!(parameters.max_labor_base.value(), 1.3);
        assert_eq!(parameters.min_projected_industrial_population.value(), 5.0);
        assert_eq!(parameters.default_residential_ratio.value(), 1.3);
        assert_eq!(parameters.max_residential_ratio.value(), 2.0);
        assert_eq!(parameters.max_commercial_ratio.value(), 2.0);
        assert_eq!(parameters.max_industrial_ratio.value(), 2.0);
        assert_eq!(parameters.max_tax_rate.value(), 20.0);
        assert_eq!(parameters.tax_table_scale.value(), 600.0);
    }

    #[test]
    fn test_effect_parameters() {
        let parameters = SimulationParameters::default();
        assert_eq!(parameters.get_road_effect(), 32);
        assert_eq!(parameters.get_fire_effect(), 1000);
        assert_eq!(parameters.get_police_effect(), 1000);
    }
}
