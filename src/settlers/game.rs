use glium::backend::{glutin::Display, Facade};
use glium::glutin::surface::WindowSurface;
use glium::{Frame, Surface};
use winit::dpi::PhysicalPosition;
use winit::event::{ElementState, KeyEvent, MouseButton};
use winit::event_loop::EventLoop;
use winit::window::Window;

use crate::settlers::base::BaseGame;

pub trait Scene {
    // Called on mouse move
    fn mouse_move(&mut self, position: PhysicalPosition<f64>);
    // Called on recieving mouse input
    fn mouse_input(&mut self, state: ElementState, button: MouseButton);
    // Called on recieving keyboard input
    fn keyboard_input(&mut self, event: KeyEvent);

    // Called every time before draw
    fn update(&mut self) -> Result<(), Box<dyn std::error::Error>>;
    // Draw to the screen with frame
    fn draw<F>(&self, facade: &F, frame: Frame) -> Frame
    where
        F: ?Sized + Facade;
}

pub enum Expansion {
    Base,
    Seafarers,
    CitiesAndKnights,
}

pub struct Settlers {
    scene: Expansion,
}

impl Settlers {
    pub fn new() -> Self {
        Self {
            scene: Expansion::Base,
        }
    }

    pub fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // =======================================
        let event_loop = winit::event_loop::EventLoopBuilder::new()
            .build()
            .expect("event loop building");
        let (window, display) = glium::backend::glutin::SimpleWindowBuilder::new()
            .with_title("Gamblers of Catan")
            .build(&event_loop);

        let mut base_game = BaseGame::new(&display);
        // Game loop
        let _ = event_loop.run(move |event, window_target| {
            match event {
                winit::event::Event::WindowEvent { event, .. } => match event {
                    winit::event::WindowEvent::CloseRequested => window_target.exit(),
                    winit::event::WindowEvent::Resized(window_size) => {
                        display.resize(window_size.into());
                    }
                    // Input events
                    winit::event::WindowEvent::CursorMoved { position, .. } => {
                        base_game.mouse_move(position)
                    }
                    winit::event::WindowEvent::MouseInput { state, button, .. } => {
                        base_game.mouse_input(state, button)
                    }
                    winit::event::WindowEvent::KeyboardInput { event, .. } => {
                        base_game.keyboard_input(event)
                    }

                    winit::event::WindowEvent::RedrawRequested => {
                        base_game.update().unwrap();
                        let mut target = display.draw();
                        target.clear_color(0.0, 0.5, 0.6, 1.0);
                        base_game.draw(&display, target).finish().unwrap();
                    }
                    _ => (),
                },
                winit::event::Event::AboutToWait => {
                    window.request_redraw();
                }
                _ => (),
            };
        });
        Ok(())
    }
}

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut game = Settlers::new();
    game.run()
}
