use crate::settlers::game::Scene;
use crate::settlers::Board;
use glium::backend::Facade;
use glium::Frame;
use winit::dpi::PhysicalPosition;
use winit::event::{ElementState, KeyEvent, MouseButton};

pub struct BaseGame {
    board: Board,
}

impl BaseGame {
    pub fn new() -> Self {
        let board = Board::new().randomize();
        Self {}
    }
}

impl Scene for BaseGame {
    // Called on mouse move
    fn mouse_move(&mut self, position: PhysicalPosition<f64>) {}
    // Called on recieving mouse input
    fn mouse_input(&mut self, state: ElementState, button: MouseButton) {}
    // Called on recieving keyboard input
    fn keyboard_input(&mut self, event: KeyEvent) {}

    // Called every time before draw
    fn update(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }

    fn draw<F>(&self, facade: &F, frame: Frame) -> Frame
    where
        F: ?Sized + Facade,
    {
        frame
    }
}
