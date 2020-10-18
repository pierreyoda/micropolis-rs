use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{FromPrimitive as FromPrimitiveTrait, ToPrimitive as ToPrimitiveTrait};

#[derive(Clone, Debug, PartialEq, Eq, FromPrimitive, ToPrimitive)]
pub enum ZoneType {
    Commercial,
    Industrial,
    Residential,
}

impl ZoneType {
    pub fn from_usize(value: usize) -> Option<Self> {
        FromPrimitiveTrait::from_usize(value)
    }

    pub fn to_usize(&self) -> Option<usize> {
        match self {
            _ => ToPrimitiveTrait::to_usize(self),
        }
    }
}
