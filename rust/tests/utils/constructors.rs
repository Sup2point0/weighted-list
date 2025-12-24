use crate::str;

use weighted_list::WList;


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
pub fn el() -> WList<String, i32>
{
    WList::new()
}

#[allow(dead_code)]
pub fn wl() -> WList<String, i32>
{
    WList::init(data_string(false))
}

#[allow(dead_code)]
pub fn wll() -> WList<String, i32>
{
    WList::init(data_string(true))
}
