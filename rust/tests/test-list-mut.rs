mod utils;
use utils::*;

use weighted_list::*;


#[test]
fn push()
{
    assert!( *el().push_item(WeightedItem::unit(str!("elysion")))
        == wlist!(
            (1, str!("elysion"))
        )
    );
    assert!( *wl().push_item(WeightedItem::unit(str!("elysion")))
        == wlist!(
            (2, str!("sup")),
            (3, str!("nova")),
            (5, str!("shard")),
            (1, str!("elysion"))
        )
    );

    assert!( *el().push_new_item(1, str!("elysion"))
        ==  wlist!(
            (1, str!("elysion"))
        )
    );
    assert!( *wl().push_new_item(1, str!("elysion"))
        == wlist!(
            (2, str!("sup")),
            (3, str!("nova")),
            (5, str!("shard")),
            (1, str!("elysion"))
        )
    );

    assert!( *el().push_value(str!("elysion"))
        == wlist!(
            (1, str!("elysion"))
        )
    );
}

#[test]
fn insert()
{
    assert!( *el().insert_item(0, WeightedItem::new(1, str!("elysion")))
        == wlist!(
            (1, str!("elysion"))
        )
    );

    assert!( *el().insert_value(0, str!("elysion"))
        == wlist!(
            (1, str!("elysion"))
        )
    );

    assert!( *el().insert_new_item(0, (1, str!("elysion")))
        == wlist!(
            (1, str!("elysion"))
        )
    );

    let first = wlist!(
        (1, str!("elysion")),
        (2, str!("sup")),
        (3, str!("nova")),
        (5, str!("shard"))
    );
    assert!( *wl().insert_new_item(0, (1, str!("elysion"))) == first );
    assert!( *wl().insert_new_item(1, (1, str!("elysion"))) == first );

    let second = wlist!(
        (2, str!("sup")),
        (1, str!("elysion")),
        (3, str!("nova")),
        (5, str!("shard"))
    );
    assert!( *wl().insert_new_item(2, (1, str!("elysion"))) == second );
    assert!( *wl().insert_new_item(3, (1, str!("elysion"))) == second );
    assert!( *wl().insert_new_item(4, (1, str!("elysion"))) == second );

    let third = wlist!(
        (2, str!("sup")),
        (3, str!("nova")),
        (1, str!("elysion")),
        (5, str!("shard"))
    );
    assert!( *wl().insert_new_item(5, (1, str!("elysion"))) == third );
    assert!( *wl().insert_new_item(6, (1, str!("elysion"))) == third );
    assert!( *wl().insert_new_item(7, (1, str!("elysion"))) == third );
    assert!( *wl().insert_new_item(8, (1, str!("elysion"))) == third );
    assert!( *wl().insert_new_item(9, (1, str!("elysion"))) == third );

    let fourth = wlist!(
        (2, str!("sup")),
        (3, str!("nova")),
        (5, str!("shard")),
        (1, str!("elysion"))
    );
    assert!( *wl().insert_new_item(10, (1, str!("elysion"))) == fourth );
    assert!( *wl().insert_new_item(11, (1, str!("elysion"))) == fourth );
}
