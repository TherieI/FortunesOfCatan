#[macro_use]
extern crate glium;
extern crate rand;
mod settlers;

pub mod settings {
    use winit::dpi::PhysicalSize;
    pub const WINDOW_DEFAULT_SIZE: PhysicalSize<u32> = PhysicalSize {
        width: 1080,
        height: 720,
    };
    pub const DEBUG: bool = true;
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    settlers::game::main(settings::DEBUG)
}
