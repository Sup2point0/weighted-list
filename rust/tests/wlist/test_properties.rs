use crate::*;
use weighted_list::*;


#[test] fn len()
{
    assert_eq!( el().len(), 0 );
    assert_eq!( wl().len(), 10 );

    assert_eq!( elf().len(), 0.0 );
    assert_eq!( wlf().len(), 10.0 );
}

#[test] fn with_capacity()
{
    assert_eq!( WL::with_capacity(0).capacity(), 0 );
    assert_eq!( WL::with_capacity(69).capacity(), 69 );
}

#[test] fn total_items()
{
    assert_eq!( el().total_items(), 0 );
    assert_eq!( wl().total_items(), 3 );
    
    assert_eq!( elf().total_items(), 0 );
    assert_eq!( wlf().total_items(), 3 );
}

#[test] fn is_empty()
{
    assert!( el().is_empty() );
    assert!( !wl().is_empty() );
    
    assert!( elf().is_empty() );
    assert!( !wlf().is_empty() );
}

#[test] fn is_zero()
{
    assert!( el().is_zero() );
    assert!( !wl().is_zero() );
    assert!( wlist![(0, "qi")].is_zero() );
    assert!( !wlist![(0, "qi"), (2, "sup")].is_zero() );

    assert!( elf().is_zero() );
    assert!( !wlf().is_zero() );
    assert!( wlist![(0.0, "qis")].is_zero() );
    assert!( !wlist![(0.0, "qis"), (2.0, "sups")].is_zero() );
}

#[test] fn has_negative_weights()
{
    assert!( !el().has_negative_weights() );
    assert!( !wl().has_negative_weights() );
    assert!( wlist![(-1, "ix")].has_negative_weights() );
    assert!( wlist![(-1, "ix"), (2, "sup")].has_negative_weights() );
}
