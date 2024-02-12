use crate::settlers::board::card::{Gamble, Occupant, Resource};
use glium::texture::texture2d::Texture2d;
use glium::Surface;
use rand::{thread_rng, Rng};

#[derive(Clone, Copy)]
struct Vertex {
    pos: [f32; 2],
    tex_coords: [f32; 2],
}
implement_vertex!(Vertex, pos, tex_coords);

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

    pub fn with_resource(resource: Resource) -> Self {
        Hex { resource: Some(resource), occupants: None}
    }

    pub fn rob(&mut self) -> &mut Self {
        self.occupants = Some(Occupant::Robber);
        self
    }

    pub fn random_set() -> Vec<Resource> {
        // Number of resource tiles available
        let mut total_tiles: Vec<Resource> = Vec::with_capacity(5);
        total_tiles.push(Resource::Wood(4));
        total_tiles.push(Resource::Brick(3));
        total_tiles.push(Resource::Ore(3));
        total_tiles.push(Resource::Wheat(4));
        total_tiles.push(Resource::Sheep(4));
        total_tiles.push(Resource::Desert);

        let mut rng = rand::thread_rng();
        let tiles: Vec<Resource> = Vec::with_capacity(20);
        while total_tiles.len() > 0 {
            // Get random tile
            let resource_index: usize = rng.gen_range(0..total_tiles.len());
            match total_tiles[resource_index] {
                Resource::Desert => {
                    // Do something specific for Desert
                    total_tiles.remove(resource_index);
                },
                Resource::Wood(mut amount_left)
                | Resource::Brick(mut amount_left)
                | Resource::Ore(mut amount_left)
                | Resource::Wheat(mut amount_left)
                | Resource::Sheep(mut amount_left) => {
                    amount_left -= 1;
                    if amount_left == 0 {
                        total_tiles.remove(resource_index);
                    }
                },
            };
            break;
        }
        tiles
    }
}

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
        let tiles = [[Hex::new(); I]; J];
        Board { tiles, }
    }
}
