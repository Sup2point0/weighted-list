use itertools::*;

use crate::*;


#[test] fn contains_weight()
{
    assert!( wl().contains_weight(2) );
    assert!( !el().contains_weight(2) );
}

#[test] fn contains_value()
{
    assert!( wl().contains_value(&"sup".to_owned()) );
    assert!( !el().contains_value(&"sup".to_owned()) );
}

#[test] fn weighted_sum()
{
    assert_eq!(
        0 as u32,
        el().weighted_sum(|v| v.chars().collect_vec().len() as u32)
    );

    assert_eq!(
        2*3 + 3*4 + 5*5 as u32,
        wl().weighted_sum(|v| v.chars().collect_vec().len() as u32)
    );
}

#[test] fn normalised_weighted_sum()
{
    assert_eq!(
        0 as f64,
        el().normalised_weighted_sum(|v| v.chars().collect_vec().len() as f64).unwrap()
    );
    assert_eq!(
        0.2 * 3.0
        + 0.3 * 4.0
        + 0.5 * 5.0,
        wl().normalised_weighted_sum(|v| v.chars().collect_vec().len() as f64).unwrap()
    );
}
