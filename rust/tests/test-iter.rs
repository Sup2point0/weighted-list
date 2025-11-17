mod utils;
use utils::*;

// use weighted_list::*;


#[test]
fn iter()
{
    let mut list = wl();

    for _ in list.iter() {}

    for item in list.iter_mut() {
        item.weight += 1;
    }

    // assert 

    for _ in list {}

    // for item in &mut list {
    //     item.weight += 1;
    // }
}
