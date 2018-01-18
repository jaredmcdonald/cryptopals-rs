use std::time::{SystemTime, UNIX_EPOCH};
use mersenne_twister::MersenneTwister;
use rand::{Rng, thread_rng};

fn prng_oracle() -> u32 {
    let start = SystemTime::now();
    let since_the_epoch = start.duration_since(UNIX_EPOCH).unwrap();
    let ms_timestamp = since_the_epoch.as_secs() * 1000 +
        since_the_epoch.subsec_nanos() as u64 / 1_000_000;
    let offset = thread_rng().gen_range(40, 1000);
    let seed = ms_timestamp + offset;

    let mut prng = MersenneTwister::new();
    prng.seed(seed as u32);
    prng.rand()
}

pub fn run_22() {
    println!("{}", prng_oracle());
}
