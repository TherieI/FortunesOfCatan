use crate::settlers::matrix::Mat4;


#[derive(Clone, Copy)]
/// For drawing bounding boxes
pub struct BoundingVertex {
    pos: [f32; 2],
    size: [f32; 2],
}
implement_vertex!(BoundingVertex, pos, size);

impl BoundingVertex {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        BoundingVertex {
            pos: [x, y],
            size: [width, height],
        }
    }
}

/// A region of the screen. For example, AABB bounding.
pub trait BoundingBox {
    fn vertex(&self) -> BoundingVertex;
    fn within(&self, normalized: (f32, f32)) -> bool;
}

/// Types that are clickable should implement this trait
pub trait Clickable {
    /// Click output is what you recieve from a mouse click
    type ClickOutput;
    /// Is the mouse position within a certain region of the screen based on a bounding box
    fn bounding(&self, mvp: Option<&Mat4>) -> Box<dyn BoundingBox>;
    /// Output, should the proper region be clicked
    fn output(&self) -> Self::ClickOutput;
}

#[derive(Debug, Clone)]
/// Standard AABB bounding box
pub struct AABB {
    pos: (f32, f32),
    dim: (f32, f32),
}

impl AABB {
    pub fn at(x: f32, y: f32, length: f32, width: f32) -> Self {
        AABB {
            pos: (x, y),
            dim: (length, width),
        }
    }
}

impl BoundingBox for AABB {
    fn vertex(&self) -> BoundingVertex {
        BoundingVertex::new(
            self.pos.0,
            self.pos.1,
            self.dim.0,
            self.dim.1,
        )
    }

    fn within(&self, normalized: (f32, f32)) -> bool {
        // println!("{:?} | {:?}", normalized, self.pos);
        self.pos.0 < normalized.0
            && normalized.0 < self.pos.0 + self.dim.0
            && self.pos.1 < normalized.1
            && normalized.1 < self.pos.1 + self.dim.1
    }
}
