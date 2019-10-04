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
