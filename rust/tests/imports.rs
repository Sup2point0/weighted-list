#![allow(unused_imports)]


#[test] fn imports()
{
    use weighted_list::{
        Weight,
        WeightedItem, WItem,
        WeightedList, WList, wlist,
    };
}

#[test] #[cfg(feature = "frozen")] fn frozen_imports()
{
    use weighted_list::{
        FrozenWeightedItem, FWItem,
        FrozenWeightedList, FWList, fwlist,
    };
}
