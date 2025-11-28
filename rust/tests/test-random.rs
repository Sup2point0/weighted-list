mod utils;
use utils::*;

use weighted_list::*;


#[test]
fn select_single()
{
    let mut rng = rand::rng();

    let list: WeightedList<String, i32> = wlist!(
        (100, str!("sup")),
        (5, str!("woah"))
    );

    let outs = vec!["sup", "woah"];
    let mut out;
    for _ in 0..50 {
        out = list.select_random_value(&mut rng);
        assert!( outs.contains(&out.unwrap().as_str()) );
    }

    let outs = vec![wit!(100, str!("sup")), wit!(5, str!("woah"))];
    let mut out;
    for _ in 0..50 {
        out = list.select_random_item(&mut rng);
        assert!( outs.contains(out.unwrap()) )
    }
}

#[test]
fn take_single()
{
    let mut rng = rand::rng();

    let mut list = wl();
    list.take_random_item_entire(&mut rng); assert_eq!( list.total_values(), 2 );
    list.take_random_item_entire(&mut rng); assert_eq!( list.total_values(), 1 );
    list.take_random_item_entire(&mut rng); assert_eq!( list.total_values(), 0 );

    let mut list = wl();
    list.take_by_random(&mut rng, 1); assert_eq!( list.total_values(), 3 );
    list.take_by_random(&mut rng, 5); assert_eq!( list.total_values(), 2 );
    list.take_by_random(&mut rng, 5); assert_eq!( list.total_values(), 1 );
    list.take_by_random(&mut rng, 5); assert_eq!( list.total_values(), 0 );
}

#[test]
fn select_many()
{

}
