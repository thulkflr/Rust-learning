use crate::garden::vegetables::Asparagus;
//or use Asparagus;
// this is the root crate:
pub mod garden;
// The pub mod garden; line tells the compiler to include the code it finds in src/garden.rs, which is:
// Filename: src/garden.rs

fn main() {

    let plant = Asparagus {};
    println!("Im {plant:?}");

}
