use crate::*;
use weighted_list::*;


#[test] fn prune()
{
    assert_eq!( *el().prune(), el() );
    assert_eq!( *wl().prune(), wl() );
    assert_eq!( *wlist![(0, str!("sup"))].prune(), el() );
    assert_eq!( *wlist![(-1, str!("sup"))].prune(), el() );
}

#[test] fn set_weights()
{
    assert_eq!( *el().zero_all_weights(), el() );

    assert_eq!(
        *wl().zero_all_weights(),
        wlist![
            (0, str!("sup")),
            (0, str!("nova")),
            (0, str!("shard")),
        ]
    );

    assert_eq!( *el().set_all_weights(37), el() );

    assert_eq!(
        *wl().set_all_weights(37),
        wlist![
            (37, str!("sup")),
            (37, str!("nova")),
            (37, str!("shard")),
        ]
    );
}

#[test] fn normalise()
{
    assert_eq!( el().normalised().unwrap(), wlist![] );

    assert_eq!(
        wl().normalised().unwrap(),
        wlist![
            (0.2, str!("sup")),
            (0.3, str!("nova")),
            (0.5, str!("shard")),
        ]
    );
}

#[test] fn merge_single()
{
    assert_eq!(
        *el().merge_item(wit!(2, str!("sup"))),
        wlist![(2, str!("sup"))]
    );
    assert_eq!(
        *wl().merge_item(wit!(2, str!("sup"))),
        wlist![
            (4, str!("sup")),
            (3, str!("nova")),
            (5, str!("shard")),
        ]
    );
    assert_eq!(
        *wl().merge_item(wit!(2, str!("elysion"))),
        wlist![
            (2, str!("sup")),
            (3, str!("nova")),
            (5, str!("shard")),
            (2, str!("elysion")),
        ]
    );
    
    assert_eq!(
        *el().merge_new_item(2, str!("sup")),
        wlist![(2, str!("sup"))]
    );
    assert_eq!(
        *wl().merge_new_item(2, str!("sup")),
        wlist![
            (4, str!("sup")),
            (3, str!("nova")),
            (5, str!("shard")),
        ]
    );
    assert_eq!(
        *wl().merge_new_item(2, str!("elysion")),
        wlist![
            (2, str!("sup")),
            (3, str!("nova")),
            (5, str!("shard")),
            (2, str!("elysion")),
        ]
    );
    
    assert_eq!(
        *el().merge_value(str!("sup")),
        wlist![(1, str!("sup"))]
    );
    assert_eq!(
        *wl().merge_value(str!("sup")),
        wlist![
            (3, str!("sup")),
            (3, str!("nova")),
            (5, str!("shard")),
        ]
    );
    assert_eq!(
        *wl().merge_value(str!("elysion")),
        wlist![
            (2, str!("sup")),
            (3, str!("nova")),
            (5, str!("shard")),
            (1, str!("elysion")),
        ]
    );
}

#[test] fn merge_many()
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

#[test] fn merge_duplicates()
{
    assert_eq!( *el().merge_duplicates(), el() );
    assert_eq!( *wl().merge_duplicates(), wl() );
    assert_eq!(
        *wlist![(1, "sup"), (2, "sup")].merge_duplicates(),
        wlist![(3, "sup")]
    );
}

#[test] fn take_one_at()
{
    let mut list = wl();

    list.take_one_at(0);
    assert_eq!(list, wlist!(
        (1, str!("sup")),
        (3, str!("nova")),
        (5, str!("shard"))
    ));

    list.take_one_at(8);
    assert_eq!(list, wlist!(
        (1, str!("sup")),
        (3, str!("nova")),
        (4, str!("shard"))
    ));

    list.take_one_at(3);
    assert_eq!(list, wlist!(
        (1, str!("sup")),
        (2, str!("nova")),
        (4, str!("shard"))
    ));
}

#[test] fn take_by_at()
{
    let mut list = wl();

    list.take_by_at(2, 0);
    assert_eq!(list, wlist!(
        (3, str!("nova")),
        (5, str!("shard"))
    ));
}
