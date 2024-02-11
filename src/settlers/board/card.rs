// Generic resource cards
pub enum Resource {
    Wood(u8),
    Brick(u8),
    Ore(u8),
    Wheat(u8),
    Sheep(u8),
}

// Development cards (gambling cards)
pub enum Gamble {
    Knight(u8),
    RoadBuilder(u8),
    YearOfPlenty(u8),
    Monopoly(u8),
    Victory(u8),
}
