#![allow(dead_code)]

// A number in modular arithmetic
struct Number {
    value: u64,
    modulo: u64
}

trait Reduce {
    fn reduce(&self);
}

// trait Add {
//     fn add(&self, );
// }

// trait Multiply {
//     fn multiply(&self);
// }


// A naive implementation of modular arithmetic
struct Naive ();
impl Reduce for Naive {
    fn reduce(&self) {
        println!("Hello");
    }
}

// Barret implementation of modular arithmetic
struct Barret {
}

// Montgomery implementation of modular arithmetic
// [x] = (xR) mod N
struct Montgomery {

}

fn main() {

    println!("Hello, world!");
    let foo = Naive();
    foo.reduce();
}
