// use weighted_list::WeightedList;
mod utils;
use utils::*;


#[test]
fn accessors()
{
    assert!( el().len() == 0 );
    assert!( wl().len() == 10 );

    let e: [&String; 0] = [];
    assert!( el().values().eq(e) );
    assert!( wl().values().eq(["sup", "nova", "shard"]) );

    let e: [i32; 0] = [];
    assert!( el().weights().eq(e) );
    assert!( wl().weights().eq([2, 3, 5]) );

    let e: [(i32, &String); 0] = [];
    assert!( el().raw().eq(e) );
    assert!( wl().raw().eq([
        (2, &"sup".to_string()),
        (3, &"nova".to_string()),
        (5, &"shard".to_string()),
    ]) );
}
