use std::*;

use num_traits as nums;


/// Any general numerical type, such as `u32`, `usize`, `f64`. The type `W` of item weights in a [`WeightedList<V,W>`](crate::WeightedList) implement this trait.
pub trait Weight:
    Copy
    + nums::NumAssign
    + nums::NumCast
    + PartialOrd
    + iter::Sum
    + fmt::Debug
{}

impl<Type> Weight for Type where Type:
    Copy
    + nums::NumAssign
    + nums::NumCast
    + PartialOrd
    + iter::Sum
    + fmt::Debug
{}
