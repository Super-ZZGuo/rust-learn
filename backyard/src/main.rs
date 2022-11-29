use crate::garden::vegetables::Asparagus;
pub mod garden;

fn main() {
    let plant = Asparagus {};
    println!("It is a struct {:?}",plant);
}
