use weighted_list::WeightedList;


fn wl() -> WeightedList<String, i32>
{
    WeightedList::new(Vec::from([
        (2, String::from("sup")),
        (3, String::from("nova")),
        (5, String::from("shard")),
    ]))
}


#[test]
fn test_constructors()
{
    let _: WeightedList<String, i32>;

    _ = WeightedList::from([
        (2, String::from("sup")),
        (3, String::from("nova")),
        (5, String::from("shard")),
    ]);

    let _: WeightedList<bool, f64>;

    _ = WeightedList::from([
        (2.0, false),
        (4.2, true),
    ]);
}

fn test_equality()
{
    assert!( WeightedList::empty() == WeightedList::empty() );
    assert!( wl() == wl() );
}
