// use utils::*;

use statrs::distribution as dist;
use statrs::distribution::DiscreteCDF;
use statrs::statistics::Distribution;

use weighted_list::*;


const SIGNIFICANCE_LEVEL: f64 = 0.05;


#[test]
#[ignore]
fn select_single_stats()
{
    let mut rng = rand::rng();

    let list = wlist![
        (100, "sup"),
        (5, "woah"),
    ];

    let n = 3000;

    '_sup: {
        let p = list.items()[0].weight as f64 / list.len() as f64;
        let binomialdist = dist::Binomial::new(p, n).unwrap();

        let mut observed = 0;

        for _ in 0..n {
            if *list.select_random_value(&mut rng).unwrap() == "sup" {
                observed += 1;
            }
        }

        let lower_bound = binomialdist.inverse_cdf(SIGNIFICANCE_LEVEL);
        let upper_bound = binomialdist.inverse_cdf(1.0 - SIGNIFICANCE_LEVEL);

        let err_lower = format!("Got `sup` {observed} times, expected < {lower_bound} with probability {SIGNIFICANCE_LEVEL}");
        let err_upper = format!("Got `sup` {observed} times, expected > {upper_bound} with probability {SIGNIFICANCE_LEVEL}");

        assert!( observed >= lower_bound, "{err_lower}");
        assert!( observed <= upper_bound, "{err_upper}");

        println!("Got `sup` {observed}/{n} times, expected {}", binomialdist.mean().unwrap().round());
    }

    '_woah: {
        let p = list.items()[1].weight as f64 / list.len() as f64;
        let binomialdist = dist::Binomial::new(p, n).unwrap();

        let mut observed = 0;

        for _ in 0..n {
            if *list.select_random_value(&mut rng).unwrap() == "woah" {
                observed += 1;
            }
        }

        let lower_bound = binomialdist.inverse_cdf(SIGNIFICANCE_LEVEL);
        let upper_bound = binomialdist.inverse_cdf(1.0 - SIGNIFICANCE_LEVEL);

        let err_lower = format!("Got `woah` {observed} times, expected < {lower_bound} with probability {SIGNIFICANCE_LEVEL}");
        let err_upper = format!("Got `woah` {observed} times, expected > {upper_bound} with probability {SIGNIFICANCE_LEVEL}");

        assert!( observed >= lower_bound, "{err_lower}");
        assert!( observed <= upper_bound, "{err_upper}");
        
        println!("Got `woah` {observed}/{n} times, expected {}", binomialdist.mean().unwrap().round());
    }
}
