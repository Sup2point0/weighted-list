mod utils;
use utils::*;

use weighted_list::*;


#[test]
#[ignore]
fn select_single_stats_1()
{
    stats::test_binomial(&wlist![
        (100, "sup"),
        (1, "ayo"),
    ]);
}

#[test]
#[ignore]
fn select_single_stats_2()
{
    stats::test_binomial(&wlist![
        (100, "sup"),
        (5, "woah"),
    ]);
}

#[test]
#[ignore]
fn select_single_stats_3()
{
    stats::test_binomial(&wl());
}

#[test]
#[ignore]
fn select_single_stats_4()
{
    stats::test_binomial(&wlist![
        (1000, "sup"),
        (1, "WOAH")
    ]);
}
