use std::f32::consts::PI;

use crate::settlers::board::{
    card::{Gamble, Occupant, Resource},
    hex,
};
use rand::{seq::SliceRandom, Rng};

use std::fmt::{Display, Formatter};

const BOARD_OFFSET: (f32, f32) = (5., 4.2);

#[derive(Clone, Copy)]
pub struct Vertex {
    pos: [f32; 2],
    /// Contains the resource & value of the hex
    /// Resource: First 8 bits, Hex value: Next 8 bits
    /// Resource | ID
    /// None     | 0
    /// Desert   | 1
    /// Brick    | 2
    /// Wood     | 3
    /// Ore      | 4
    /// Wheat    | 5
    /// Sheep    | 6
    hex_meta: u32,
}
implement_vertex!(Vertex, pos, hex_meta);

impl Vertex {
    fn new(x: f32, y: f32) -> Vertex {
        Vertex {
            pos: [x, y],
            hex_meta: 0,
        }
    }

    fn add_meta(&mut self, resource: Option<Resource>) {
        self.hex_meta = match resource {
            Some(Resource::Brick(n)) => 0u32 | 2 | (n as u32) << 8,
            Some(Resource::Wood(n)) => 0u32 | 3 | (n as u32) << 8,
            Some(Resource::Ore(n)) => 0u32 | 4 | (n as u32) << 8,
            Some(Resource::Wheat(n)) => 0u32 | 5 | (n as u32) << 8,
            Some(Resource::Sheep(n)) => 0u32 | 6 | (n as u32) << 8,
            Some(Resource::Desert) => 0u32 | 1,
            // Shouldn't ever be none
            None => 0,
        }
    }
}

impl Display for Vertex {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "({}, {})", self.pos[0], self.pos[1])
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Hex {
    // If resource is None, then the tile is an ocean
    resource: Option<Resource>,
    occupants: Option<Occupant>,
}

impl Hex {
    pub fn new() -> Self {
        Hex {
            resource: None,
            occupants: None,
        }
    }

    pub fn set_resource(&mut self, resource: Resource) -> &mut Self {
        self.resource = Some(resource);
        self
    }

    pub fn rob(&mut self) -> &mut Self {
        self.occupants = Some(Occupant::Robber);
        self
    }

    // Generate a set of tiles for the default 5x5 board
    pub fn random_set() -> Vec<Resource> {
        let mut out: Vec<Resource> = Vec::with_capacity(20);
        // Number of resource tiles available
        let mut tiles: Vec<Resource> = Vec::with_capacity(7);
        tiles.push(Resource::Wood(4));
        tiles.push(Resource::Brick(3));
        tiles.push(Resource::Ore(3));
        tiles.push(Resource::Wheat(4));
        tiles.push(Resource::Sheep(4));
        tiles.push(Resource::Desert);

        let mut rng = rand::thread_rng();
        let mut tile_value = 2;
        let mut inc = 1;

        while tiles.len() > 0 {
            // Get random tile
            let resource_index: usize = rng.gen_range(0..tiles.len());
            let resource = tiles[resource_index].clone();
            match &mut tiles[resource_index] {
                Resource::Desert => {
                    // Do something specific for Desert
                    out.push(Resource::Desert);
                    tiles.remove(resource_index);
                }
                Resource::Wood(amount_left)
                | Resource::Brick(amount_left)
                | Resource::Ore(amount_left)
                | Resource::Wheat(amount_left)
                | Resource::Sheep(amount_left) => {
                    // Add Tile and random number to output
                    if tile_value == 7 {
                        // Skip 7's
                        tile_value += 1;
                    }
                    out.push(resource.clone_with_value(tile_value));
                    inc += 1;
                    if inc > 1 {
                        tile_value += 1;
                        inc = 0;
                    }
                    // Deplete tiles
                    *amount_left -= 1;
                    if *amount_left == 0 {
                        tiles.remove(resource_index);
                    }
                }
            };
        }
        out.shuffle(&mut rng);
        out
    }
}

#[derive(Debug)]
pub struct Board<const I: usize, const J: usize> {
    tiles: [[Hex; I]; J],
}

impl<const I: usize, const J: usize> Board<I, J> {
    pub fn new() -> Self {
        Board {
            tiles: [[Hex::new(); I]; J],
        }
    }

    pub fn random_default() -> Self {
        if I != 5 || I != J {
            panic!("Board must be 5x5");
        }
        let mut tiles = [[Hex::new(); I]; J];
        let mut chances = Hex::random_set();
        for j in 0..J {
            for i in 1..I - 1 {
                tiles[j][i].set_resource(chances.pop().unwrap());
            }
        }
        tiles[1][0].set_resource(chances.pop().unwrap());
        tiles[2][0].set_resource(chances.pop().unwrap());
        tiles[3][0].set_resource(chances.pop().unwrap());
        tiles[2][4].set_resource(chances.pop().unwrap());

        Board { tiles }
    }

    // fn verts_of_pos(&self, hex_pos: (f32, f32), resource: Resource) -> Vec<Vertex> {
    //     let hex_radius = 2.7f32;
    //     let mut vertices = Vec::new();
    //     // Create vertex for center of hexagon
    //     let mut middle_vertex = Vertex::new(hex_pos.0, hex_pos.1);
    //     middle_vertex.add_meta(Some(resource));
    //     vertices.push(middle_vertex);
    //     // Create vertex for each 6 points of hexagon
    //     for i in 0..6 {
    //         let pos_x = hex_radius * f32::cos(2. * PI * i as f32 / 6. + PI / 2.);
    //         let pos_y = hex_radius * f32::sin(2. * PI * i as f32 / 6. + PI / 2.);
    //         let mut vertex = Vertex::new(
    //             hex_pos.0 + pos_x,
    //             hex_pos.1 + pos_y,
    //             1.2 * pos_x / 6. + 0.5,
    //             1.1 * pos_y / 6. + 0.5,
    //         );
    //         vertex.add_meta(Some(resource));
    //         vertices.push(vertex)
    //     }
    //     vertices
    // }

    pub fn buffers(&self) -> Vec<Vertex> {
        let mut vertices = Vec::new();
        for j in 0..J {
            for i in 0..I {
                let resource = self.tiles[j][i].resource;
                if let Some(res) = resource {

                    let offset = if j % 2 == 1 {
                        BOARD_OFFSET.0 / 2.
                    } else {
                        0.
                    };
                    
                    let mut vertex = Vertex::new(
                        BOARD_OFFSET.0 * i as f32 + offset,
                        BOARD_OFFSET.1 * j as f32,
                    );
                    vertex.add_meta(Some(res));
                    vertices.push(vertex);
                    
                } else {
                    // Water
                }
            }
        }
        vertices
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn default_random_generation() {
        let tiles = Hex::random_set();
        assert_eq!(tiles.len(), 19);
        for res in tiles.iter() {
            println!("{:?}", res);
        }
    }

    #[test]
    fn board5x5() {
        let board: Board<5, 5> = Board::random_default();
        println!("{:?}", board);
    }

    #[test]
    fn buffers() {
        let board: Board<5, 5> = Board::random_default();
        let buffers = board.buffers();
    }
}
