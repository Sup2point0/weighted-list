use weighted_list::WeightedList;


#[allow(dead_code)]
pub fn data_string() -> [(i32, String); 3]
{
    [
        (2, String::from("sup")),
        (3, String::from("nova")),
        (5, String::from("shard")),
    ]
}

#[allow(dead_code)]
pub fn el() -> WeightedList<String, i32>
{
    WeightedList::new()
}

#[allow(dead_code)]
pub fn wl() -> WeightedList<String, i32>
{
    WeightedList::from(data_string())
}
