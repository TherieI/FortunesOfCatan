// Tiles on the board
#[derive(Debug, Clone, Copy)]
pub enum Resource {
    Wood(u8),
    Brick(u8),
    Ore(u8),
    Wheat(u8),
    Sheep(u8),
    Desert,
}

impl Resource {
    pub fn clone_with_value(&self, val: u8) -> Self {
        match self {
            Resource::Desert => Resource::Desert,
            Resource::Wood(n) => Resource::Wood(val),
            Resource::Brick(n) => Resource::Brick(val),
            Resource::Ore(n) => Resource::Ore(val),
            Resource::Wheat(n) => Resource::Wheat(val),
            Resource::Sheep(n) => Resource::Sheep(val)
        }
    }
}

// Development cards (gambling cards)
pub enum Gamble {
    Knight(u8),
    RoadBuilder(u8),
    YearOfPlenty(u8),
    Monopoly(u8),
    Victory(u8),
}

// Entities that can occupy a tile on the board, e.g the robber or merchant
#[derive(Debug, Clone, Copy)]
pub enum Occupant {
    Robber,
    Merchant,
}

#[cfg(test)]
mod tests {

    pub enum Res {
        Wood { n: u8 },
        Brick(u8),
    }

    fn match_test(re: &mut Res) {
        match re {
            Res::Wood { n } => {
                *n -= 1;
            },
            Res::Brick(val) => {
                *val -= 1;
            }
            _ => ()
        }
    }

    #[test]
    fn edit_enums() {
        let mut wood = Res::Wood { n: 10 };
        let mut brick = Res::Brick(10);
        match_test(&mut wood);
        match_test(&mut brick);
        if let Res::Wood { n } = wood {
            assert_eq!(n, 9);
        }
        if let Res::Brick(n) = brick {
            assert_eq!(n, 9)
        }
    }
}

