mod utils;
use utils::*;

use weighted_list::*;


#[test]
fn merge_single()
{
    assert_eq!(
        *el().merge_item(wit!(1, str!("sup"))),
        wlist![(1, str!("sup"))]
    );

    assert_eq!(
        *wl().merge_item(wit!(1, str!("sup"))),
        wlist![
            (3, str!("sup")),
            (3, str!("nova")),
            (5, str!("shard")),
        ]
    );

    assert_eq!(
        *wl().merge_item(wit!(1, str!("elysion"))),
        wlist![
            (2, str!("sup")),
            (3, str!("nova")),
            (5, str!("shard")),
            (1, str!("elysion")),
        ]
    );
}

#[test]
fn merge_many()
{
    assert_eq!( *el().merge_with(el()), el() );
    assert_eq!( *el().merge_with(wl()), wl() );

    assert_eq!(
        *wl().merge_with(wl()),
        wlist![(4, str!("sup")), (6, str!("nova")), (10, str!("shard"))]
    );

    assert_eq!( *el().merge_duplicates(), el() );
    assert_eq!( *wl().merge_duplicates(), wl() );

    assert_eq!(
        *wlist![(1, str!("sup")), (2, str!("sup"))].merge_duplicates(),
        wlist![(3, str!("sup"))]
    )
}

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
