#![allow(dead_code)]
#![allow(unused_variables)]
#![feature(test)]

use std::collections::HashMap;

trait Add {
    fn add(&self, x: Self) -> u64;
}

trait Multiply {
    fn multiply(&self, x: Self) -> u64;
}

trait Reduce {
    fn reduce(&self) -> u64;
}

struct ModularArithmetic {
    value: u64,
    modulo: u64,
    reduce_f: Box<dyn Fn(u64, u64) -> u64>,
}
impl Add for ModularArithmetic {
    fn add(&self, x: Self) -> u64 {
        // We only need at most 2 words, since size will at most double
        const N: usize = 2;

        let mut carry: u64 = 0;
        let mut ws: [u64; N + 1] = [0; N + 1];
        let mut us: [u64; N] = [0; N];
        let mut vs: [u64; N] = [0; N];

        // TODO Add a utility method that generalizes this for any number of words
        us[0] = (&self.value << 32) >> 32;
        vs[0] = (x.value << 32) >> 32;

        us[1] = &self.value >> 32;
        vs[1] = &self.value >> 32;

        // Loop over the words
        for i in 0..N {
            // TODO Generalize, also slightly wrong (b should be 1 more than MAX)
            let b: u64 = if i == 0 { 1 << 32 } else { u64::MAX };
            ws[i] = (us[i] + vs[i] + carry).rem_euclid(b);
            carry = (us[i] + vs[i] + carry) / b;
        }
        ws[N] = carry;

        // Perform the reduction
        // Because (A * B) mod N = (A mod N) * (B mod N)
        // = (wsi mod N) * (byte_shift mod N) mod N
        // where byte_shift is the number needed to shift word wsi to its correct
        // location in the resulting number (0000[wsi contents]0000)
        for i in 0..N {
            // TODO Generalize 32 based on how many words its broken into
            // TODO This breaks for i = 2, because that's beyond max u64, need to creatively handle that
            let byte_shift: u64 = 1 << (i * 32);
            let product = (&self.reduce_f)(ws[i], self.modulo) * (&self.reduce_f)(byte_shift, self.modulo);
            ws[i] = (&self.reduce_f)(product, self.modulo);
        }

        // TODO Need to either use bigint structure or reduce, since ws[2] is overflows u64
        return ws[0] + (ws[1] << 32);// + ws[2] << 64;
    }
}
impl Multiply for ModularArithmetic {
    fn multiply(&self, x: Self) -> u64 {
        // TODO Implement multiply

        // Determine how many words to break values into

        // Break each value into words

        // Run through the words doing multiplication
        // Hold onto the results



        // Reduce

        
        return 0;
    }
}

/*
    REDUCTION FUNCTIONS
*/

// A naive implementation of modular arithmetic
fn naive_reduce(value: u64, modulo: u64) -> u64 {
    println!("value: {:?}", value);
    let q: u64 = value / modulo;
    println!("quotient: {:?}", q);
    println!("reduction result: {:?}", value - q * modulo);
    return value - q * modulo;
}

// Implementation of Barrett reduction for modular arithmetic
fn barrett_reduce(value: u64, modulo: u64) -> u64 {
    // Approximate division by n (modulo) by multiplication by m/(2**k)
    // Choose m = floor(2**k/n) to avoid underflow error

    // Choose k st 2**k > n
    // Compute m = 2**k / n
    // Only do that division once, save the precomputed result, and use it in the future
    // Map from modulo to precomputed k-values for Barrett
    // TODO How to choose good m here? I'm choosing the smallest but that can't be right
    let precomputed_ms: HashMap<u64, (u64, u64)> = HashMap::from([
        // (500, 1), // n = 500, k = 9, m = 1
        (500, (20, 2097)), // n = 500, k = 20, m = 2097
        (6440809, (40, 170710)), // n = 6440809, k = 40, m = 170710
    ]);

    match precomputed_ms.get(&modulo) {
        Some((k, m)) => {
            let q: u64 = (value * m) >> k;
            println!("Quotient: {:?}", q);
            
            let mut result: u64 = value - q * modulo;
            println!("Reduction result: {:?}", result);

            while result >= modulo {
                result -= modulo;
            }

            return result;
        }
        None => {
            panic!("Modulo {} does not have a precomputed Barrett m!", modulo);
        }
    }   
}

// Implementation of Barrett reduction for modular arithmetic
fn barrett_reduce(value: u64, modulo: u64) -> u64 {
    // Approximate division by n (modulo) by multiplication by m/(2**k)
    // Choose m = floor(2**k/n) to avoid underflow error

    // Choose k st 2**k > n
    // Compute m = 2**k / n
    // Only do that division once, save the precomputed result, and use it in the future
    // Map from modulo to precomputed k-values for Barrett
    // TODO How to choose good m here? I'm choosing the smallest but that can't be right
    let precomputed_ms: HashMap<u64, (u64, u64)> = HashMap::from([
        // (500, 1), // n = 500, k = 9, m = 1
        (500, (20, 2097)), // n = 500, k = 20, m = 2097
        (6440809, (40, 170710)), // n = 6440809, k = 40, m = 170710
    ]);

    match precomputed_ms.get(&modulo) {
        Some((k, m)) => {
            let q: u64 = (value * m) >> k;
            println!("Quotient: {:?}", q);
            
            let mut result: u64 = value - q * modulo;
            println!("Reduction result: {:?}", result);

            while result >= modulo {
                result -= modulo;
            }

            return result;
        }
        None => {
            panic!("Modulo {} does not have a precomputed Barrett m!", modulo);
        }
    }   
}

// All 3 of these structs add and multiply in the same way,
// but after doing so they then reduce and either keep the reduced form
// or convert to classical form.
// The structure should instead be a modular arithmetic implementation
// that also takes a reduce function!
// The 3 cases to test are when a naive function is passed (Division)
// the barret reduction, and the Montgomery reduction

