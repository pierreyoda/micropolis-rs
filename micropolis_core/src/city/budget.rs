use crate::utils::Percentage;

/// Integer-based money storage type. Must be copy by default.
///
/// Corresponds to the `Quad` typedef in the C++ code,
/// implemented as a `long`.
pub type MoneyValue = u32;

pub struct BudgetLine {
    /// Absolute amount of money granted for this budget line.
    value: MoneyValue,
    /// Percentage of requested costs relative to the funding level.
    percentage: Percentage,
}

impl BudgetLine {
    pub fn new() -> Self {
        BudgetLine {
            value: 0,
            percentage: Percentage::from_integer(100).unwrap(),
        }
    }
}

pub struct CityBudget {
    fire_department: BudgetLine,
    police_department: BudgetLine,
    roads: BudgetLine,
}

impl CityBudget {
    pub fn new() -> Self {
        CityBudget {
            fire_department: BudgetLine::new(),
            police_department: BudgetLine::new(),
            roads: BudgetLine::new(),
        }
    }

    pub fn update_budget_lines(&mut self, available_budget: MoneyValue) {
        let budget_lines = [&self.fire_department, &self.police_department, &self.roads];
        let budgets_values = budget_lines
            .into_iter()
            .map(|line| line.value as f64 * line.percentage.value());
        let mut total: f64 = budgets_values.sum();
        if total > 0 {

        }
    }
}
