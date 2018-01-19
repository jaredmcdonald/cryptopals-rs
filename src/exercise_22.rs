use mersenne_twister::MersenneTwister;
use rand::{Rng, thread_rng};
use utils::now;

fn prng_oracle() -> u32 {
    let unix_timestamp = now();
    let offset = thread_rng().gen_range(40, 1000);
    let seed = unix_timestamp + offset;

    let mut prng = MersenneTwister::new();

    prng.seed(seed);
    prng.rand()
}

fn find_seed_brute_force(r: u32) -> Option<u32> {
    let ts = now();
    let mut count_from = ts + 1000;

    while count_from >= ts - 5 {
        let mut prng = MersenneTwister::new();
        prng.seed(count_from as u32);
        if prng.rand() == r {
            return Some(count_from as u32)
        }
        count_from -= 1;
    }
    None
}

pub fn run_22() {
    let r = prng_oracle();
    println!("bruteforced the seed: {}", find_seed_brute_force(r).unwrap());
}
