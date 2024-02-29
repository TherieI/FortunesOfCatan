/// Vertex for a simple (x, y) position
#[derive(Clone, Copy)]
pub struct Vert2 {
    pos: [f32; 2],
}
implement_vertex!(Vert2, pos);

impl Vert2 {
    pub const fn new(x: f32, y: f32) -> Self {
        Vert2 { pos: [x, y] }
    }
}

impl From<(f32, f32)> for Vert2 {
    fn from(tuple: (f32, f32)) -> Self {
        Vert2 {
            pos: [tuple.0, tuple.1],
        }
    }
}

pub mod quad {
    use super::Vert2;

    pub const VERTICES: [Vert2; 4] = [
        Vert2::new(-1., -1.), // Bottom left
        Vert2::new(1., -1.),  // Bottom right
        Vert2::new(-1., 1.),  // Top left
        Vert2::new(1., 1.),   // Top right
    ];

    pub const INDICES: [u32; 6] = [0, 1, 2, 1, 2, 3];
}
