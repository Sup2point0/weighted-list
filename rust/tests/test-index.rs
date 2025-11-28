mod utils;
use utils::*;

// use weighted_list::*;


#[test]
fn index()
{
    let list = wl();

    assert_eq!( list[0].value, "sup" );
    assert_eq!( list[1].value, "sup" );
    assert_eq!( list[2].value, "nova" );
    assert_eq!( list[3].value, "nova" );
    assert_eq!( list[4].value, "nova" );
    assert_eq!( list[5].value, "shard" );
    assert_eq!( list[6].value, "shard" );
    assert_eq!( list[7].value, "shard" );
    assert_eq!( list[8].value, "shard" );
    assert_eq!( list[9].value, "shard" );
}
