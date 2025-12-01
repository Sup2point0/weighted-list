mod utils;
use utils::*;

// use weighted_list::*;


#[test]
fn test_chaining()
{
    let mut list = el();

    list.push_value(str!("sup"))
        .push_value(str!("nova"))
        .merge_value(str!("sup"))
        .merge_with(wl())
        .insert_value(50, str!("sup"))
        .merge_duplicates();
}
