use super::{
    building::{self, Building, BuildingVertex, Structure},
    card::Resource,
    hex::{Hex, HexEdge, HexVertex, MAX_HEX},
};
use crate::{rand::Rng, settlers::matrix::Vec3};
use rand::seq::SliceRandom;
use std::{cell::RefCell, fs::read_to_string, rc::Rc};

#[derive(Debug)]
pub enum ParseMapError {
    FileNotFound,
    NotFocm,
    ExpansionNotFound,
    ResourcesNotFound,
    ResourceParseError,
    ChanceNotFound,
    ChanceParseError,
    ChanceNotSuitable,
    HexChanceMismatch,
    MapNotFound,
    MapParseError,
    MapSizeIncompatability,
}

const BOARD_OFFSET: (f32, f32) = (5., 4.22);

#[derive(Debug)]
pub struct Board {
    // All structures, including road, settlement, city...
    buildings: Vec<Rc<RefCell<Structure>>>,
    tiles: Vec<Vec<Option<Hex>>>,
    // Attributes of Hex tiles, used to randomize the map
    distribution: Vec<Resource>,
    chances: Vec<u8>,
}

impl Board {
    pub fn from_file(file: &'static str) -> Result<Self, ParseMapError> {
        // Filter the comments and newlines from the file
        let content = read_to_string(file).map_err(|_| ParseMapError::FileNotFound)?;
        let lines: Vec<_> = content
            .lines()
            .filter_map(|line| {
                let line = line.trim();
                if line.starts_with("#") || line.len() == 0 {
                    // If the line is a comment or the line is blank, filter it
                    None
                } else {
                    Some(line)
                }
            })
            .collect();
        // First line should be __FOCM__
        if *lines.get(0).ok_or(ParseMapError::NotFocm)? != "__FOCM__" {
            return Err(ParseMapError::NotFocm);
        }
        // Find expansion
        let expansion_pos = lines
            .iter()
            .position(|line| *line == "[EXPANSION]")
            .ok_or(ParseMapError::ExpansionNotFound)?;
        let _expansion = lines
            .get(expansion_pos + 1)
            .ok_or(ParseMapError::ExpansionNotFound)?;
        // Find resources
        let resource_pos = lines
            .iter()
            .position(|line| *line == "[RESOURCES]")
            .ok_or(ParseMapError::ResourcesNotFound)?;
        let resources = lines
            .get(resource_pos + 1)
            .ok_or(ParseMapError::ResourcesNotFound)?;
        // Parse resources
        let mut total_tiles: u32 = 0;
        let resources: Vec<Resource> = resources
            .split(",")
            .map(|res| {
                let res_split: Vec<_> = res.trim().split(":").collect();
                if res_split.len() != 2 {
                    // Resources should be in the format "RE:X"
                    return Err(ParseMapError::ResourceParseError);
                }
                let count = res_split[1]
                    .parse::<u8>()
                    .map_err(|_| ParseMapError::ResourceParseError)?;
                total_tiles += count as u32;
                match res_split[0] {
                    "WO" => Ok(Resource::Wood(count)),
                    "BR" => Ok(Resource::Brick(count)),
                    "OR" => Ok(Resource::Ore(count)),
                    "WH" => Ok(Resource::Wheat(count)),
                    "SH" => Ok(Resource::Sheep(count)),
                    "DE" => Ok(Resource::Desert(Some(count))),
                    _ => Err(ParseMapError::ResourceParseError),
                }
            })
            .collect::<Result<_, _>>()?;
        // Find chance values
        let chance_pos = lines
            .iter()
            .position(|line| *line == "[CHANCES]")
            .ok_or(ParseMapError::ChanceNotFound)?;
        let chances = lines
            .get(chance_pos + 1)
            .ok_or(ParseMapError::ChanceNotFound)?;
        // Parse chances
        let chances: Vec<u8> = chances
            .split(",")
            .map(|chance| {
                let c = chance.trim().parse::<u8>();
                if let Some(val) = c.ok() {
                    if val < 2 || val > 12 || val == 7 {
                        Err(ParseMapError::ChanceNotSuitable)
                    } else {
                        Ok(val)
                    }
                } else {
                    Err(ParseMapError::ChanceParseError)
                }
            })
            .collect::<Result<_, _>>()?;
        // Ensure total chances == total tiles (excepting the desert tiles)
        let desert_tiles = resources
            .iter()
            .find(|res| {
                if let Resource::Desert(_) = res {
                    true
                } else {
                    false
                }
            })
            .map(|res| {
                if let Resource::Desert(val) = res {
                    val.unwrap_or(0)
                } else {
                    0
                }
            })
            .unwrap();
        if total_tiles - desert_tiles as u32 != chances.len() as u32 || total_tiles > MAX_HEX {
            Err(ParseMapError::HexChanceMismatch)?
        }
        // Find map
        let map_pos = lines
            .iter()
            .position(|line| *line == "[MAP]")
            .ok_or(ParseMapError::MapNotFound)?;
        // Get dimensions
        let map_dim: Vec<usize> = lines
            .get(map_pos + 1)
            .ok_or(ParseMapError::MapNotFound)?
            .split("x")
            .map(|dim| {
                dim.parse::<usize>()
                    .map_err(|_| ParseMapError::MapParseError)
            })
            .collect::<Result<_, _>>()?;
        if map_dim.len() != 2 {
            Err(ParseMapError::MapParseError)?
        }
        // We add 1 to create a border around the map
        let mut actual_tiles = 0;
        let mut map: Vec<Vec<Option<Hex>>> = vec![vec![None; map_dim[0] + 2]; map_dim[1] + 2];
        for i in 1..(map_dim[1] + 1) {
            let row = lines
                .get(map_pos + 2 + i - 1)
                .ok_or(ParseMapError::MapSizeIncompatability)?;
            // Iterate through all the characters of each row of the map
            for (j, c) in row.chars().enumerate() {
                match c {
                    '0' => map[i][j + 1] = None,
                    '1' => {
                        map[i][j + 1] = Some(Hex::new());
                        actual_tiles += 1;
                    }
                    _ => Err(ParseMapError::MapParseError)?,
                }
            }
        }
        if actual_tiles != total_tiles {
            Err(ParseMapError::MapSizeIncompatability)?
        }
        let mut map = Self {
            buildings: Vec::new(),
            tiles: map,
            distribution: resources,
            chances,
        };
        // Generate all Structure positions on the map
        map.gen_structure_positions();
        Ok(map)
    }

