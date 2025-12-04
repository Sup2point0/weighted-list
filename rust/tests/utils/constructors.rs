use crate::str;

use weighted_list::WeightedList;


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


#[allow(dead_code)]
pub fn el() -> WeightedList<String, i32>
{
    WeightedList::new()
}

#[allow(dead_code)]
pub fn wl() -> WeightedList<String, i32>
{
    WeightedList::init(data_string(false))
}

#[allow(dead_code)]
pub fn wll() -> WeightedList<String, i32>
{
    WeightedList::init(data_string(true))
}
