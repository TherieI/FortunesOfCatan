use crate::settlers::board::card::{Gamble, Occupant, Resource};
use glium::texture::texture2d::Texture2d;
use glium::Surface;
use rand::{seq::SliceRandom, thread_rng, Rng};

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
        let mut tile_value = 1;
        let mut inc = 1;

        let calulate_tile = |i: usize, remaining_resource: &mut u8| {
            // Add Tile and random number to output
            inc += 1;
            if inc > 1 {
                tile_value += 1;
                inc = 0;
            }
            // Deplete tiles
            *remaining_resource -= 1;
            println!("{}", *remaining_resource);
            if *remaining_resource == 0 {
                tiles.remove(i);
            }
        };
        while tiles.len() > 0 {
            // Get random tile
            let resource_index: usize = rng.gen_range(0..tiles.len());
            let resource: Resource;
            match &mut tiles[resource_index] {
                Resource::Desert => {
                    // Do something specific for Desert
                    out.push(Resource::Desert);
                    tiles.remove(resource_index);
                },
                Resource::Wood(amount_left)
                | Resource::Brick(amount_left)
                | Resource::Ore(amount_left)
                | Resource::Wheat(amount_left)
                | Resource::Sheep(amount_left) => {
                    // Add Tile and random number to output
                    inc += 1;
                    if inc > 1 {
                        tile_value += 1;
                        inc = 0;
                    }
                    // Deplete tiles
                    *amount_left -= 1;
                    println!("{}", *amount_left);
                    if *amount_left == 0 {
                        tiles.remove(resource_index);
                    }
                },
            };
        }
        out.shuffle(&mut rng);
        out
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


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn default_random_generation() {
        let tiles = Hex::random_set();
        for res in tiles.iter() {
            println!("{:?}", res);
        }
    }
}