    /// Returns true if the tile is in bounds and is land, otherwise false
    fn land(&mut self, x: i32, y: i32) -> Option<&mut Hex> {
        if !(x < 0 || x > self.tiles[0].len() as i32 || y < 0 || y > self.tiles.len() as i32) {
            self.tiles[y as usize][x as usize].as_mut()
        } else {
            None
        }
    }

    fn gen_structure_positions(&mut self) {
        // Generate all Structure positions on the map
        for j in 1..(self.tiles.len() - 1) {
            for i in 1..(self.tiles[0].len() - 1) {
                // Would have loved to do something like `if let Some(hex) = &mut self.tiles[j][i]`
                // But then I would have had multiple mutable references when I set the neighbors
                if self.tiles[j][i].is_some() {
                    let corners = self.tiles[j][i].as_mut().unwrap().corners().to_owned();
                    use std::f32::consts::PI;
                    let hex_radius = 2.8;
                    for (corner_id, corner) in corners.iter().enumerate() {
                        // We can use these calculations to determine the position of the hex's edge relative to the center
                        // https://www.desmos.com/calculator/hwsecfsgvj
                        let theta = 2.0 * PI * corner_id as f32 / 6.0 + PI / 2.0;
                        // Position of hexagon point
                        let offset = if j % 2 == 0 { BOARD_OFFSET.0 / 2. } else { 0. };
                        let pos = (i as f32 * BOARD_OFFSET.0 + offset + hex_radius * -theta.cos(), j as f32 * BOARD_OFFSET.1 + hex_radius * theta.sin());
                        // Instanciate building
                        let building = Rc::new(RefCell::new(Structure::new(Building::Empty, pos)));
                        // There could be a better way of doing this, idk
                        match corner {
                            HexEdge::Top(structure) => {
                                if structure.is_none() {
                                    // Add building to buildings array
                                    self.buildings.push(building.clone());
                                    self.tiles[j][i]
                                        .as_mut()
                                        .unwrap()
                                        .set_corner(HexEdge::Top(Some(building.clone())));
                                    // Set edge for corrisponding neighbor(s)
                                    // Math is different for even / odd hex neighbors
                                    if j % 2 == 0 {
                                        // Even
                                        if let Some(hex) = self.land(i as i32, j as i32 + 1) {
                                            // Set corner of top left hex
                                            hex.set_corner(HexEdge::BottomRight(Some(
                                                building.clone(),
                                            )));
                                        }
                                        if let Some(hex) = self.land(i as i32 + 1, j as i32 + 1) {
                                            // Set corner of top right hex
                                            hex.set_corner(HexEdge::BottomLeft(Some(
                                                building.clone(),
                                            )));
                                        }
                                    } else {
                                        // Odd
                                        if let Some(hex) = self.land(i as i32 - 1, j as i32 + 1) {
                                            // Set corner of top left hex
                                            hex.set_corner(HexEdge::BottomRight(Some(
                                                building.clone(),
                                            )));
                                        }
                                        if let Some(hex) = self.land(i as i32, j as i32 + 1) {
                                            // Set corner of top right hex
                                            hex.set_corner(HexEdge::BottomLeft(Some(
                                                building.clone(),
                                            )));
                                        }
                                    }
                                }
                            }
                            HexEdge::TopRight(structure) => {
                                if structure.is_none() {
                                    // Add building to buildings array
                                    self.buildings.push(building.clone());
                                    self.tiles[j][i]
                                        .as_mut()
                                        .unwrap()
                                        .set_corner(HexEdge::TopRight(Some(building.clone())));
                                    // Set edge for corrisponding neighbor(s)
                                    // Math is different for even / odd hex neighbors
                                    if j % 2 == 0 {
                                        // Even
                                        if let Some(hex) = self.land(i as i32 + 1, j as i32 + 1) {
                                            // Set corner of top right hex
                                            hex.set_corner(HexEdge::Bottom(Some(
                                                building.clone(),
                                            )));
                                        }
                                        if let Some(hex) = self.land(i as i32 + 1, j as i32) {
                                            // Set corner of right hex
                                            hex.set_corner(HexEdge::TopLeft(Some(
                                                building.clone(),
                                            )));
                                        }
                                    } else {
                                        // Odd
                                        if let Some(hex) = self.land(i as i32, j as i32 + 1) {
                                            // Set corner of top right hex
                                            hex.set_corner(HexEdge::Bottom(Some(
                                                building.clone(),
                                            )));
                                        }
                                        if let Some(hex) = self.land(i as i32 + 1, j as i32) {
                                            // Set corner of right hex
                                            hex.set_corner(HexEdge::TopLeft(Some(
                                                building.clone(),
                                            )));
                                        }
                                    }
                                }
                            }
                            HexEdge::BottomRight(structure) => {
                                if structure.is_none() {
                                    // Add building to buildings array
                                    self.buildings.push(building.clone());
                                    self.tiles[j][i]
                                        .as_mut()
                                        .unwrap()
                                        .set_corner(HexEdge::BottomRight(Some(building.clone())));
                                    // Set edge for corrisponding neighbor(s)
                                    // Math is different for even / odd hex neighbors
                                    if j % 2 == 0 {
                                        // Even
                                        if let Some(hex) = self.land(i as i32 + 1, j as i32) {
                                            // Set corner of right hex
                                            hex.set_corner(HexEdge::BottomLeft(Some(
                                                building.clone(),
                                            )));
                                        }
                                        if let Some(hex) = self.land(i as i32 + 1, j as i32 - 1) {
                                            // Set corner of bottom right hex
                                            hex.set_corner(HexEdge::Top(Some(
                                                building.clone(),
                                            )));
                                        }
                                    } else {
                                        // Odd
                                        if let Some(hex) = self.land(i as i32 + 1, j as i32) {
                                            // Set corner of right hex
                                            hex.set_corner(HexEdge::BottomLeft(Some(
                                                building.clone(),
                                            )));
                                        }
                                        if let Some(hex) = self.land(i as i32, j as i32 - 1) {
                                            // Set corner of bottom right hex
                                            hex.set_corner(HexEdge::Top(Some(
                                                building.clone(),
                                            )));
                                        }
                                    }
                                }
                            }
                            HexEdge::Bottom(structure) => {
                                if structure.is_none() {
                                    // Add building to buildings array
                                    self.buildings.push(building.clone());
                                    self.tiles[j][i]
                                        .as_mut()
                                        .unwrap()
                                        .set_corner(HexEdge::Bottom(Some(building.clone())));
                                    // Set edge for corrisponding neighbor(s)
                                    // Math is different for even / odd hex neighbors
                                    if j % 2 == 0 {
                                        // Even
                                        if let Some(hex) = self.land(i as i32, j as i32 - 1) {
                                            // Set corner of bottom left hex
                                            hex.set_corner(HexEdge::TopRight(Some(
                                                building.clone(),
                                            )));
                                        }
                                        if let Some(hex) = self.land(i as i32 + 1, j as i32 - 1) {
                                            // Set corner of bottom right hex
                                            hex.set_corner(HexEdge::TopLeft(Some(
                                                building.clone(),
                                            )));
                                        }
                                    } else {
                                        // Odd
                                        if let Some(hex) = self.land(i as i32 - 1, j as i32 - 1) {
                                            // Set corner of bottom left hex
                                            hex.set_corner(HexEdge::TopRight(Some(
                                                building.clone(),
                                            )));
                                        }
                                        if let Some(hex) = self.land(i as i32, j as i32 - 1) {
                                            // Set corner of bottom right hex
                                            hex.set_corner(HexEdge::TopLeft(Some(
                                                building.clone(),
                                            )));
                                        }
                                    }
                                }
                            }
                            HexEdge::BottomLeft(structure) => {
                                if structure.is_none() {
                                    // Add building to buildings array
                                    self.buildings.push(building.clone());
                                    self.tiles[j][i]
                                        .as_mut()
                                        .unwrap()
                                        .set_corner(HexEdge::BottomLeft(Some(building.clone())));
                                    // Set edge for corrisponding neighbor(s)
                                    // Math is different for even / odd hex neighbors
                                    if j % 2 == 0 {
                                        // Even
                                        if let Some(hex) = self.land(i as i32, j as i32 - 1) {
                                            // Set corner of bottom left hex
                                            hex.set_corner(HexEdge::Top(Some(
                                                building.clone(),
                                            )));
                                        }
                                        if let Some(hex) = self.land(i as i32 - 1, j as i32) {
                                            // Set corner of left hex
                                            hex.set_corner(HexEdge::BottomRight(Some(
                                                building.clone(),
                                            )));
                                        }
                                    } else {
                                        // Odd
                                        if let Some(hex) = self.land(i as i32 - 1, j as i32 - 1) {
                                            // Set corner of bottom left hex
                                            hex.set_corner(HexEdge::Top(Some(
                                                building.clone(),
                                            )));
                                        }
                                        if let Some(hex) = self.land(i as i32 - 1, j as i32) {
                                            // Set corner of left hex
                                            hex.set_corner(HexEdge::BottomRight(Some(
                                                building.clone(),
                                            )));
                                        }
                                    }
                                }
                            }
                            HexEdge::TopLeft(structure) => {
                                if structure.is_none() {
                                    // Add building to buildings array
                                    self.buildings.push(building.clone());
                                    self.tiles[j][i]
                                        .as_mut()
                                        .unwrap()
                                        .set_corner(HexEdge::TopLeft(Some(building.clone())));
                                    // Set edge for corrisponding neighbor(s)
                                    // Math is different for even / odd hex neighbors
                                    if j % 2 == 0 {
                                        // Even
                                        if let Some(hex) = self.land(i as i32, j as i32 + 1) {
                                            // Set corner of top left hex
                                            hex.set_corner(HexEdge::Bottom(Some(
                                                building.clone(),
                                            )));
                                        }
                                        if let Some(hex) = self.land(i as i32 - 1, j as i32) {
                                            // Set corner of left hex
                                            hex.set_corner(HexEdge::TopRight(Some(
                                                building.clone(),
                                            )));
                                        }
                                    } else {
                                        // Odd
                                        if let Some(hex) = self.land(i as i32 - 1, j as i32 + 1) {
                                            // Set corner of top left hex
                                            hex.set_corner(HexEdge::Bottom(Some(
                                                building.clone(),
                                            )));
                                        }
                                        if let Some(hex) = self.land(i as i32 - 1, j as i32) {
                                            // Set corner of left hex
                                            hex.set_corner(HexEdge::TopRight(Some(
                                                building.clone(),
                                            )));
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn randomize(&mut self) {
        let mut rng = rand::thread_rng();
        let mut distribution = self.distribution.clone();
        let mut chances = self.chances.clone();
        distribution.shuffle(&mut rng);
        chances.shuffle(&mut rng);

        let mut random_tiles = Vec::new();

        while distribution.len() > 0 {
            // Get random tile
            let resource_index: usize = rng.gen_range(0..distribution.len());
            let resource = distribution[resource_index].clone();
            match &mut distribution[resource_index] {
                Resource::Desert(amount_left) => {
                    // Option specific for Desert
                    random_tiles.push(Resource::Desert(None));
                    if let Some(remaining) = amount_left {
                        *remaining -= 1;
                        if *remaining == 0 {
                            distribution.remove(resource_index);
                        }
                    }
                }
                Resource::Wood(amount_left)
                | Resource::Brick(amount_left)
                | Resource::Ore(amount_left)
                | Resource::Wheat(amount_left)
                | Resource::Sheep(amount_left) => {
                    // Add Tile and random number to output
                    random_tiles.push(
                        resource.clone_with_value(
                            chances
                                .pop()
                                .expect("Chances and total tiles should be equal"),
                        ),
                    );
                    // Deplete distribution
                    *amount_left -= 1;
                    if *amount_left == 0 {
                        distribution.remove(resource_index);
                    }
                }
            };
        }
        // Add move each random tile in random_tiles to the map
        for j in 0..self.tiles.len() {
            for i in 0..self.tiles[0].len() {
                if let Some(hex) = &mut self.tiles[j][i] {
                    hex.set_resource(random_tiles.pop().unwrap());
                }
            }
        }
        // println!("{:?}", self.tiles);
    }

    pub fn hex_buffers(&self) -> Vec<HexVertex> {
        let mut vertices = Vec::new();
        let (width, height) = (self.tiles[0].len(), self.tiles.len());
        for j in 0..height {
            for i in 0..width {
                if let Some(hex) = &self.tiles[j][i] {
                    // Push a single point, the center of the hexagon, to the buffer
                    // The gpu will transform this point into a hexagon in the geometry shader
                    let offset = if j % 2 == 0 { BOARD_OFFSET.0 / 2. } else { 0. };
                    let mut vertex = HexVertex::new(
                        BOARD_OFFSET.0 * i as f32 + offset,
                        BOARD_OFFSET.1 * j as f32,
                    );
                    vertex.add_meta(Some(hex.resource()));
                    vertices.push(vertex);
                } else {
                    // Water
                }
            }
        }
        vertices
    }

    pub fn building_buffers(&self) -> Vec<BuildingVertex> {
        self.buildings
            .iter()
            .map(|structure| {
                let pos = structure.borrow().position();
                let mut bv = BuildingVertex::new(pos.0, pos.1);
                bv.set_structure(&structure.borrow());
                bv
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    // use super::*;
    #[test]
    fn default_random_generation() {}

    #[test]
    fn check_output() {
        // println!("{:?}", Map::parse_map("src/settlers/board/maps/default.focm"));
        // assert!(parse_map("src/settlers/board/maps/default.focm").is_ok());
        let i: i32 = -2;
        let u: usize = i as usize;
        println!("{}", u)
    }
}
