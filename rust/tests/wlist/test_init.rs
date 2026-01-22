use std::collections::HashSet;

use crate::*;
use weighted_list::*;


#[test] fn constructors()
{
    let _: WL = el();
    let _: WL = wl();
    let _: WL = wll();

    let list: WL = WList::with_capacity(42);
    assert!( list.capacity() >= 42 );
}

#[test] fn macro_constructor()
{
    let list = wlist![
        (2, str!("sup")),
        (3, str!("nova")),
        (5, str!("shard")),
    ];

    assert_eq!( list[0], wit!(2, str!("sup")) )
}

#[test] fn from_expanded()
{
    assert_eq!( WList::from_expanded([]), el() );

    assert_eq!(
        WList::from_expanded(["qi", "sup", "sup"]),
        wlist![(1, "qi"), (2, "sup")]
    );

    assert_eq!(
        WList::from_expanded([
            str!("sup"), str!("sup"),
            str!("nova"), str!("nova"), str!("nova"),
            str!("shard"), str!("shard"), str!("shard"), str!("shard"), str!("shard"),
        ]),
        wl()
    );
}

#[test] fn from_iterator()
{
}

#[test] fn from_pairs()
{
    // From Vec<(W, V)>
    assert_eq!( WList::from(data_string(false)), wl() );

    // From [(W, V); N]
    let e: [(u32, String); 0] = [];
    assert_eq!( WList::from(e), el() );

    assert_eq!(
        WList::from([
            (2, str!("sup")),
            (3, str!("nova")),
            (5, str!("shard")),
        ]),
        wl()
    );
}

#[test] fn from_iter_pairs()
{
    // FromIterator Vec<(W, V)>
    let vec: Vec<(u32, String)> = vec![];
    assert_eq!( WL::from_iter(vec), el() );

    assert_eq!(
        WList::from_iter([
            (2, str!("sup")),
            (3, str!("nova")),
            (5, str!("shard")),
        ]),
        wl()
    );

    // FromIterator [(W, V); N]
    let array: [(u32, String); 0] = [];
    assert_eq!( WL::from_iter(array), el() );

    assert_eq!(
        WList::from_iter([
            (2, str!("sup")),
            (3, str!("nova")),
            (5, str!("shard")),
        ]),
        wl()
    );

    // FromIterator HashSet<(W,V)>
    let list = WList::from_iter(
        HashSet::from([
            (2, str!("sup")),
            (3, str!("nova")),
            (5, str!("shard")),
        ])
    );

    assert_eq!(
        HashSet::<WI>::from_iter(list),
        HashSet::<WI>::from_iter(wl())
    );
}

#[test] fn from_items()
{
    // From WList<V,W>
    assert_eq!( WList::from(wl()), wl() );
    
    // From Vec<WItem<V,W>>
    assert_eq!( WList::from(wl().items().clone()), wl() );
}

#[test] fn from_iter_items()
{
    // FromIterator WList<V,W>
    assert_eq!( WList::from_iter(wl()), wl() );
    
    // FromIterator Vec<WItem<V,W>>
    assert_eq!( WList::from_iter(wl().items().clone()), wl() );
}

#[test] fn asref_vec()
{
    let list = wl();

    fn test<T>(_: impl AsRef<Vec<T>>) {}

    test(list);
}

#[test] fn deref_to_slice()
{
    let list = wl();

    let _ = list.first();
    let _ = (*list).len();
}
