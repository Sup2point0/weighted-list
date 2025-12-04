use statrs::distribution as dist;
use statrs::distribution::DiscreteCDF;
use statrs::statistics::Distribution;

use weighted_list::*;


const TRIALS:             u64 = 20000;
const CONFIDENCE_PERCENT: i32 = 95;
const CRITICAL_PERCENT:   i32 = 100 - CONFIDENCE_PERCENT;
const SIGNIFICANCE_LEVEL: f64 = CRITICAL_PERCENT as f64 / 200.0;


#[allow(dead_code, non_camel_case_types)]
pub enum Method {
    SELECT_SINGLE,
    SELECT_MANY,
    SHUFFLE,
}


#[allow(dead_code)]
pub fn test_binomial<V>(
    wlist: &WeightedList<V, i32>,
    method: Method,
) -> ()
    where V: Clone + Eq + std::fmt::Display,
{
    let mut rng = rand::rng();

    let mut test_binomial_single = |item: &WeightedItem<V, i32>| {
        let value = &item.value;

        let prob = match method {
            Method::SHUFFLE => 1.0 / wlist.total_values() as f64,
            _               => item.weight as f64 / wlist.len() as f64,
        };

        let binomialdist = dist::Binomial::new(prob, TRIALS).unwrap();

        let mut observed: u64 = 0;

        match method {
            Method::SELECT_SINGLE => {
                for _ in 0..TRIALS {
                    if *wlist.select_random_value(&mut rng).unwrap() == *value {
                        observed += 1;
                    }
                }
            },
            Method::SELECT_MANY => {
                observed += wlist.select_random_values().rng(&mut rng)
                    .count(TRIALS as u32)
                    .call().iter()
                    .filter(|val| *val == value).count() as u64;
            },
            Method::SHUFFLE => {
                for _ in 0..TRIALS {
                    if wlist.shuffled_weights(&mut rng) == *wlist {
                        observed += 1;
                    }
                }
            },
        }

        let expected = binomialdist.mean().unwrap().round() as i32;
        let lower_bound = binomialdist.inverse_cdf(SIGNIFICANCE_LEVEL);
        let upper_bound = binomialdist.inverse_cdf(1.0 - SIGNIFICANCE_LEVEL);

        let err_lower = format!(
            "OUTLIER: too few -- got `{value}`: {observed} times -- expected: {expected} -- critical region < {lower_bound} has probability: {}", SIGNIFICANCE_LEVEL
        );
        
        let err_upper = format!(
            "OUTLIER: too many -- got `{value}`: {observed} times -- expected: {expected} -- critical region > {upper_bound} has probability: {}", SIGNIFICANCE_LEVEL
        );

        assert!( observed > lower_bound, "{err_lower}");
        assert!( observed < upper_bound, "{err_upper}");

        let deviation = (
            (
                1000.0 * (
                    (observed as i32 - expected) as f64
                    / expected as f64
                )
            ).round() / 10.0
        ).round() as i32;

        println!(
            "CONSISTENT -- got `{value}`: {observed}/{TRIALS} times -- expected: {expected} -- confidence interval: {lower_bound}...{upper_bound} -- deviation: {deviation}%",
        );
    };

    for item in wlist.iter() {
        test_binomial_single(&item);
    }
}
