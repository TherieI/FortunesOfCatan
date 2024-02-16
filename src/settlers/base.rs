use crate::settlers::game::Scene;
use crate::settlers::shader::create_program;
use crate::settlers::Board;
use glium::backend::Facade;
use glium::{Frame, IndexBuffer, Program, Surface, VertexBuffer, Texture2d};
use winit::dpi::{PhysicalPosition, PhysicalSize};
use winit::event::{ElementState, KeyEvent, MouseButton};

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
    board: Board<5, 5>,
    hex_shader: Program,
    texture_map: Texture2d,
    camera: Camera,
    delta_time: DeltaTime,
    mouse: Mouse,
}

impl BaseGame {
    pub fn new<F>(facade: &F) -> Self
    where
        F: Sized + Facade,
    {
        let board: Board<5, 5> = Board::random_default();
        let image = image::load(std::io::Cursor::new(&include_bytes!("../../assets/hex/brick.png")),
                        image::ImageFormat::Png).unwrap().to_rgba8();
        let image_dimensions = image.dimensions();
        let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
        let texture = glium::texture::Texture2d::new(facade, image).unwrap();

        Self {
            board,
            hex_shader: create_program(facade, "glsl/hex.v.glsl", "glsl/hex.f.glsl", None)
                .expect("Shaders should be found."),
            texture_map: texture,
            camera: Camera::new(0., 0.),
            delta_time: DeltaTime::new(),
            mouse: Mouse::new(),
        }
    }

    fn mvp(&self) -> [[f32; 4]; 4] {
        let mut projection = Mat4::projection(16. / 8., 90., 1.0, -1.0);
        let mut transform = Mat4::identity();
        transform.translate(-0.4, -0.6, 0.).scale_uniformly(0.13);
        let view = self.camera.view_matrix();
        projection.multiply_by(&view).multiply_by(&transform);
        projection.to_array()
    }
}

impl Scene for BaseGame {
    // Called on mouse move
    fn mouse_move(
        &mut self,
        position: PhysicalPosition<f64>,
        window_dimensions: PhysicalSize<u32>,
    ) {
        let last_pos = self.mouse.last_pos();
        if self.mouse.left_pressed() {
            self.camera.move_to(|x, y| {
                (
                    x + (last_pos.x - position.x) as f32 * MOUSE_SPEED / window_dimensions.width as f32,
                    y - (last_pos.y - position.y) as f32 * MOUSE_SPEED / window_dimensions.height as f32,
                )
            })
        }
        self.mouse.update_cursor(position);
    }

    // Called on recieving mouse input
    fn mouse_input(&mut self, state: ElementState, button: MouseButton) {
        self.mouse.update_buttons(state, button);
    }
    // Called on recieving keyboard input
    fn keyboard_input(&mut self, event: KeyEvent) {
        use winit::keyboard::{KeyCode, PhysicalKey};
        let move_const = 100.;
        match &event.physical_key {
            PhysicalKey::Code(KeyCode::KeyW) => self
                .camera
                .move_to(|x, y| (x, y + move_const * self.delta_time.delta())),
            _ => (),
        }
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
        let (vertices, indices) = self.board.buffers();
        let vertex_buffer = VertexBuffer::new(facade, &vertices).unwrap();
        let index_buffer =
            IndexBuffer::new(facade, glium::index::PrimitiveType::TrianglesList, &indices).unwrap();

        frame
            .draw(
                &vertex_buffer,
                &index_buffer,
                &self.hex_shader,
                &uniform! { perspective: self.mvp(), tex_map: &self.texture_map},
                &Default::default(),
            )
            .unwrap();
        frame
    }
}
