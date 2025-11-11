use weighted_list::WeightedList;
mod utils;
use utils::*;


#[test]
fn constructors()
{
    let _: WeightedList<String, i32> = el();

    let _: WeightedList<String, i32> = wl();

    let _: WeightedList<bool, f64> = WeightedList::from([
        (2.0, false),
        (4.2, true),
    ]);
}
