use mersenne_twister::MersenneTwister;

pub fn run_21() {
    let mut rng = MersenneTwister::new();
    rng.seed(1);
    println!("{}\n{}\n{}", rng.rand(), rng.rand(), rng.rand());
}
