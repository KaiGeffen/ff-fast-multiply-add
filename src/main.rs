#![allow(dead_code)]
#![allow(unused_variables)]

// A number in modular arithmetic
struct Number {
    value: u64,
    modulo: u64
}

trait Reduce {
    fn reduce(&self) -> u64;
}

trait Add {
    fn add(&self, x: Self) -> u64;
}

trait Multiply {
    fn multiply(&self, x: Self) -> u64;
}

// Member of the modular arithmetic field
trait ModularArithmetic: Reduce + Add + Multiply {}


// A naive implementation of modular arithmetic
struct Naive {
    value: u64,
    modulo: u64
}
impl Reduce for Naive {
    fn reduce(&self) -> u64 {
        let q: u64 = &self.value / &self.modulo;
        return &self.value - q * &self.modulo;
    }
}
impl Add for Naive {
    fn add(&self, x: Self) -> u64 {
        // We only need at most 2 words, since size will at most double
        const N: usize = 2;

        let mut carry: u64 = 0;
        let mut ws: [u64; N] = [0; N + 1];
        let mut us: [u64; N] = [0; N];
        let mut vs: [u64; N] = [0; N];

        us[0] = (&self.value << 32) >> 32;
        vs[0] = (x.value << 32) >> 32;

        us[1] = &self.value >> 32;
        vs[1] = &self.value >> 32;

        // Loop over the words
        for i in 0..N {
            ws[i] = (us[i] + vs[i] + carry).rem_euclid(self.modulo);
            carry = (us[i] + vs[i] + carry) / self.modulo
        }
        ws[N] = carry;

        return ws[0];
    }
}
impl Multiply for Naive {
    fn multiply(&self, x: Self) -> u64 {
        return self.value;
    }
}
impl ModularArithmetic for Naive {}







// Barret implementation of modular arithmetic
struct Barret {
    value: u64,
    modulo: u64
}
impl Reduce for Barret {
    fn reduce(&self) -> u64 {
        let q: u64 = &self.value / &self.modulo;
        return &self.value - q * &self.modulo;
    }
}
impl Add for Barret {
    fn add(&self, x: Self) -> u64 {
        return self.value;
    }
}
impl Multiply for Barret {
    fn multiply(&self, x: Self) -> u64 {
        return self.value;
    }
}
impl ModularArithmetic for Barret {}

// Montgomery implementation of modular arithmetic
// [x] = (xR) mod N
struct Montgomery {

}

fn main() {

    println!("Hello, world!");
    let a = Naive{value: 3, modulo: 7};
    let b = Naive{value: 4, modulo: 7};

    println!("{}", a.add(b));
}
