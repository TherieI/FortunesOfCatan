use crate::settlers::game::Scene;
use crate::settlers::shader::create_program;
use crate::settlers::Board;
use glium::backend::Facade;
use glium::{Frame, IndexBuffer, Program, Surface, VertexBuffer};
use winit::dpi::PhysicalPosition;
use winit::event::{ElementState, KeyEvent, MouseButton};

pub struct BaseGame {
    board: Board<5, 5>,
    hex_shader: Program,
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
        }
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

    fn draw<F>(&self, facade: &F, mut frame: Frame) -> Frame
    where
        F: ?Sized + Facade,
    {
        let (vertices, indices) = self.board.buffers();
        let vertex_buffer = VertexBuffer::new(facade, &vertices).unwrap();
        let index_buffer =
            IndexBuffer::new(facade, glium::index::PrimitiveType::TrianglesList, &indices).unwrap();

        let size = (0.03, 0.03);
        let aspect = 16. / 8.;
        let fov = 90.;
        let distance = 1.0;
        let perspective_matrix: [[f32; 4]; 4] = [
            [size.0 / aspect, 0.0, 0.0, 0.0],
            [0.0, size.1, 0.0, 0.0],
            [0.0, 0.0, 1.0 / fov, -1.0 / fov],
            [0.0, 0.0, 1.0 / distance, 1.0],
        ];
        frame
            .draw(
                &vertex_buffer,
                &index_buffer,
                &self.hex_shader,
                &uniform! { perspective: perspective_matrix},
                &Default::default(),
            )
            .unwrap();
        frame
    }
}
