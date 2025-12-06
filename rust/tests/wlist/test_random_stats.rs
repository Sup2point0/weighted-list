use crate::*;

use weighted_list::*;


#[test] #[ignore]
fn select_single_stats_1()
{
    stats::test_binomial(
        &wlist![(100, "sup"), (1, "ayo")],
        stats::Method::SELECT_SINGLE
    );
}

#[test] #[ignore]
fn select_single_stats_2()
{
    stats::test_binomial(
        &wlist![(100, "sup"), (5, "woah")],
        stats::Method::SELECT_SINGLE
    );
}

#[test] #[ignore]
fn select_single_stats_3()
{
    stats::test_binomial(
        &wl(),
        stats::Method::SELECT_SINGLE
    );
}

#[test] #[ignore]
fn select_single_stats_4()
{
    stats::test_binomial(
        &wlist![(1000, "sup"), (1, "WOAH")],
        stats::Method::SELECT_SINGLE
    );
}

#[test] #[ignore]
fn select_many_stats_1()
{
    stats::test_binomial(&wl(), stats::Method::SELECT_MANY);
}

#[test] #[ignore]
fn shuffle_stats_1()
{
    stats::test_binomial(&wl(), stats::Method::SHUFFLE);
}

#[test] #[ignore]
fn shuffle_stats_2()
{
    stats::test_binomial(&wll(), stats::Method::SHUFFLE);
}
