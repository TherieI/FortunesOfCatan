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
