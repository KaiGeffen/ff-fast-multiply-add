#![allow(dead_code)]
#![allow(unused_variables)]

trait Add {
    fn add(&self, x: Self) -> u64;
}

trait Multiply {
    fn multiply(&self, x: Self) -> u64;
}

type ReductionFunc = fn(u64, u64) -> u64;

#[derive(Default)]
struct ModularArithmetic {
    value: u64,
    modulo: u64,
    // reduce_f: ReductionFunc
}
impl Add for ModularArithmetic {
    fn add(&self, x: Self) -> u64 {
        // We only need at most 2 words, since size will at most double
        const N: usize = 2;

        let mut carry: u64 = 0;
        let mut ws: [u64; N + 1] = [0; N + 1];
        let mut us: [u64; N] = [0; N];
        let mut vs: [u64; N] = [0; N];

        // TODO Add a utility method that generalizes this
        us[0] = (&self.value << 32) >> 32;
        vs[0] = (x.value << 32) >> 32;

        us[1] = &self.value >> 32;
        vs[1] = &self.value >> 32;

        // Loop over the words
        for i in 0..N {
            ws[i] = (us[i] + vs[i] + carry).rem_euclid(self.modulo);
            carry = (us[i] + vs[i] + carry) / self.modulo;
        }
        ws[N] = carry;

        // TODO Need to either use bigint structure or reduce, since ws[2] is overflows u64
        return ws[0] + (ws[1] << 32);// + ws[2] << 64;
    }
}
impl Multiply for ModularArithmetic {
    fn multiply(&self, x: Self) -> u64 {
        let q: u64 = &self.value / &self.modulo;
        return &self.value - q * &self.modulo;
    }
}

// A naive implementation of modular arithmetic
fn naive_reduce(value: u64, modulo: u64) -> u64 {
    let q: u64 = value / modulo;
    return value - q * modulo;
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
    let a = ModularArithmetic{value: 3, modulo: 1000};
    let b = ModularArithmetic{value: 4, modulo: 1000};

    println!("{}", a.add(b));
}

#[cfg(test)]
mod modular_arithmetic_works {
    use super::*;

    #[test]
    fn add_basic() {
        let a = ModularArithmetic{value: 3, modulo: 1000};
        let b = ModularArithmetic{value: 4, modulo: 1000};

        assert_eq!(a.add(b), 7);
    }

    #[test]
    fn add_large_numbers() {
        let a = ModularArithmetic{value: 27311837, modulo: u64::MAX};
        let b = ModularArithmetic{value: 88689789, modulo: u64::MAX};

        assert_eq!(a.add(b), 116001626);
    }
}
