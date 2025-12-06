use crate::*;
// use weighted_list::*;


#[test] fn sort()
{
    let mut list = el();
    list.sort();
    assert_eq!( list, el() );

    let mut list = wl();
    list.sort();
    assert_eq!( list, wl() );
}
