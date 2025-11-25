mod utils;
// use utils::*;

use weighted_list::*;


#[test]
fn select_single()
{
    let list: WeightedList<String, i32> = wlist!(
        (100, str!("sup")),
        (5, str!("woah"))
    );
    let outs = vec![str!("sup"), str!("woah")];

    let mut out;
    let mut rng = rand::rng();

    for _ in 0..50 {
        out = list.select_random_value(&mut rng);
        assert!( outs.contains(out.unwrap()) );
    }
}