fn main() {

    println!("Hello, world!");
    let a = ModularArithmetic{value: 3, modulo: 1000, reduce_f: Box::new(naive_reduce)};
    let b = ModularArithmetic{value: 4, modulo: 1000, reduce_f: Box::new(naive_reduce)};

    println!("{}", a.add(b));
}

#[cfg(test)]
mod modular_arithmetic_works {
    use super::*;

    #[test]
    fn add_basic() {
        let a = ModularArithmetic{value: 3, modulo: 1000, reduce_f: Box::new(naive_reduce)};
        let b = ModularArithmetic{value: 4, modulo: 1000, reduce_f: Box::new(naive_reduce)};

        assert_eq!(a.add(b), 7);
    }

    #[test]
    fn add_large_numbers() {
        let a = ModularArithmetic{value: 27311837, modulo: u64::MAX, reduce_f: Box::new(naive_reduce)};
        let b = ModularArithmetic{value: 88689789, modulo: u64::MAX, reduce_f: Box::new(naive_reduce)};

        assert_eq!(a.add(b), 116001626);
    }

    #[test]
    fn add_works_when_exceeding_modulo_less_than_2x() {
        let a = ModularArithmetic{value: 301, modulo: 500, reduce_f: Box::new(naive_reduce)};
        let b = ModularArithmetic{value: 400, modulo: 500, reduce_f: Box::new(naive_reduce)};

        assert_eq!(a.add(b), 201);
    }

    #[test]
    fn add_works_when_a_times_b_equals_modulo() {
        let a = ModularArithmetic{value: 2, modulo: 4, reduce_f: Box::new(naive_reduce)};
        let b = ModularArithmetic{value: 2, modulo: 4, reduce_f: Box::new(naive_reduce)};

        assert_eq!(a.add(b), 0);
    }

    // #[test]
    // fn naive_reduce_sanity() {
    //     let a = ModularArithmetic{value: 10, modulo: 8, reduce_f: Box::new(naive_reduce)};

    //     assert_eq!(a.add(b), 201);
    // }

    #[test]
    fn barret() {
        let a = ModularArithmetic{value: 301, modulo: 500, reduce_f: Box::new(naive_reduce)};
        let b = ModularArithmetic{value: 400, modulo: 500, reduce_f: Box::new(naive_reduce)};

        assert_eq!(a.add(b), 201);
    }
}
#[cfg(test)]
mod barrett {
    use super::*;

    #[test]
    fn add_no_reduce() {
        let a = ModularArithmetic{value: 3, modulo: 500, reduce_f: Box::new(barrett_reduce)};
        let b = ModularArithmetic{value: 4, modulo: 500, reduce_f: Box::new(barrett_reduce)};

        assert_eq!(a.add(b), 7);
    }

    #[test]
    fn add_reduce() {
        let a = ModularArithmetic{value: 503, modulo: 500, reduce_f: Box::new(barrett_reduce)};
        let b = ModularArithmetic{value: 4, modulo: 500, reduce_f: Box::new(barrett_reduce)};

        assert_eq!(a.add(b), 7);
    }
}

// Benchmark tests to see the effect of not dividing

extern crate test;
use test::Bencher;
use rand::Rng;
#[cfg(test)]
mod benchmarks {
    use super::*;

    const REPS: usize = 30;
    const MOD: u64 = 6440809;

    #[bench]
    fn naive_addition(bencher: &mut Bencher) {
        let mut rng = rand::thread_rng();

        // Do additions (Including reduction) and track how long that takes
        bencher.iter(|| {
            for _ in 0..REPS {
                // Ideally, don't measure the time to instantiate
                let a = ModularArithmetic{value: rng.gen::<u64>(), modulo: MOD, reduce_f: Box::new(naive_reduce)};
                let b = ModularArithmetic{value: rng.gen::<u64>(), modulo: MOD, reduce_f: Box::new(naive_reduce)};

                a.add(b);
            }
        })
    }

    #[bench]
    fn barrett_addition(bencher: &mut Bencher) {
        let mut rng = rand::thread_rng();

        // Do additions (Including reduction) and track how long that takes
        bencher.iter(|| {
            for _ in 0..REPS {
                // Ideally, don't measure the time to instantiate
                let a = ModularArithmetic{value: rng.gen::<u64>(), modulo: MOD, reduce_f: Box::new(barrett_reduce)};
                let b = ModularArithmetic{value: rng.gen::<u64>(), modulo: MOD, reduce_f: Box::new(barrett_reduce)};
                a.add(b);
            }
        })
    }

    #[bench]
    fn naive_add_small_numbers(bencher: &mut Bencher) {
        // Do additions (Including reduction) and track how long that takes
        bencher.iter(|| {
            for _ in 0..REPS {
                // Ideally, don't measure the time to instantiate
                let a = ModularArithmetic{value: 3, modulo: MOD, reduce_f: Box::new(naive_reduce)};
                let b = ModularArithmetic{value: 5, modulo: MOD, reduce_f: Box::new(naive_reduce)};
                a.add(b);
            }
        })
    }
    #[bench]
    fn barrett_add_small_numbers(bencher: &mut Bencher) {
        // Do additions (Including reduction) and track how long that takes
        bencher.iter(|| {
            for _ in 0..REPS {
                // Ideally, don't measure the time to instantiate
                let a = ModularArithmetic{value: 3, modulo: MOD, reduce_f: Box::new(barrett_reduce)};
                let b = ModularArithmetic{value: 5, modulo: MOD, reduce_f: Box::new(barrett_reduce)};
                a.add(b);
            }
        })
    }



}
