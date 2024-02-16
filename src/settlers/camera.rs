use super::matrix::Mat4;
// Found this which may be useful after I made the camera class: https://github.com/glium/glium/blob/master/examples/support/camera.rs

pub struct Camera {
    pos: (f32, f32),
}

impl Camera {
    pub fn new(pos_x: f32, pos_y: f32) -> Self {
        Camera {
            pos: (pos_x, pos_y),
        }
    }

    pub fn view_matrix(&self) -> Mat4 {
        Mat4::look_at((self.pos.0, self.pos.1, 0.), (self.pos.0, self.pos.1, -1.), (0., 1., 0.))
    }

    /// Move camera to a new position. Closure parameters are the camera's current position.
    pub fn move_to(&mut self, new_pos: impl Fn(f32, f32) -> (f32, f32)) {
        self.pos = new_pos(self.pos.0, self.pos.1);
    }

    pub fn position(&self) -> (f32, f32) {
        self.pos
    }
}
