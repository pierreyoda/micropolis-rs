use std::cmp::min;

use num_traits::Unsigned;
use rand::{distributions::uniform::SampleUniform, Rng};

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
}

/// Generate a random integer in the given inclusive range.
pub fn random_in_range<U: Unsigned + Ord + SampleUniform, R: Rng>(
    rng: &mut R,
    lower: U,
    upper: U,
) -> U {
    rng.gen_range(lower, upper + U::one())
}

/// Generate a random integer in the given inclusive range with a bias
/// towards smaller values.
pub fn erandom_in_range<U: Unsigned + Ord + SampleUniform, R: Rng>(
    rng: &mut R,
    lower: U,
    upper: U,
) -> U {
    let z = random_in_range(rng, lower, upper);
    let x = random_in_range(rng, lower, upper);
    min(z, x)
}

#[cfg(test)]
mod tests {
    use super::Percentage;

    #[test]
    fn test_percentage_struct() {
        assert_eq!(Percentage::from_integer(0).unwrap().value(), 0f64);
        assert_eq!(Percentage::from_integer(29).unwrap().value(), 0.29f64);
        assert_eq!(Percentage::from_integer(100).unwrap().value(), 1f64);
        assert_eq!(Percentage::from_integer(103), None);
    }
}
