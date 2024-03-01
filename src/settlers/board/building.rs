use crate::settlers::matrix::Vec3;
use super::{card::Resource, hex::Hex};

// House
// Has pointers to all surrounding tiles
// position can be found by average of other positions
// Potentially increase the size of the board so every position has three surrounding tiles

#[derive(Clone, Copy)]
pub struct BuildingVertex {
    pos: [f32; 2],
    meta: u32,
}
implement_vertex!(BuildingVertex, pos, meta);

impl BuildingVertex {
    pub fn new(x: f32, y: f32) -> BuildingVertex{
        BuildingVertex {
            pos: [x, y],
            meta: 0
        }
    }

    // Todo: Add metadata implementation (road direction, structure color)
}

/// Represents a catan structure.
/// Stores the position of the structure relative to the position of the tiles in Map::tiles.
#[derive(Debug)]
pub enum Structure<'h> {
    Road {
        position: Vec3,
    },
    Settlement {
        position: Vec3,
        hexes: [Option<&'h Hex>; 3],
    },
    City {
        position: Vec3,
        hexes: [Option<&'h Hex>; 3],
    },
}

impl<'h> Structure<'h> {
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
            },
            Self::City { hexes, .. } => {
                for hex_option in hexes.iter() {
                    // Iter through all surrounding hexes
                    if let Some(hex) = hex_option {
                        // Ensure the hex is a land tile
                        if !hex.is_robbed() && hex.resource().chance() == roll {
                            // And that it is not robbed and was rolled
                            // You get one card for cities
                            resources.push(hex.resource().clone_with_value(2));
                        }
                    }
                }
            }
        }
        resources
    }
}