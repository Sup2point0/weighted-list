mod utils;

use utils::*;
use weighted_list::*;


#[test]
fn select_single()
{
    let list: WeightedList<String, i32> = WeightedList::from([
        (100, "sup".to_string()),
        (5, "woahhhhh".to_string())
    ]);
    let mut val: String;

    let mut rng = rand::rng();

    for _ in 0..50 {
        val = list.select_random_value(&mut rng).clone();
        println!("{val}");
    }
}
