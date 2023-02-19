
#![feature(test)]
#![allow(unused_imports)]
extern crate test;
use test::Bencher;
use test::black_box;

// Fib code taken from https://docs.rs/num-bigint/latest/num_bigint/
use num_bigint::BigUint;
use num_traits::{Zero, One};
use std::mem::replace;

// Calculate large fibonacci numbers.
pub fn fib(n: usize) -> BigUint {
    let mut f0: BigUint = Zero::zero();
    let mut f1: BigUint = One::one();
    for _ in 0..n {
        let f2 = f0 + &f1;
        // This is a low cost way of swapping f0 with f1 and f1 with f2.
        f0 = replace(&mut f1, f2);
    }
    f0
}


#[cfg(test)]
mod tests {
    use super::*;
    // use core::u64::MAX;

    // #[test]
    // fn const_time_multiplication() {
    //     let result = MAX + MAX - MAX;
    //     assert_eq!(result, 4);
    // }

    // #[test]
    // fn const_time_multiplication() {
    //     let result = MAX + MAX - MAX;
    //     assert_eq!(result, 4);
    // }

    #[bench]
    fn fib_builtin(b: &mut Bencher) {
        b.iter(|| {
            test::black_box(fib(20));
        })
    }
}
// 27311837

/*
Aim is to accomplish these things in this hack:

Can add bigints (integers of arbitrary size) in a time-efficient way
Multiply ""

Addition operation is done in constant time so as to avoid timing-attack
Multiplication ""

From a description of issues with bigint, it claims that their standard addition/multiplication operations take 3 times as long.
I doubt it's a constant multiplier, likely it goes from O(N) to O(N^2) or something similar

I'm now able to run benchmark tests

*/

