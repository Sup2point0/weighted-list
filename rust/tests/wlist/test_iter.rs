use itertools::Itertools;

use crate::*;
use weighted_list::*;


#[test] fn into_iter()
{
    let list = wl();

    assert_eq!(
        list.into_iter().find(|item| item.weight == 2).unwrap(),
        wit!(2, str!("sup"))
    );
}
#[test] fn iter()
{
    let list = wl();

    for _ in list.iter() {}

    assert_eq!(
        list.iter().find(|item| item.weight == 2).unwrap(),
        &wit!(2, str!("sup"))
    );
}

#[test] fn iter_mut()
{
    let mut list = wl();

    for item in list.iter_mut() {
        item.weight += 1;
    }

    assert_eq!(
        list,
        wlist![
            (3, str!("sup")),
            (4, str!("nova")),
            (6, str!("shard")),
        ]
    );

    list.iter_mut().map(|item| item.weight -= 1).collect_vec();

    assert_eq!( list, wl() );
}

#[test] fn iter_sugar()
{
    let mut list = wl();

    for _ in &list {}

    for item in &mut list {
        item.weight += 1;
    }

    assert_eq!(
        list,
        wlist![
            (3, str!("sup")),
            (4, str!("nova")),
            (6, str!("shard")),
        ]
    );
}
