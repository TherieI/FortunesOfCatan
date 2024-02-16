use crate::settlers::game::Scene;
use crate::settlers::shader::create_program;
use crate::settlers::Board;
use glium::backend::Facade;
use glium::{Frame, IndexBuffer, Program, Surface, VertexBuffer};
use winit::dpi::PhysicalPosition;
use winit::event::{ElementState, KeyEvent, MouseButton};

use super::camera::Camera;
use super::game::DeltaTime;
use super::matrix::Mat4;

pub struct BaseGame {
    board: Board<5, 5>,
    hex_shader: Program,
    camera: Camera,
    delta_time: DeltaTime
}

impl BaseGame {
    pub fn new<F>(facade: &F) -> Self
    where
        F: Sized + Facade,
    {
        let board: Board<5, 5> = Board::random_default();
        Self {
            board,
            hex_shader: create_program(facade, "glsl/hex.v.glsl", "glsl/hex.f.glsl", None)
                .expect("Shaders should be found."),
            camera: Camera::new(0., 0.),
            delta_time: DeltaTime::new()
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
    fn mouse_move(&mut self, position: PhysicalPosition<f64>) {
    }
    // Called on recieving mouse input
    fn mouse_input(&mut self, state: ElementState, button: MouseButton) {}
    // Called on recieving keyboard input
    fn keyboard_input(&mut self, event: KeyEvent) {
        use winit::keyboard::{PhysicalKey, KeyCode};
        let move_const = 100.;
        match &event.physical_key {
            PhysicalKey::Code(KeyCode::KeyW) => self.camera.move_to(|x, y| {
                (x, y + move_const * self.delta_time.delta())
            }),
            _ => ()
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
                &uniform! { perspective: self.mvp()},
                &Default::default(),
            )
            .unwrap();
        frame
    }
}
