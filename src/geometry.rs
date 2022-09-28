#[derive(Default, PartialEq, Eq, Debug, Clone, Copy)]
pub struct Block {
    pub x: u8,
    pub y: u8,
}

#[derive(Debug, Clone, Copy)]
pub struct Area {
    pub min_x: u8,
    pub max_x: u8,
    pub min_y: u8,
    pub max_y: u8,
}

impl Area {
    pub fn new(max_x: u8, max_y: u8) -> Self {
        Self {
            min_x: 0,
            max_x,
            min_y: 0,
            max_y,
        }
    }
}
