use crate::*;
use weighted_list::*;


#[test] fn from_frozen_weighted_list()
{
    assert_eq!( WeightedList::from(fwl()), wl() );
    assert_eq!( WeightedList::from_iter(fwl()), wl() );
}
