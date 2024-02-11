#[macro_use]
extern crate glium;

mod settlers;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    settlers::game::main()
}
