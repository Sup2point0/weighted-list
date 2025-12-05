mod utils;
use utils::*;

use itertools::Itertools;

use weighted_list::*;


#[test] fn select_single()
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
        assert!( outs.contains(out.unwrap()) );
    }
}

#[test] fn take_single()
{
    let mut rng = rand::rng();

    let mut list = wl();
    list.take_entire_random(&mut rng); assert_eq!( list.total_values(), 2 );
    list.take_entire_random(&mut rng); assert_eq!( list.total_values(), 1 );
    list.take_entire_random(&mut rng); assert_eq!( list.total_values(), 0 );

    let mut list = wl();
    list.take_by_random(&mut rng, 1); assert_eq!( list.total_values(), 3 );
    list.take_by_random(&mut rng, 5); assert_eq!( list.total_values(), 2 );
    list.take_by_random(&mut rng, 5); assert_eq!( list.total_values(), 1 );
    list.take_by_random(&mut rng, 5); assert_eq!( list.total_values(), 0 );
}

#[test] fn select_many()
{
    let mut rng = rand::rng();

    let list = wl();
    let count = list.len() as usize;

    let valid = vec!["sup", "nova", "shard"];
    let mut results;

    '_standard: {
        for _ in 0..50 {
            results = list.select_random_values()
                .rng(&mut rng)
                .count(count)
                .call();

            for result in results {
                assert!( valid.contains(&result.as_str()) );
            }
        }
    }

    '_excess: {
        for _ in 0..50 {
            results = list.select_random_values()
                .rng(&mut rng)
                .count(count * 2)
                .call();

            for result in results {
                assert!( valid.contains(&result.as_str()) );
            }
        }
    }

    '_unique: {
        for _ in 0..2 {
            results = list.select_random_values()
                .rng(&mut rng)
                .count(count)
                .unique(true)
                .call();

            assert!(
                results.len() == 3,
                "Expected 3 unique items, got {}: {:?}", results.len(), results
            );
            assert!( results.contains(&str!("sup")) );
            assert!( results.contains(&str!("nova")) );
            assert!( results.contains(&str!("shard")) );
        }
    }

    '_replace: {
        let mut counts;

        for _ in 0..50 {
            results = list.select_random_values()
                .rng(&mut rng)
                .count(count)
                .replace(false)
                .call();
            
            counts = results.iter().counts();
            assert_eq!( counts[&str!("sup")], 2 );
            assert_eq!( counts[&str!("nova")], 3 );
            assert_eq!( counts[&str!("shard")], 5 );
        }
    }

    '_replace_decrement: {
        let mut counts;

        for _ in 0..50 {
            results = list.select_random_values()
                .rng(&mut rng)
                .count(count)
                .replace(false)
                .decrement(2)
                .call();
            
            counts = results.iter().counts();
            assert_eq!( counts[&str!("sup")], 1 );
            assert_eq!( counts[&str!("nova")], 2 );
            assert_eq!( counts[&str!("shard")], 3 );
        }
    }
}

#[test] fn shuffle()
{
    let mut list = wl();
    list.append(&mut wl());
    list.append(&mut wl());
    list.append(&mut wl());
    let orig = list.clone();

    let mut rng = rand::rng();
    assert_ne!( *list.shuffle_weights(&mut rng), orig );

    assert_ne!( list.shuffled_weights(&mut rng), list );
}
