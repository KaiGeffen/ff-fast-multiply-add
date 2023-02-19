#![allow(dead_code)]

// A number in modular arithmetic
struct Number {
    value: u64,
    modulo: u64
}

trait Reduce {
    fn reduce(&self) -> u64;
}

// trait Add {
//     fn add(&self, );
// }

// trait Multiply {
//     fn multiply(&self);
// }


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

// Barret implementation of modular arithmetic
struct Barret {
    value: u64,
    modulo: u64
}
impl Reduce for Barret {
    fn reduce(&self) -> u64 {
        return self.value;
    }
}

// Montgomery implementation of modular arithmetic
// [x] = (xR) mod N
struct Montgomery {

}

fn main() {

    println!("Hello, world!");
    let foo = Naive{value: 133, modulo: 40};
    println!("{}", foo.reduce());
}
