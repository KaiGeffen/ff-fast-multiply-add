#![allow(dead_code)]
#![allow(unused_variables)]

trait Add {
    fn add(&self, x: Self) -> u64;
}

trait Multiply {
    fn multiply(&self, x: Self) -> u64;
}

trait Reduce {
    fn reduce(&self) -> u64;
}

type ReductionFunc = fn(u64, u64) -> u64;

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

        // TODO Add a utility method that generalizes this
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
        // = (wsi mod modulo) * (byte_shift mod modulo)
        // where byte_shift is the number needed to shift word wsi to its correct
        // location in the resulting number (0000[wsi contents]0000)
        for i in 0..N {
            // TODO Generalize 32 based on how many words its broken into
            // TODO This breaks for i = 2, because that's beyond max u64, need to creatively handle that
            let byte_shift: u64 = 1 << (i * 32);
            ws[i] = (&self.reduce_f)(ws[i], self.modulo) * (&self.reduce_f)(byte_shift, self.modulo);
            println!("{}", ws[i]);
        }

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
// NOTE For testing purposes
impl Reduce for ModularArithmetic {
    fn reduce(&self) -> u64 {
        return (self.reduce_f)(self.value, self.modulo)
    }
}

/*
    REDUCTION FUNCTIONS
*/

// A naive implementation of modular arithmetic
fn naive_reduce(value: u64, modulo: u64) -> u64 {
    println!("value: {:?}", value);
    let q: u64 = value / modulo;
    println!("quotiant: {:?}", q);
    println!("reduction result: {:?}", value - q * modulo);
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

    // #[test]
    // fn naive_reduce_sanity() {
    //     let a = ModularArithmetic{value: 10, modulo: 8, reduce_f: Box::new(naive_reduce)};

    //     assert_eq!(a.add(b), 201);
    // }
}
