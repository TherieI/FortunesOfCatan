use super::matrix::Mat4;

pub struct Camera {
    pos: (f32, f32)
}

impl Camera {
    pub fn new(pos_x: f32, pos_y: f32) -> Self {
        Camera {
            pos: (pos_x, pos_y)
        }
    }

    pub fn view_matrix(&self) -> Mat4 {
        // Mat4::look_at()
        todo!()
    }
}
