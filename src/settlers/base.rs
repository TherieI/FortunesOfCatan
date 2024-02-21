use std::time::Instant;

use crate::settlers::game::Scene;
use crate::settlers::shader::create_program;
use crate::settlers::Board;
use glium::backend::Facade;
use glium::index::NoIndices;
use glium::{Frame, IndexBuffer, Program, Surface, Texture2d, VertexBuffer};
use winit::dpi::{PhysicalPosition, PhysicalSize};
use winit::event::{ElementState, KeyEvent, MouseButton, MouseScrollDelta, TouchPhase};

use super::camera::Camera;
use super::game::DeltaTime;
use super::matrix::Mat4;

const MOUSE_SPEED: f32 = 10.;

pub struct Mouse {
    left_click_pressed: bool,
    last_mouse_pos: PhysicalPosition<f64>,
}

impl Mouse {
    pub fn new() -> Self {
        Mouse {
            left_click_pressed: false,
            last_mouse_pos: PhysicalPosition::new(0., 0.),
        }
    }

    pub fn update_buttons(&mut self, state: ElementState, button: MouseButton) {
        match state {
            ElementState::Pressed => {
                if button == MouseButton::Left {
                    self.left_click_pressed = true;
                }
            }
            ElementState::Released => {
                if button == MouseButton::Left {
                    self.left_click_pressed = false;
                }
            }
        }
    }

    pub fn update_cursor(&mut self, normal_pos: PhysicalPosition<f64>) {
        self.last_mouse_pos = normal_pos;
    }

    pub fn left_pressed(&self) -> bool {
        self.left_click_pressed
    }

    pub fn last_pos(&self) -> PhysicalPosition<f64> {
        self.last_mouse_pos
    }
}

pub struct BaseGame {
    window_dim: PhysicalSize<u32>,
    // Keep track of the program time
    time: Instant,
    board: Board<5, 5>,
    hex_shader: Program,
    // Texture for the hex's
    texture_map: Texture2d,
    camera: Camera,
    delta_time: DeltaTime,
    mouse: Mouse,
    scale: f32,
}

impl BaseGame {
    pub fn new<F>(facade: &F) -> Self
    where
        F: Sized + Facade,
    {
        let board: Board<5, 5> = Board::random_default();
        // Generate texture for our hex's
        let image = image::load(
            std::io::Cursor::new(&include_bytes!("../../assets/hex/resource_tilemap_v1.png")),
            image::ImageFormat::Png,
        )
        .unwrap()
        .to_rgba8();
        let image_dimensions = image.dimensions();
        let image =
            glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
        let texture = glium::texture::Texture2d::new(facade, image).unwrap();

        use crate::settings::WINDOW_DEFAULT_SIZE;
        Self {
            window_dim: WINDOW_DEFAULT_SIZE,
            time: Instant::now(),
            board,
            hex_shader: create_program(facade, "glsl/hex.v.glsl", "glsl/hex.f.glsl", Some("glsl/hex.g.glsl"))
                .expect("Shaders should be found."),
            texture_map: texture,
            camera: Camera::new(0., 0.),
            delta_time: DeltaTime::new(),
            mouse: Mouse::new(),
            scale: 0.13,
        }
    }

    fn mvp(&self) -> [[f32; 4]; 4] {
        let mut projection = Mat4::projection(
            self.window_dim.width as f32 / self.window_dim.height as f32,
            90.,
            1.0,
            -1.0,
        );
        let mut transform = Mat4::identity();
        transform
            .translate(-0.4, -0.6, 0.)
            .scale_uniformly(self.scale);
        let view = self.camera.view_matrix();
        projection.multiply_by(&view).multiply_by(&transform);
        projection.to_array()
    }
}

impl Scene for BaseGame {
    // Called on mouse move
    fn mouse_move(&mut self, position: PhysicalPosition<f64>) {
        let last_pos = self.mouse.last_pos();
        if self.mouse.left_pressed() {
            self.camera.move_to(|x, y, z| {
                (
                    x + (last_pos.x - position.x) as f32 * MOUSE_SPEED
                        / self.window_dim.width as f32,
                    y - (last_pos.y - position.y) as f32 * MOUSE_SPEED
                        / self.window_dim.height as f32,
                    z,
                )
            })
        }
        self.mouse.update_cursor(position);
    }

    // Called on recieving mouse input
    fn mouse_input(&mut self, state: ElementState, button: MouseButton) {
        self.mouse.update_buttons(state, button);
    }

    fn scroll_input(&mut self, delta: MouseScrollDelta, _phase: TouchPhase) {
        match delta {
            MouseScrollDelta::LineDelta(_, scroll) => self.scale += scroll / 200.,
            _ => (),
        }
    }

    // Called on recieving keyboard input
    fn keyboard_input(&mut self, event: KeyEvent) {
        use winit::keyboard::{KeyCode, PhysicalKey};
        let move_const = 100.;
        match &event.physical_key {
            PhysicalKey::Code(KeyCode::KeyW) => self
                .camera
                .move_to(|x, y, z| (x, y + move_const * self.delta_time.delta(), z)),
            _ => (),
        }
    }

    fn window_size(&mut self, new_size: PhysicalSize<u32>) {
        self.window_dim = new_size;
    }

    // Called every time before draw
    fn update(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.delta_time.update();
        Ok(())
    }

    fn draw<F>(&self, facade: &F, mut frame: Frame) -> Frame
    where
        F: ?Sized + Facade,
    {
        // ============== Background ===============


        // ============== Hex tiles ================
        let vertices  = self.board.buffers();
        let vertex_buffer = VertexBuffer::new(facade, &vertices).unwrap();
        let index_buffer = NoIndices(glium::index::PrimitiveType::Points);
        use glium::uniforms::{
            MagnifySamplerFilter, MinifySamplerFilter, Sampler,
        };
        
        frame
            .draw(
                &vertex_buffer,
                &index_buffer,
                &self.hex_shader,
                &uniform! { u_mvp: self.mvp(),
                    u_resolution: (self.window_dim.width, self.window_dim.height),
                    u_time: self.time.elapsed().as_secs_f32(),
                    tex_map: Sampler::new(&self.texture_map)
                        .minify_filter(MinifySamplerFilter::Nearest)
                        .magnify_filter(MagnifySamplerFilter::Nearest)
                },
                &Default::default(),
            )
            .unwrap();
        frame

        // =============== Settlements / Cities / Roads ==================
        
    }
}
