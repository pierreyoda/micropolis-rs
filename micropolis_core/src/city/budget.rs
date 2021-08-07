use std::convert::TryInto;

use crate::utils::Percentage;

/// Integer-based money storage type. Must be copy by default.
///
/// Corresponds to the `Quad` typedef in the C++ code, implemented as a `long`.
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

    pub fn update_budget_lines(&mut self, tax_funds: MoneyValue, total_funds: MoneyValue) {
        let [budget_roads, budget_fire, budget_police] = [
            (self.roads.value as f64 * self.roads.percentage.value()) as MoneyValue,
            (self.fire_department.value as f64 * self.fire_department.percentage.value())
                as MoneyValue,
            (self.police_department.value as f64 * self.police_department.percentage.value())
                as MoneyValue,
        ];
        let mut total = budget_roads + budget_fire + budget_police;
        let mut available_budget = tax_funds + total_funds;

        if available_budget > total {
            // TODO: why are we not substracting from the available budget as below
            self.fire_department.value = budget_fire;
            self.police_department.value = budget_police;
            self.roads.value = budget_roads;
        } else if total > 0 {
            // Not enough available budget to fund everything.
            // First spend on roads, then on fire, then on police.

            if available_budget > budget_roads {
                // Enough budget to fully fund roads.
                self.roads.value = budget_roads;
                available_budget -= budget_roads;

                if available_budget > budget_fire {
                    // Enough budget to fully fund fire.
                    self.fire_department.value = budget_fire;
                    available_budget -= budget_fire;

                    if available_budget > budget_police {
                        // Enough budget to fully fund police.
                        // FIXME: Hey what are we doing here? Should never get here.
                        // We tested for available_budget > total above
                        // (where total = fireInt + policeInt + roadInt),
                        // so this should never happen.
                        self.police_department.value = budget_police;
                        available_budget -= budget_police;
                    } else {
                        // Fuly funded roads and fire.
                        // Partially fund police.
                        self.police_department.value = available_budget;
                        if available_budget > 0 {
                            // Scale back police percentage to available cash.
                            self.police_department.percentage = Percentage::from_integer(
                                (available_budget / self.police_department.value)
                                    .try_into()
                                    .unwrap(),
                            )
                            .unwrap();
                        } else {
                            // Exactly nothing left, so scale back police percentage to zero.
                            self.police_department.percentage =
                                Percentage::from_integer(0).unwrap();
                        }
                    }
                } else {
                    // Not enough budget to fully fund fire.
                    self.fire_department.value = available_budget;

                    // No police after funding roads and fire.
                    self.police_department.value = 0;
                    self.police_department.percentage = Percentage::from_integer(0).unwrap();

                    if available_budget > 0 {
                        // Scale back fire percentage to available cash.
                        self.fire_department.percentage = Percentage::from_integer(
                            (available_budget / self.fire_department.value)
                                .try_into()
                                .unwrap(),
                        )
                        .unwrap();
                    } else {
                        // Exactly nothing left, so scale back fire percentage to zero.
                        self.fire_department.percentage = Percentage::from_integer(0).unwrap();
                    }
                }
            } else {
                assert!(available_budget == total);
                assert!(total == 0);
                // Zero funding, so no values but full percentages.
                self.roads = BudgetLine::new();
                self.fire_department = BudgetLine::new();
                self.police_department = BudgetLine::new();
            }
        }
    }
}
