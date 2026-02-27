#![allow(dead_code)]

use crate::str;

use weighted_list::WList;

#[cfg(feature = "frozen")]
use weighted_list::FWList;


pub fn data_string(long: bool) -> Vec<(u32, String)>
{
    [
        vec![
            (2, str!("sup")),
            (3, str!("nova")),
            (5, str!("shard")),
        ],
        
        if long {
            vec![
                (7, str!("cortex")),
                (13, str!("origin")),
                (20, str!("vision")),
            ]
        } else {
            vec![]
        }
    ].concat()
}


pub fn el() -> WList<String, u32>
{
    WList::new()
}

pub fn wl() -> WList<String, u32>
{
    WList::from(data_string(false))
}

pub fn wll() -> WList<String, u32>
{
    WList::from(data_string(true))
}


/// Construct an empty `FrozenWeightedList` for testing.
#[cfg(feature = "frozen")]
pub fn efl() -> FWList<String, u32>
{
    FWList::new()
}

/// Construct a `FrozenWeightedList` for testing:
/// 
/// ```ignore
/// wl => fwlist![
///   (2, "sup"),
///   (3, "nova"),
///   (5, "shard")
/// ]
/// ```
#[cfg(feature = "frozen")]
pub fn fwl() -> FWList<String, u32>
{
    FWList::init(data_string(false))
}

/// Construct a `FrozenWeightedList` for testing with more items:
/// 
/// ```ignore
/// fwl => fwlist![
///   (2, "sup"),
///   (3, "nova"),
///   (5, "shard")
///   (7, "cortex"),
///   (13, "origin"),
///   (20, "vision"),
/// ]
/// ```
#[cfg(feature = "frozen")]
pub fn fwll() -> FWList<String, u32>
{
    FWList::init(data_string(true))
}
