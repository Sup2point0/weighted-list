use std::*;
use std::{
    error::Error,
    fmt::{ Debug, Display },
};


/// An error returned when failing to cast one numeric type to another, which may be required for certain [`WeightedList`](crate::WeightedList) operations.
#[derive(Debug)]
pub struct NumCastFailure
{
    pub(crate) value: String,
    pub(crate) target: &'static str,
}

impl Display for NumCastFailure
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        write!(f, "Failed to cast {:?} to type {}", self.value, self.target)
    }
}

impl Error for NumCastFailure {}


/// A method requires a non-empty [`WeightedList`](crate::WeightedList) but received an empty one.
#[derive(Debug)]
pub struct EmptyWeightedList
{
    pub(crate) reason: &'static str,
}

impl Display for EmptyWeightedList
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        write!(f, "{}", self.reason)
    }
}

impl Error for EmptyWeightedList {}
