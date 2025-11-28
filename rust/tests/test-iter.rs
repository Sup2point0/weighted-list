mod utils;
use utils::*;

use weighted_list::*;


#[test]
fn iter_methods()
{
    let mut list = wl();

    for _ in list.iter() {}

    for item in list.iter_mut() {
        item.weight += 1;
    }

    assert_eq!(list, wlist!(
        (3, str!("sup")),
        (4, str!("nova")),
        (6, str!("shard"))
    ) );
}

#[test]
fn iter_sugar()
{
    let list = wl();
    // let mut list = wl();

    for _ in list {}

    // for item in &list {}

    // for item in &mut list {
    //     item.weight += 1;
    // }
}
