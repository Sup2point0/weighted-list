use crate::*;
use weighted_list::*;


#[test] fn weights()
{
    assert!( el().weights().eq([0; 0]) );
    assert!( wl().weights().eq([2, 3, 5]) );
}

#[test] fn values()
{
    assert!( el().values().eq([""; 0]) );
    assert!( wl().values().eq(["sup", "nova", "shard"]) );
}

#[test] fn items()
{
    let e: [WeightedItem<String, i32>; 0] = [];
    assert!( el().items().eq(&e) );

    assert!( wl().items().eq(&[
        wit!(2, str!("sup")),
        wit!(3, str!("nova")),
        wit!(5, str!("shard")),
    ]) );
}

#[test] fn raw()
{
    let e: [(i32, &String); 0] = [];
    assert!( el().raw().eq(e) );

    assert!(
        wl().raw().eq([
            (2, &str!("sup")),
            (3, &str!("nova")),
            (5, &str!("shard")),
        ])
    );
}

#[test] fn expanded()
{
    assert!(
        wl().expanded().eq([
            &str!("sup"),
            &str!("sup"),
            &str!("nova"),
            &str!("nova"),
            &str!("nova"),
            &str!("shard"),
            &str!("shard"),
            &str!("shard"),
            &str!("shard"),
            &str!("shard"),
        ])
    );
}
