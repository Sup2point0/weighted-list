mod utils;

use utils::*;
use weighted_list::*;


#[test]
fn push()
{
    assert!(
        *el().push_item(WeightedItem::unit(String::from("elysion")))
            == WeightedList::from([
                (1, String::from("elysion"))
            ])
    );
    assert!(
        *wl().push_item(WeightedItem::unit(String::from("elysion")))
            == WeightedList::from([
                (2, String::from("sup")),
                (3, String::from("nova")),
                (5, String::from("shard")),
                (1, String::from("elysion"))
            ])
    );

    assert!(
        *el().push_new_item(1, String::from("elysion"))
            == WeightedList::from([
                (1, String::from("elysion"))
            ])
    );
    assert!(
        *wl().push_new_item(1, String::from("elysion"))
            == WeightedList::from([
                (2, String::from("sup")),
                (3, String::from("nova")),
                (5, String::from("shard")),
                (1, String::from("elysion"))
            ])
    );

    assert!(
        *el().push_value(String::from("elysion"))
            == WeightedList::from([
                (1, String::from("elysion"))
            ])
    );
}

#[test]
fn insert()
{
    assert!(
        *el().insert_item(0, WeightedItem::new(1, String::from("elysion")))
            == WeightedList::from([
                (1, String::from("elysion"))
            ])
    );

    assert!(
        *el().insert_value(0, String::from("elysion"))
            == WeightedList::from([
                (1, String::from("elysion"))
            ])
    );

    assert!(
        *el().insert_new_item(0, (1, String::from("elysion")))
            == WeightedList::from([
                (1, String::from("elysion"))
            ])
    );

    let first = WeightedList::from([
        (1, String::from("elysion")),
        (2, String::from("sup")),
        (3, String::from("nova")),
        (5, String::from("shard"))
    ]);
    assert!( *wl().insert_new_item(0, (1, String::from("elysion"))) == first );
    assert!( *wl().insert_new_item(1, (1, String::from("elysion"))) == first );

    let second = WeightedList::from([
        (2, String::from("sup")),
        (1, String::from("elysion")),
        (3, String::from("nova")),
        (5, String::from("shard"))
    ]);
    assert!( *wl().insert_new_item(2, (1, String::from("elysion"))) == second );
    assert!( *wl().insert_new_item(3, (1, String::from("elysion"))) == second );
    assert!( *wl().insert_new_item(4, (1, String::from("elysion"))) == second );

    let third = WeightedList::from([
        (2, String::from("sup")),
        (3, String::from("nova")),
        (1, String::from("elysion")),
        (5, String::from("shard"))
    ]);
    assert!( *wl().insert_new_item(5, (1, String::from("elysion"))) == third );
    assert!( *wl().insert_new_item(6, (1, String::from("elysion"))) == third );
    assert!( *wl().insert_new_item(7, (1, String::from("elysion"))) == third );
    assert!( *wl().insert_new_item(8, (1, String::from("elysion"))) == third );
    assert!( *wl().insert_new_item(9, (1, String::from("elysion"))) == third );

    let fourth = WeightedList::from([
        (2, String::from("sup")),
        (3, String::from("nova")),
        (5, String::from("shard")),
        (1, String::from("elysion"))
    ]);
    assert!( *wl().insert_new_item(10, (1, String::from("elysion"))) == fourth );
    assert!( *wl().insert_new_item(11, (1, String::from("elysion"))) == fourth );
}
