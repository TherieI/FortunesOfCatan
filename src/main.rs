#[macro_use]
extern crate glium;
extern crate rand;

mod settlers;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    settlers::game::main()
}
