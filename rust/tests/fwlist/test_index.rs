use crate::*;
use weighted_list::*;


#[test] fn index()
{
    let list = fwl();

    assert_eq!( list[0].value(), "sup" );
    assert_eq!( list[1].value(), "sup" );
    assert_eq!( list[2].value(), "nova" );
    assert_eq!( list[3].value(), "nova" );
    assert_eq!( list[4].value(), "nova" );
    assert_eq!( list[5].value(), "shard" );
    assert_eq!( list[6].value(), "shard" );
    assert_eq!( list[7].value(), "shard" );
    assert_eq!( list[8].value(), "shard" );
    assert_eq!( list[9].value(), "shard" );
}

#[test] fn iter_methods()
{
    let list = wl();

    for _ in list.iter() {}
}

#[test] fn iter_sugar()
{
    let list = fwl();

    for _ in &list {}

    for _ in list {}
}
