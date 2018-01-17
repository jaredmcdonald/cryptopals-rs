// use core::num::Wrapping;

const W: usize = 32;
const N: usize = 624;
// const M: usize = 397;
// const R: usize = 31;
// const A: usize = 0x9908b0df;
// const U: usize = 11;
const D: u32 = 0xffffffff;
// const S: usize = 7;
// const B: usize = 0x9d2c5680;
// const T: usize = 15;
// const C: usize = 0xefc60000;
// const L: usize = 18;
const F: u32 = 1812433253;

// https://en.wikipedia.org/wiki/Mersenne_Twister
// https://github.com/nryoung/algorithms/blob/master/algorithms/random/mersenne_twister.py
struct MersenneTwister {
    state: Vec<u32>,
    index: usize,
}

impl MersenneTwister {
    pub fn new() -> MersenneTwister {
        MersenneTwister {
            state: vec![],
            index: 0,
        }
    }

    pub fn seed(&mut self, seed: u32) {
        self.index = 0;
        self.state = vec![seed];

        for i in 1..N {
            let prev = self.state[i - 1];
            let mut n = F.wrapping_mul(prev ^ (prev >> (W - 2)) + i as u32);
            n &= D; // can't find this in the wikipedia pseudocode but it's in the python impl :/
            self.state.push(n);
        }
    }
}

pub fn run_21() {
    let mut rng = MersenneTwister::new();
    rng.seed(100);
}
