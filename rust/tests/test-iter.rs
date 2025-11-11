// use weighted_list::WeightedList;
mod utils;
use utils::*;


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
