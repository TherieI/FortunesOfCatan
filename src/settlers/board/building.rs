use super::{card::Resource, hex::Hex};
use crate::settlers::{interface::clickable::{BoundingBox, Clickable, AABB}, matrix::Vec3};
use std::{borrow::Borrow, rc::Rc, sync::Arc};

// House
// Has pointers to all surrounding tiles
// position can be found by average of other positions
// Potentially increase the size of the board so every position has three surrounding tiles

pub const SETTLEMENT_PATH: &'static str = "../../../assets/structures/settlement.png";

#[derive(Clone, Copy)]
pub struct BuildingVertex {
    pos: [f32; 2],
    /// First 4 bits: Type of structure
    /// 0 - ROAD,
    /// 1 - SETTLEMENT,
    /// 2 - CITY,
    /// Next 8 bits: Color ID (Player)
    /// ...
    /// Next 4 bits: Road info
    meta: u16,
}
implement_vertex!(BuildingVertex, pos, meta);

impl BuildingVertex {
    pub fn new(x: f32, y: f32) -> BuildingVertex {
        BuildingVertex {
            pos: [x, y],
            meta: 0,
        }
    }

    pub fn set_structure(&mut self, structure: &Structure) {
        let id = match structure {
            Structure::Road { .. } => 0u16,
            Structure::Settlement { .. } => 1,
            Structure::City { .. } => 2,
        };
        // Clear first 4 bits then add id
        self.meta = (self.meta & 0b1111111111110000) | id;
    }

    pub fn set_color(&mut self, color_id: u8) {
        let mut data = self.meta;
        // Clearing old color, then inserting new color
        data = ((data >> 4) & 0b111100000000) | color_id as u16;
        self.meta = (data << 4) | (self.meta & 0b1111);
    }
}

/// Represents a catan structure.
/// Stores the position of the structure relative to the position of the tiles in Map::tiles.
#[derive(Debug, Clone)]
pub enum Structure {
    Road {
        position: Vec3,
    },
    Settlement {
        position: Vec3,
        hexes: [Option<Rc<Hex>>; 3],
    },
    City {
        position: Vec3,
        hexes: [Option<Rc<Hex>>; 3],
    },
}

impl Structure {
    pub fn hexes(&self) -> &[Option<Rc<Hex>>; 3] {
        match self {
            Self::Road { .. } => &[None, None, None],
            Self::Settlement { hexes, .. } => hexes,
            Self::City { hexes, .. } => hexes
        }
    }
    /// Return resource for the player based on structure
    pub fn collect_resources(&self, roll: u8) -> Vec<Resource> {
        // Collect resources will only ever return a vec of length 0 - 3,
        let mut resources = Vec::with_capacity(3);
        match self {
            Self::Road { .. } => (),
            Self::Settlement { hexes, .. } => {
                for hex_option in hexes.iter() {
                    // Iter through all surrounding hexes
                    if let Some(hex) = hex_option {
                        // Ensure the hex is a land tile
                        if !hex.is_robbed() && hex.resource().chance() == roll {
                            // And that it is not robbed and was rolled
                            // You get one card for settlements
                            resources.push(hex.resource().clone_with_value(1));
                        }
                    }
                }
            }
            Self::City { hexes, .. } => {
                for hex_option in hexes.iter() {
                    // Iter through all surrounding hexes
                    if let Some(hex) = hex_option {
                        // Ensure the hex is a land tile
                        if !hex.is_robbed() && hex.resource().chance() == roll {
                            // And that it is not robbed and was rolled
                            // You get two card for cities
                            resources.push(hex.resource().clone_with_value(2));
                        }
                    }
                }
            }
        }
        resources
    }
}

impl Clickable for Structure {
    type ClickOutput = ();

    fn bounding(&self) -> Box<dyn BoundingBox> {
        Box::new(AABB::at(0., 0., 0., 0.))
    }

    fn output(&self) -> Self::ClickOutput {

    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn vertex_color() {
        let mut v = BuildingVertex::new(0., 0.);
        v.set_color(3);
        assert_eq!(v.meta, 48);
    }
    #[test]
    fn vertex_structure() {
        let mut v = BuildingVertex::new(0., 0.);
        v.set_structure(&Structure::City {
            position: Vec3::new(0., 0., 0.),
            hexes: [None, None, None],
        });
        assert_eq!(v.meta, 2);
    }
    #[test]
    fn vertex_all_meta() {
        let mut v = BuildingVertex::new(0., 0.);
        v.set_color(5);
        v.set_structure(&Structure::City {
            position: Vec3::new(0., 0., 0.),
            hexes: [None, None, None],
        });
        assert_eq!(82, v.meta);
    }
}
