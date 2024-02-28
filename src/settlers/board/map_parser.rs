use std::fs::read_to_string;

use super::{card::Resource, hex::{Hex, MAX_HEX}};

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
            total_tiles += 1;
            match res_split[0] {
                "WO" => Ok(Resource::Wood(
                    res_split[1]
                        .parse()
                        .map_err(|_| ParseMapError::ResourceParseError)?,
                )),
                "BR" => Ok(Resource::Brick(
                    res_split[1]
                        .parse()
                        .map_err(|_| ParseMapError::ResourceParseError)?,
                )),
                "OR" => Ok(Resource::Ore(
                    res_split[1]
                        .parse()
                        .map_err(|_| ParseMapError::ResourceParseError)?,
                )),
                "WH" => Ok(Resource::Wheat(
                    res_split[1]
                        .parse()
                        .map_err(|_| ParseMapError::ResourceParseError)?,
                )),
                "SH" => Ok(Resource::Sheep(
                    res_split[1]
                        .parse()
                        .map_err(|_| ParseMapError::ResourceParseError)?,
                )),
                "DE" => Ok(Resource::Desert(Some(
                    res_split[1]
                        .parse()
                        .map_err(|_| ParseMapError::ResourceParseError)?,
                ))),
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
    let chances: Vec<u8> = chances.split(",").map(|chance| {
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
    }).collect::<Result<_, _>>()?;
    // Ensure total chances is the same as total tiles (excepting the desert tiles)
    todo!();
    if total_tiles != chances.len() as u32 || total_tiles > MAX_HEX {
        Err(ParseMapError::HexChanceMismatch)?
    }

    let mut map = vec![vec![Hex::new(); map_dim.1]; map_dim.0];
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
