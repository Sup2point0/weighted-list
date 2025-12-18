use crate::*;
use weighted_list::*;


// == IN-PLACE == //

#[test] fn push()
{
    assert_eq!(
        *el().push_item(WeightedItem::unit(str!("elysion"))),
        wlist!( (1, str!("elysion")) )
    );
    assert_eq!(
        *wl().push_item(WeightedItem::unit(str!("elysion"))),
        wlist!(
            (2, str!("sup")),
            (3, str!("nova")),
            (5, str!("shard")),
            (1, str!("elysion"))
        )
    );

    assert_eq!(
        *el().push_new_item(1, str!("elysion")),
        wlist!( (1, str!("elysion")) )
    );
    assert_eq!(
        *wl().push_new_item(1, str!("elysion")),
        wlist!(
            (2, str!("sup")),
            (3, str!("nova")),
            (5, str!("shard")),
            (1, str!("elysion"))
        )
    );

    assert_eq!(
        *el().push_value(str!("elysion")),
        wlist!( (1, str!("elysion")) )
    );
}

#[test] fn insert()
{
    assert_eq!(
        *el().insert_item(0, WeightedItem::new(1, str!("elysion"))),
        wlist!( (1, str!("elysion")) )
    );

    assert_eq!(
        *el().insert_value(0, str!("elysion")),
        wlist!( (1, str!("elysion")) )
    );

    assert_eq!(
        *el().insert_new_item(0, (1, str!("elysion"))),
        wlist!( (1, str!("elysion")) )
    );

    let first = wlist!(
        (1, str!("elysion")),
        (2, str!("sup")),
        (3, str!("nova")),
        (5, str!("shard"))
    );
    assert_eq!( *wl().insert_new_item(0, (1, str!("elysion"))), first );
    assert_eq!( *wl().insert_new_item(1, (1, str!("elysion"))), first );

    let second = wlist!(
        (2, str!("sup")),
        (1, str!("elysion")),
        (3, str!("nova")),
        (5, str!("shard"))
    );
    assert_eq!( *wl().insert_new_item(2, (1, str!("elysion"))), second );
    assert_eq!( *wl().insert_new_item(3, (1, str!("elysion"))), second );
    assert_eq!( *wl().insert_new_item(4, (1, str!("elysion"))), second );

    let third = wlist!(
        (2, str!("sup")),
        (3, str!("nova")),
        (1, str!("elysion")),
        (5, str!("shard"))
    );
    assert_eq!( *wl().insert_new_item(5, (1, str!("elysion"))), third );
    assert_eq!( *wl().insert_new_item(6, (1, str!("elysion"))), third );
    assert_eq!( *wl().insert_new_item(7, (1, str!("elysion"))), third );
    assert_eq!( *wl().insert_new_item(8, (1, str!("elysion"))), third );
    assert_eq!( *wl().insert_new_item(9, (1, str!("elysion"))), third );

    let fourth = wlist!(
        (2, str!("sup")),
        (3, str!("nova")),
        (5, str!("shard")),
        (1, str!("elysion"))
    );
    assert_eq!( *wl().insert_new_item(10, (1, str!("elysion"))), fourth );
    assert_eq!( *wl().insert_new_item(11, (1, str!("elysion"))), fourth );
}

#[test] fn remove_at()
{
    let orig = wl();
    let mut list = wl();

    assert_eq!( list.remove_at(0), orig.items()[0] );
    assert_eq!( list.remove_at(0), orig.items()[1] );
    assert_eq!( list.remove_at(0), orig.items()[2] );
}

#[test] fn truncate()
{
    assert_eq!( *el().truncate(0), el() );
    assert_eq!( *el().truncate(1), el() );
    assert_eq!( *el().truncate(10), el() );

    assert_eq!( *wl().truncate(0), el() );
    assert_eq!( *wl().truncate(1), wlist![(1, str!("sup"))] );
    assert_eq!( *wl().truncate(2), wlist![(2, str!("sup"))] );
    assert_eq!( *wl().truncate(3), wlist![(2, str!("sup")), (1, str!("nova"))] );
    assert_eq!( *wl().truncate(4), wlist![(2, str!("sup")), (2, str!("nova"))] );
    assert_eq!( *wl().truncate(5), wlist![(2, str!("sup")), (3, str!("nova"))] );
    assert_eq!( *wl().truncate(6), wlist![(2, str!("sup")), (3, str!("nova")), (1, str!("shard"))] );

    assert_eq!( *wl().truncate(wl().len()), wl() );
}

#[test] fn retain()
{}

#[test] fn retain_mut()
{}

#[test] fn clear()
{
    assert_eq!( *wl().clear(), el() );
}


// == OUT-OF-PLACE == //

#[test] fn sorted()
{
    assert_eq!( el().sorted(), el() );
    assert_eq!( wl().sorted(), wl() );
    assert_eq!( wl().reversed().sorted(), wl() );
}
