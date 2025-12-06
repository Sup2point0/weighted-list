use crate::*;
use weighted_list::*;


#[test] fn constructors()
{
    let _: WeightedList<String, i32> = el();

    let _: WeightedList<String, i32> = wl();

    let _: WeightedList<bool, f64> = wlist!(
        (2.0, false),
        (4.2, true)
    );
    
    let list = wl();
    let mut cloned = list.clone();
    cloned.clear();
    assert_ne!( list, cloned );
}

#[test] fn macros()
{
    let list: WeightedList<String, i32> = wlist!(
        (2, str!("sup")),
        (3, str!("nova")),
        (5, str!("shard"))
    );

    assert_eq!( list[0], wit!(2, str!("sup")) )
}
