mod utils;
use utils::*;

// use weighted_list::*;


#[test]
fn equality()
{
    assert!( el() == el() );
    assert!( wl() == wl() );

    assert!( el() != wl() );
    assert!( wl() != el() );
}
