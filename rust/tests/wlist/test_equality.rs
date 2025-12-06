use crate::*;

// use weighted_list::*;


#[test] fn equality()
{
    assert_eq!( el(), el() );
    assert_eq!( wl(), wl() );

    assert_ne!( el(), wl() );
    assert_ne!( wl(), el() );
}
