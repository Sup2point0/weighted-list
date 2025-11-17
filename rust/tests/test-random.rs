mod utils;
use utils::*;

use weighted_list::*;


#[test]
fn select_single()
{
    let list: WeightedList<String, i32> = wlist!(
        (100, str!("sup")),
        (5, str!("woahhhhh"))
    );

    let mut rng = rand::rng();

    for _ in 0..50 {
        list.select_random_value(&mut rng);
    }
}
