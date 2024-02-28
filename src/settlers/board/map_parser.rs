use std::fs::read_to_string;

use super::{
    card::Resource,
    hex::{Hex, MAX_HEX},
};

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

pub fn parse_map(file: &'static str) -> Result<Vec<Vec<Hex>>, ParseMapError> {
    let map_dim = (0usize, 0usize);
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
            dim.parse::<usize>().map_err(|_| ParseMapError::MapParseError)
        })
        .collect::<Result<_, _>>()?;
    if map_dim.len() != 2 {
        Err(ParseMapError::MapParseError)?
    }
    let mut map = vec![vec![Hex::new(); map_dim[1]]; map_dim[0]];
    for i in (map_pos + 2)..(map_pos + 2 + map_dim[0]) {
        let row = lines.get(i).ok_or(ParseMapError::MapParseError)?;
        for (j, c) in row.chars().enumerate() {
            match c {
                '0' => 
                '1' =>
                _ => Err(ParseMapError::MapParseError)?
            }
        }
    }

    Ok(map)
}

#[cfg(test)]
mod tests {
    use super::parse_map;
    #[test]
    fn check_output() {
        println!("{:?}", parse_map("src/settlers/board/maps/default.focm"));
        assert!(parse_map("src/settlers/board/maps/default.focm").is_ok());
    }
}
