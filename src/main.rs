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
    fn add(&self, x: Box<dyn Add>) -> u64;
}

trait Multiply {
    fn multiply(&self, x: Box<dyn Add>) -> u64;
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
    fn add(&self, x: Box<dyn Add>) -> u64 {
        return self.value;
    }
}
impl Multiply for Naive {
    fn multiply(&self, x: Box<dyn Add>) -> u64 {
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
    fn add(&self, x: Box<dyn Add>) -> u64 {
        return self.value;
    }
}
impl Multiply for Barret {
    fn multiply(&self, x: Box<dyn Add>) -> u64 {
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
    let foo = Naive{value: 133, modulo: 40};
    println!("{}", foo.reduce());
}
