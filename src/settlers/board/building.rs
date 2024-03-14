use super::{card::Resource, hex::Hex};
use crate::settlers::{
    interface::clickable::{BoundingBox, Clickable, AABB},
    matrix::Vec3,
};
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
    /// 0 - EMPTY
    /// 1 - ROAD,
    /// 2 - SETTLEMENT,
    /// 3 - CITY,
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
        let id = match structure.building {
            Building::Empty => 0u16,
            Building::Road => 1u16,
            Building::Settlement => 2,
            Building::City => 3,
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

#[derive(Debug, Clone, Copy)]
pub enum Building {
    Empty,
    Road,
    Settlement,
    City,
}

/// Represents a catan structure.
/// Stores the position of the structure relative to the position of the tiles in Map::tiles.
#[derive(Debug, Clone)]
pub struct Structure {
    building: Building,
    position: (f32, f32),
    hexes: [Option<Rc<Hex>>; 3],
}

impl Structure {
    pub fn uninitialized() -> Self {
        Self::new(Building::Empty, (0., 0.))
    }

    pub fn new(building: Building, position: (f32, f32)) -> Self {
        Self {
            building,
            position,
            hexes: [None, None, None]
        }
    }

    pub fn add_hex(&mut self, hex: Rc<Hex>) {
        for i in 0..self.hexes.len() {
            if self.hexes[i].is_none() {
                self.hexes[i] = Some(hex);
                break;
            }
        }
    }

    pub fn building(&self) -> Building {
        self.building
    }

    pub fn position(&self) -> (f32, f32) {
        self.position
    }

    pub fn hexes(&self) -> &[Option<Rc<Hex>>; 3] {
        match self.building {
            Building::Road => &[None, None, None],
            Building::Settlement => &self.hexes,
            Building::City => &self.hexes,
            _ => &[None, None, None]
        }
    }
    /// Return resource for the player based on structure
    pub fn collect_resources(&self, roll: u8) -> Vec<Resource> {
        // Collect resources will only ever return a vec of length 0 - 3,
        let mut resources = Vec::with_capacity(3);
        match self.building {
            Building::Road => (),
            Building::Settlement => {
                for hex_option in self.hexes.iter() {
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
            Building::City => {
                for hex_option in self.hexes.iter() {
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
            _ => ()
        };
        resources
    }
}

impl Clickable for Structure {
    type ClickOutput = ();

    fn bounding(&self) -> Box<dyn BoundingBox> {
        Box::new(AABB::at(0., 0., 0., 0.))
    }

    fn output(&self) -> Self::ClickOutput {}
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
        v.set_structure(&Structure::new(Building::City, (0., 0.)));
        assert_eq!(v.meta, 2);
    }
    #[test]
    fn vertex_all_meta() {
        let mut v = BuildingVertex::new(0., 0.);
        v.set_color(5);
        v.set_structure(&Structure::new(Building::City, (0., 0.)));
        assert_eq!(82, v.meta);
    }
}
