use crate::*;
use weighted_list::*;


#[test] fn constructors()
{
    let _: FrozenWeightedList<String, i32> = efl();

    let _: FrozenWeightedList<String, i32> = fwl();

    let _: FrozenWeightedList<bool, f64> = fwlist!(
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
    let list: FrozenWeightedList<String, i32> = fwlist!(
        (2, str!("sup")),
        (3, str!("nova")),
        (5, str!("shard"))
    );

    // assert_eq!( list[0], fwit!(2, 2, str!("sup")) )
}
