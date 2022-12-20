use num_traits::Num;

pub mod random;

#[derive(Clone, Debug, PartialEq)]
pub struct Percentage(f64);

impl Percentage {
    pub fn from_integer(percentage: u8) -> Option<Self> {
        if percentage <= 100 {
            Some(Percentage(percentage as f64 / 100f64))
        } else {
            None
        }
    }

    /// Value, in percentage (%).
    pub fn value(&self) -> f64 {
        self.0
    }

    /// Increment by 1%.
    pub fn increment(&mut self) {
        self.0 += 1f64;
    }
}

impl From<f64> for Percentage {
    fn from(val: f64) -> Self {
        Percentage(val)
    }
}

/// Clamp a value between [min, max].
pub fn clamp<U: Num + Ord>(value: U, lower: U, upper: U) -> U {
    value.clamp(lower, upper)
}

#[cfg(test)]
mod tests {
    use super::Percentage;

    #[test]
    fn test_percentage_struct() {
        assert_eq!(Percentage::from_integer(0).unwrap().value(), 0f64);
        assert_eq!(Percentage::from_integer(29).unwrap().value(), 0.29f64);
        assert_eq!(Percentage::from_integer(100).unwrap().value(), 1f64);
        assert_eq!(
            (Percentage)(0.79f64.into()),
            Percentage::from_integer(79).unwrap()
        );
        assert_eq!(Percentage::from_integer(103), None);
    }
}
