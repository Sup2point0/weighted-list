// use weighted_list::WeightedList;
mod utils;
use utils::*;


#[test]
fn equality()
{
    assert!( el() == el() );
    assert!( wl() == wl() );

    assert!( el() != wl() );
    assert!( wl() != el() );
}
