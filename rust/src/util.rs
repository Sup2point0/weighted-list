use std::fmt::Debug;

use num_traits;
use num_traits::NumCast;

use crate::*;


pub fn try_cast<T, R>(n: T) -> Result<R, NumCastFailure>
    where
        T: NumCast + Copy + Debug,
        R: NumCast,
{
    num_traits::cast::<T, R>(n)
        .ok_or(
            NumCastFailure {
                value: format!("{n:?}"),
                target: std::any::type_name::<R>(),
            }
        )
}
