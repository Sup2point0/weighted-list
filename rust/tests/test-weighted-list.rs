use weighted_list::WeightedList;


fn data_string() -> [(i32, String); 3]
{
    [
        (2, String::from("sup")),
        (3, String::from("nova")),
        (5, String::from("shard")),
    ]
}

fn el() -> WeightedList<String, i32>
{
    WeightedList::empty()
}

fn wl() -> WeightedList<String, i32>
{
    WeightedList::from(data_string())
}


#[test]
fn testtconstructors()
{
    let _: WeightedList<String, i32> = el();

    let _: WeightedList<String, i32> = wl();

    let _: WeightedList<bool, f64> = WeightedList::from([
        (2.0, false),
        (4.2, true),
    ]);
}

#[test]
fn testtequality()
{
    assert!( el() == el() );
    assert!( wl() == wl() );

    assert!( el() != wl() );
    assert!( wl() != el() );
}

#[test]
fn testtproperties()
{
    assert!( el().len() == 0 );
    assert!( wl().len() == 10 );

    let e: [&String; 0] = [];
    assert!( el().values().eq(e) );
    assert!( wl().values().eq(["sup", "nova", "shard"]) );

    let e: [i32; 0] = [];
    assert!( el().weights().eq(e) );
    assert!( wl().weights().eq([2, 3, 5]) );

    let e: [(i32, &String); 0] = [];
    assert!( el().raw().eq(e) );
    assert!( wl().raw().eq([
        (2, &"sup".to_string()),
        (3, &"nova".to_string()),
        (5, &"shard".to_string()),
    ]) );
}
