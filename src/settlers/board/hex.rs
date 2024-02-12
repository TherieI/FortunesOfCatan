use crate::settlers::board::card::{Gamble, Occupant, Resource};
use glium::texture::texture2d::Texture2d;
use glium::Surface;
use rand::{seq::SliceRandom, thread_rng, Rng};

#[derive(Clone, Copy)]
pub struct Vertex {
    pos: [f32; 2],
    tex_coords: [f32; 2],
}
implement_vertex!(Vertex, pos, tex_coords);

impl Vertex {
    fn new(x: f32, y: f32, s: f32, t: f32) -> Vertex {
        Vertex {
            pos: [x, y],
            tex_coords: [s, t],
        }
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
                },
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
                },
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
            for i in 1..I-1 {
                tiles[j][i].set_resource(chances.pop().unwrap());
            }
        }
        tiles[1][0].set_resource(chances.pop().unwrap());
        tiles[2][0].set_resource(chances.pop().unwrap());
        tiles[3][0].set_resource(chances.pop().unwrap());
        tiles[2][4].set_resource(chances.pop().unwrap());

        Board { tiles, }
    }

    pub fn buffers(&self) -> (Vec<Vertex>, Vec<u32>) {
        let vertices = Vec::new();
        let indices = Vec::new();

        (vertices, indices)
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
}