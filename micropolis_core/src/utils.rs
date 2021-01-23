use std::cmp::min;

use num_traits::Num;
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

impl Into<Percentage> for f64 {
    fn into(self) -> Percentage {
        Percentage(self)
    }
}

/// Clamp a value between [min, max].
pub fn clamp<U: Num + Ord>(value: U, lower: U, upper: U) -> U {
    if value < lower {
        lower
    } else if value > upper {
        upper
    } else {
        value
    }
}

/// Generate a random integer in the given inclusive range.
pub fn random_in_range<U: Copy + Num + Ord + SampleUniform, R: Rng>(
    rng: &mut R,
    lower: U,
    upper: U,
) -> U {
    rng.gen_range(lower..=upper + U::one())
}

/// Get a random 16 bit integer.
pub fn random_16<R: Rng>(rng: &mut R) -> u16 {
    rng.gen_range(0..=u16::MAX)
}

/// Generate a random integer in the given inclusive range with a bias
/// towards smaller values.
pub fn erandom_in_range<U: Copy + Num + Ord + SampleUniform, R: Rng>(
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
        assert_eq!(
            (Percentage)(0.79f64.into()),
            Percentage::from_integer(79).unwrap()
        );
        assert_eq!(Percentage::from_integer(103), None);
    }
}
