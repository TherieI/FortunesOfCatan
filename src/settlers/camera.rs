use super::matrix::{Mat4, Vec3};
// Found this which may be useful after I made the camera class: https://github.com/glium/glium/blob/master/examples/support/camera.rs

pub struct Camera {
    pos: Vec3,
}

impl Camera {
    pub fn new(pos_x: f32, pos_y: f32) -> Self {
        Camera {
            pos: (pos_x, pos_y, 0.0).into(),
        }
    }

    pub fn view_matrix(&self) -> Mat4 {
        Mat4::look_at(
            self.pos,
            self.pos - Vec3::new(0., 0., 1.),
            Vec3::new(0., 1., 0.),
        )
    }

    /// Move camera to a new position. Closure parameters are the camera's current position.
    pub fn move_to(&mut self, new_pos: impl Fn(f32, f32, f32) -> (f32, f32, f32)) {
        let pos = self.pos.as_tuple();
        self.pos = new_pos(pos.0, pos.1, pos.2).into();
        // println!("Updated camera position: {:?}", self.pos);
    }

    pub fn position(&self) -> &Vec3 {
        &self.pos
    }
}
