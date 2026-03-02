use std::*;

use num_traits as nums;


/// Any numerical type, such as `u32`, `usize`, `f64`. The type `W` of item weights in a [`WeightedList<V,W>`](crate::WeightedList) must implement this trait.
pub trait Weight:
      Copy
    + nums::NumAssign
    + nums::NumCast
    + PartialOrd
    + iter::Sum
    + fmt::Debug
{}

/// Auto implementation for all types that fulfil the trait’s requirements.
impl<Type> Weight for Type where Type:
      Copy
    + nums::NumAssign
    + nums::NumCast
    + PartialOrd
    + iter::Sum
    + fmt::Debug
{}
