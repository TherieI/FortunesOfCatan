use crate::settlers::board::card::{Occupant, Resource};
use std::{
    cell::RefCell,
    fmt::{Display, Formatter},
    rc::Rc,
};

use super::building::Structure;

/// Maximum number of hex's on the board
pub const MAX_HEX: u32 = 64;

#[derive(Clone, Copy)]
pub struct HexVertex {
    pos: [f32; 2],
    /// Contains the resource & value of the hex
    /// Resource: First 8 bits, Hex value: Next 8 bits
    /// Resource | ID
    /// None     | 0
    /// Desert   | 1
    /// Wood     | 2
    /// Brick    | 3
    /// Ore      | 4
    /// Wheat    | 5
    /// Sheep    | 6
    hex_meta: u32,
}
implement_vertex!(HexVertex, pos, hex_meta);

impl HexVertex {
    pub fn new(x: f32, y: f32) -> HexVertex {
        HexVertex {
            pos: [x, y],
            hex_meta: 0,
        }
    }

    pub fn add_meta(&mut self, resource: Option<Resource>) {
        self.hex_meta = match resource {
            Some(Resource::Wood(n)) => 0u32 | 2 | (n as u32) << 8,
            Some(Resource::Brick(n)) => 0u32 | 3 | (n as u32) << 8,
            Some(Resource::Ore(n)) => 0u32 | 4 | (n as u32) << 8,
            Some(Resource::Wheat(n)) => 0u32 | 5 | (n as u32) << 8,
            Some(Resource::Sheep(n)) => 0u32 | 6 | (n as u32) << 8,
            Some(Resource::Desert(_)) => 0u32 | 1,
            // Shouldn't ever be none
            None => 0,
        }
    }

    pub fn position(&self) -> (f32, f32) {
        self.pos.into()
    }
}

impl Display for HexVertex {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "({}, {})", self.pos[0], self.pos[1])
    }
}


#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Hex {
    resource: Resource,
    occupants: Option<Occupant>,
    /// Corner layout:
    /// 0 - Top Right
    /// 1 - Top Center
    /// 2 - Top Left
    /// 3 - Bottom Left
    /// 4 - Bottom Center
    /// 5 - Bottom Right
    corners: [Option<Rc<RefCell<Structure>>>; 6],
}

#[allow(dead_code)]
impl Hex {
    pub fn new() -> Self {
        Hex {
            resource: Resource::Desert(None),
            occupants: None,
            corners: [None, None, None, None, None, None],
        }
    }

    pub fn set_corner(&mut self, corner_id: usize, building: Rc<RefCell<Structure>>) {
        self.corners[corner_id] = Some(building);
    }

    pub fn resource(&self) -> Resource {
        self.resource
    }

    pub fn set_resource(&mut self, resource: Resource) -> &mut Self {
        self.resource = resource;
        self
    }

    pub fn rob(&mut self) -> &mut Self {
        self.occupants = Some(Occupant::Robber);
        self
    }

    pub fn is_robbed(&self) -> bool {
        self.occupants.is_some()
    }
}
