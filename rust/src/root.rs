use std::{
    fmt,
    iter::*,
};

use num_traits as nums;


pub trait Weight:
    nums::NumAssign
    + nums::NumCast
    + Copy
    + PartialOrd
    + Sum
    + fmt::Display
{}

impl<Type> Weight for Type where Type:
    nums::NumAssign
    + nums::NumCast
    + Copy
    + PartialOrd
    + Sum
    + fmt::Display
{}


pub trait UnsignedWeight:
    Weight
    + nums::Unsigned
{}

impl<Type> UnsignedWeight for Type where Type:
    Weight
    + nums::Unsigned
{}
