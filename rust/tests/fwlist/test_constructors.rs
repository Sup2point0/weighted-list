use crate::*;
use weighted_list::*;


#[test] fn constructors()
{
    let _: FWList<String, u32> = efl();

    let _: FWList<String, u32> = fwl();

    let _: FWList<bool, f64> = fwlist!(
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
    let list: FWList<String, u32> = fwlist!(
        (2, str!("sup")),
        (3, str!("nova")),
        (5, str!("shard"))
    );

    assert_eq!( list[0], fwit!(2, 2, str!("sup")) )
}
