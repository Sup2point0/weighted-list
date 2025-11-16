// use weighted_list::WeightedList;
mod utils;
use utils::*;


#[test]
fn index()
{
    let list = wl();

    assert!( list[0].value == "sup" );
    assert!( list[1].value == "sup" );
    assert!( list[2].value == "nova" );
    assert!( list[3].value == "nova" );
    assert!( list[4].value == "nova" );
    assert!( list[5].value == "shard" );
    assert!( list[6].value == "shard" );
    assert!( list[7].value == "shard" );
    assert!( list[8].value == "shard" );
    assert!( list[9].value == "shard" );
}
