#![allow(dead_code)]

use crate::str;

use weighted_list::WList;

#[cfg(feature = "frozen")]
use weighted_list::FWList;


// == STRING / INT == //

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
        } else { vec![] }
    ].concat()
}

/// Construct an empty `WeightedList` with integer weights for testing.
pub fn el() -> WList<String, u32>
{
    WList::new()
}

/// Construct an empty `WeightedList` with integer weights for testing:
/// 
/// ```ignore
/// wl => wlist![
///   (2, "sup"),
///   (3, "nova"),
///   (5, "shard"),
/// ]
/// ```
pub fn wl() -> WList<String, u32>
{
    WList::from(data_string(false))
}

/// Construct an empty `WeightedList` with integer weights for testing:
/// 
/// ```ignore
/// wll => wlist![
///   (2, "sup"),
///   (3, "nova"),
///   (5, "shard"),
///   (7, "cortex"),
///   (13, "origin"),
///   (20, "vision"),
/// ]
/// ```
pub fn wll() -> WList<String, u32>
{
    WList::from(data_string(true))
}


// == STRUCT / FLOAT == //

#[derive(Clone, PartialEq, Eq)]
pub struct Testruct<T: Eq>(T);

pub fn data_struct() -> Vec<(f64, Testruct<u8>)>
{
    vec![
        (2.0, Testruct(1)),
        (0.5, Testruct(2)),
        (7.5, Testruct(3)),
    ]
}

pub fn elf() -> WList<Testruct<u8>, f64>
{
    WList::new()
}

pub fn wlf() -> WList<Testruct<u8>, f64>
{
    WList::from(data_struct())
}


// == FROZEN STRING / INT == //

/// Construct an empty `FrozenWeightedList` for testing.
#[cfg(feature = "frozen")]
pub fn efl() -> FWList<String, u32>
{
    FWList::new()
}

/// Construct a `FrozenWeightedList` for testing:
/// 
/// ```ignore
/// fwl => fwlist![
///   (2, "sup"),
///   (3, "nova"),
///   (5, "shard"),
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
/// fwll => fwlist![
///   (2, "sup"),
///   (3, "nova"),
///   (5, "shard"),
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
