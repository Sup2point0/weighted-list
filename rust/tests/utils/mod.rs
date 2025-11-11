use weighted_list::WeightedList;


pub fn data_string() -> [(i32, String); 3]
{
    [
        (2, String::from("sup")),
        (3, String::from("nova")),
        (5, String::from("shard")),
    ]
}

pub fn el() -> WeightedList<String, i32>
{
    WeightedList::new()
}

pub fn wl() -> WeightedList<String, i32>
{
    WeightedList::from(data_string())
}
