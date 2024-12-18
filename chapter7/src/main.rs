
// the 'pub mod garden;' line tells the compiler to include the code it finds
// in src/garden.rs or src/garden/mod.rs
pub mod garden;

use crate::garden::vegetables::Asparagus;

// https://doc.rust-lang.org/book/ch07-02-defining-modules-to-control-scope-and-privacy.html
fn main() {
    println!("Chapter 7 Managing Projects");

    let plant = Asparagus;

    println!("I'm growing {plant:?}");
}
