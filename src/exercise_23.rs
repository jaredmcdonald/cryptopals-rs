use mersenne_twister::{MersenneTwister, untemper};
use rand::random;

pub fn run_23() {
    let seed = random::<u32>();
    let mut original_prng = MersenneTwister::new();
    original_prng.seed(seed);

    let mut untempered = vec![];
    for _ in 0..624 {
        untempered.push(untemper(original_prng.rand()));
    }

    let mut clone_prng = MersenneTwister::new();
    clone_prng.set_state(untempered);

    for _ in 0..1000 {
        assert_eq!(clone_prng.rand(), original_prng.rand());
    }
    println!("OK");
}
