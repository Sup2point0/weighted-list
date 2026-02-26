use crate::str;

use weighted_list::{ WList, FWList };


#[allow(dead_code)]
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


#[allow(dead_code)]
pub fn el() -> WList<String, u32>
{
    WList::new()
}

#[allow(dead_code)]
pub fn wl() -> WList<String, u32>
{
    WList::from(data_string(false))
}

#[allow(dead_code)]
pub fn wll() -> WList<String, u32>
{
    WList::from(data_string(true))
}


/// Construct an empty `FrozenWeightedList` for testing.
#[allow(dead_code)]
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
#[allow(dead_code)]
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
#[allow(dead_code)]
pub fn fwll() -> FWList<String, u32>
{
    FWList::init(data_string(true))
}
