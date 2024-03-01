use crate::settlers::expansions::base::BaseGame;
use glium::backend::Facade;
use glium::{Frame, Surface};
use std::time::Instant;
use winit::dpi::{PhysicalPosition, PhysicalSize};
use winit::event::{ElementState, KeyEvent, MouseButton, MouseScrollDelta, TouchPhase};

pub struct DeltaTime {
    last: Instant,
}

impl DeltaTime {
    pub fn new() -> Self {
        DeltaTime {
            last: Instant::now(),
        }
    }

    pub fn update(&mut self) {
        self.last = Instant::now();
    }

    pub fn delta(&self) -> f32 {
        self.last.elapsed().as_secs_f32()
    }
}

pub trait Scene {
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

#[allow(dead_code)]
pub enum Expansion {
    Base,
    Seafarers,
    CitiesAndKnights,
}

#[allow(dead_code)]
pub struct Settlers {
    scene: Expansion,
}

impl Settlers {
    pub fn new() -> Self {
        Self {
            scene: Expansion::Base,
        }
    }

    pub fn run(&mut self, _imgui_debug: bool) -> Result<(), Box<dyn std::error::Error>> {
        // =======================================
        let event_loop = winit::event_loop::EventLoopBuilder::new()
            .build()
            .expect("event loop building");
        let (window, display) = glium::backend::glutin::SimpleWindowBuilder::new()
            .with_title("Fortune of Catan")
            .with_inner_size(
                crate::settings::WINDOW_DEFAULT_SIZE.width,
                crate::settings::WINDOW_DEFAULT_SIZE.height,
            )
            .build(&event_loop);

        // ================ IMGUI ========================

        let mut base_game = BaseGame::new(&display);
        use winit::event::{Event, WindowEvent};
        // Game loop
        let _ = event_loop.run(move |event, window_target| {
            match event {
                winit::event::Event::WindowEvent { event, .. } => match event {
                    WindowEvent::CloseRequested => window_target.exit(),
                    WindowEvent::Resized(window_size) => {
                        display.resize(window_size.into());
                        base_game.window_size(window.inner_size());
                    }
                    // Input events
                    WindowEvent::CursorMoved { position, .. } => {
                        base_game.mouse_move(position);
                    }

                    WindowEvent::MouseInput { state, button, .. } => {
                        base_game.mouse_input(state, button);
                    }
                    WindowEvent::KeyboardInput { event, .. } => {
                        base_game.keyboard_input(event);
                    }

                    WindowEvent::MouseWheel { delta, phase, .. } => {
                        base_game.scroll_input(delta, phase);
                    }

                    WindowEvent::RedrawRequested => {
                        // Update any logic
                        base_game.update().unwrap();
                        // Create frame canvas
                        let mut target = display.draw();
                        target.clear_color(0.0, 0.5, 0.6, 1.0);
                        base_game.draw(&display, target).finish().unwrap();
                    }
                    _ => (),
                },
                Event::AboutToWait => {
                    window.request_redraw();
                }
                _ => (),
            };
        });
        Ok(())
    }
}

pub fn main(imgui_debug: bool) -> Result<(), Box<dyn std::error::Error>> {
    let mut game = Settlers::new();
    game.run(imgui_debug)
}
