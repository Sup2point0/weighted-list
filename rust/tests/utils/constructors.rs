use crate::str;

use weighted_list::{FrozenWeightedList, WeightedList};


#[allow(dead_code)]
pub fn data_string(long: bool) -> Vec<(i32, String)>
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


/// Construct an empty `WeightedList` for testing.
#[allow(dead_code)]
pub fn el() -> WeightedList<String, i32>
{
    WeightedList::new()
}

/// Construct a `WeightedList` for testing:
/// 
/// ```ignore
/// wl => wlist![
///   (2, "sup"),
///   (3, "nova"),
///   (5, "shard")
/// ]
/// ```
#[allow(dead_code)]
pub fn wl() -> WeightedList<String, i32>
{
    WeightedList::init(data_string(false))
}

/// Construct a `WeightedList` for testing with more items:
/// 
/// ```ignore
/// wl => wlist![
///   (2, "sup"),
///   (3, "nova"),
///   (5, "shard")
///   (7, "cortex"),
///   (13, "origin"),
///   (20, "vision"),
/// ]
/// ```
#[allow(dead_code)]
pub fn wll() -> WeightedList<String, i32>
{
    WeightedList::init(data_string(true))
}


/// Construct an empty `FrozenWeightedList` for testing.
#[allow(dead_code)]
pub fn efl() -> FrozenWeightedList<String, i32>
{
    FrozenWeightedList::new()
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
pub fn fwl() -> FrozenWeightedList<String, i32>
{
    FrozenWeightedList::init(data_string(false))
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
pub fn fwll() -> FrozenWeightedList<String, i32>
{
    FrozenWeightedList::init(data_string(true))
}
