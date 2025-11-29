mod utils;
use utils::*;

use weighted_list::*;


#[test]
fn take_one()
{
    let mut list = wl();

    list.take_one(0);
    assert_eq!(list, wlist!(
        (1, str!("sup")),
        (3, str!("nova")),
        (5, str!("shard"))
    ));

    list.take_one(8);
    assert_eq!(list, wlist!(
        (1, str!("sup")),
        (3, str!("nova")),
        (4, str!("shard"))
    ));

    list.take_one(3);
    assert_eq!(list, wlist!(
        (1, str!("sup")),
        (2, str!("nova")),
        (4, str!("shard"))
    ));
}

#[test]
fn take_by()
{
    let mut list = wl();

    list.take_by(0, 2);
    assert_eq!(list, wlist!(
        (3, str!("nova")),
        (5, str!("shard"))
    ));
}
