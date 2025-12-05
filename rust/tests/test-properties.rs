mod utils;
use utils::*;

use weighted_list::*;


#[test] fn len()
{
    assert_eq!( el().len(), 0 );
    assert_eq!( wl().len(), 10 );
}

#[test] fn with_capacity()
{
    assert_eq!( WeightedList::<String, i32>::with_capacity(0).capacity(), 0 );
    assert_eq!( WeightedList::<String, i32>::with_capacity(69).capacity(), 69 );
}

#[test] fn total_values()
{
    assert_eq!( el().total_values(), 0 );
    assert_eq!( wl().total_values(), 3 );
}

#[test] fn is_empty()
{
    assert!( el().is_empty() );
    assert!( !wl().is_empty() );
}

#[test] fn is_zero()
{
    assert!( !el().is_zero() );
    assert!( !wl().is_zero() );
    assert!( wlist![(0, "qi")].is_zero() );
    assert!( !wlist![(0, "qi"), (2, "sup")].is_zero() );
}

#[test] fn has_negative_weights()
{
    assert!( !el().has_negative_weights() );
    assert!( !wl().has_negative_weights() );
    assert!( wlist![(-1, "ix")].has_negative_weights() );
    assert!( wlist![(-1, "ix"), (2, "sup")].has_negative_weights() );
}
