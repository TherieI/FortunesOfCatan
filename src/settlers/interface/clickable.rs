use winit::dpi::PhysicalPosition;

/// A region of the screen. For example, AABB bounding.
pub trait BoundingBox {
    fn within(&self, mouse_pos: PhysicalPosition<f64>) -> bool;
}

/// Types that are clickable should implement this trait
pub trait Clickable {
    /// Click output is what you recieve from a mouse click
    type ClickOutput;
    /// Is the mouse position within a certain region of the screen based on a bounding box
    fn bounding(&self) -> Box<dyn BoundingBox>;
    /// Output, should the proper region be clicked
    fn output(&self) -> Self::ClickOutput;
}

#[derive(Debug, Clone)]
/// Standard AABB bounding box
pub struct AABB {
    pos: (f64, f64),
    dim: (f64, f64),
}

impl AABB {
    pub fn at(x: f64, y: f64, length: f64, width: f64) -> Self {
        AABB {
            pos: (x, y),
            dim: (length, width)
        }
    }
}

impl BoundingBox for AABB {
    fn within(&self, mouse_pos: PhysicalPosition<f64>) -> bool {
        self.pos.0 < mouse_pos.x
            && mouse_pos.x < self.pos.0 + self.dim.0
            && self.pos.1 < mouse_pos.y
            && mouse_pos.y < self.pos.1 + self.dim.1
    }
}
