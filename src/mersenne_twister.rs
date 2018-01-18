const W: usize = 32;
const N: usize = 624;
const M: usize = 397;
// const R: usize = 31;
const A: u32 = 0x9908b0df;
const U: u32 = 11;
const S: u32 = 7;
const B: u32 = 0x9d2c5680;
const T: u32 = 15;
const C: u32 = 0xefc60000;
const L: u32 = 18;
const F: u32 = 1812433253;

// https://en.wikipedia.org/wiki/Mersenne_Twister
// https://github.com/nryoung/algorithms/blob/master/algorithms/random/mersenne_twister.py
pub struct MersenneTwister {
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
            let n = F.wrapping_mul(prev ^ (prev >> (W - 2))).wrapping_add(i as u32);
            self.state.push(n);
        }
    }

    pub fn rand(&mut self) -> u32 {
        if self.index == 0 {
            self.generate();
        }
        let mut y = self.state[self.index];
        y ^= y >> U;
        y ^= (y << S) & B;
        y ^= (y << T) & C;
        y ^= y >> L;

        self.index = (self.index + 1) % N;
        y
    }

    // useful for e.g. 3.23 where we need to splice in a faked state array
    pub fn set_state(&mut self, state: Vec<u32>) {
        self.state = state;
    }

    fn generate(&mut self) {
        for i in 0..N {
            let mut n = self.state[i] & 0x80000000;    // lower R bits (i think?)
            n += self.state[(i + 1) % N] & 0x7fffffff; // upper W-2 bits (again... idk)
            self.state[i] = self.state[(i + M) % N] ^ (n >> 1);
            if n % 2 != 0 {
                self.state[i] ^= A;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_same_seed() {
        let seed = 123456;
        let mut prng1 = MersenneTwister::new();
        let mut prng2 = MersenneTwister::new();
        prng1.seed(seed);
        prng2.seed(seed);
        for _ in 0..10 {
            assert_eq!(prng1.rand(), prng2.rand());
        }
    }

    #[test]
    fn test_different_seed() {
        let mut prng1 = MersenneTwister::new();
        let mut prng2 = MersenneTwister::new();
        prng1.seed(1234);
        prng2.seed(5678);
        for _ in 0..10 {
            assert_ne!(prng1.rand(), prng2.rand());
        }
    }
}
