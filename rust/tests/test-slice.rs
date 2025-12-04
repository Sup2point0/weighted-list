mod utils;
use utils::*;


#[test]
fn sort()
{
    let mut list = el();
    list.sort();
    assert_eq!( list, el() );

    let mut list = wl();
    list.sort();
    assert_eq!( list, wl() );
}
