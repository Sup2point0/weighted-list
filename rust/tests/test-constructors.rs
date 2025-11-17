mod utils;
use utils::*;

use weighted_list::*;


#[test]
fn constructors()
{
    let _: WeightedList<String, i32> = el();

    let _: WeightedList<String, i32> = wl();

    let _: WeightedList<bool, f64> = wlist!(
        (2.0, false),
        (4.2, true)
    );
}

#[test]
fn macros()
{
    let _: WeightedList<String, i32> = wlist!(
        (2, String::from("sup")),
        (3, String::from("nova")),
        (5, String::from("shard"))
    );
}
