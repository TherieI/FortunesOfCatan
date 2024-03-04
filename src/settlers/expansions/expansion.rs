use glium::backend::Facade;
use glium::Frame;
use winit::dpi::{PhysicalPosition, PhysicalSize};
use winit::event::{ElementState, KeyEvent, MouseButton, MouseScrollDelta, TouchPhase};

pub trait Expansion {
    // Called on mouse move
    fn mouse_move(&mut self, position: PhysicalPosition<f64>);
    // Called on recieving mouse input
    fn mouse_input(&mut self, state: ElementState, button: MouseButton);
    // Called on recieving keyboard input
    fn keyboard_input(&mut self, event: KeyEvent);
    // Called on recieving mouse scroll input
    fn scroll_input(&mut self, delta: MouseScrollDelta, phase: TouchPhase);
    // Called on recieving window size update
    fn window_size(&mut self, new_size: PhysicalSize<u32>);
    // Called every time before draw
    fn update(&mut self) -> Result<(), Box<dyn std::error::Error>>;
    // Draw to the screen with frame
    fn draw<F>(&self, facade: &F, frame: Frame) -> Frame
    where
        F: ?Sized + Facade;
}