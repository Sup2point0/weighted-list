mod utils;
use utils::*;

use weighted_list::*;


#[test]
fn index()
{
    let list = wl();

    assert_eq!( list[0].value, "sup" );
    assert_eq!( list[1].value, "sup" );
    assert_eq!( list[2].value, "nova" );
    assert_eq!( list[3].value, "nova" );
    assert_eq!( list[4].value, "nova" );
    assert_eq!( list[5].value, "shard" );
    assert_eq!( list[6].value, "shard" );
    assert_eq!( list[7].value, "shard" );
    assert_eq!( list[8].value, "shard" );
    assert_eq!( list[9].value, "shard" );
}

#[test]
fn iter_methods()
{
    let mut list = wl();

    for _ in list.iter() {}

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
}

#[test]
fn iter_sugar()
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

    for _ in list {}
}
