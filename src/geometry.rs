#[derive(Debug)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
    Rotate,
}

#[derive(Default, PartialEq, Eq, Debug, Clone, Copy)]
pub struct Position {
    pub x: u8,
    pub y: u8,
}

impl Position {
    pub fn new(x: u8, y: u8) -> Self {
        Self { x, y }
    }

    pub fn top_middle(&self) -> Position {
        Position {
            x: self.x / 2,
            y: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::geometry::Position;

    #[test]
    fn position_test() {
        {
            assert_eq!(Position::new(4, 4).top_middle(), Position::new(2, 0));
            assert_eq!(Position::new(0, 4).top_middle(), Position::new(0, 0));
            assert_eq!(Position::new(0, 0).top_middle(), Position::new(0, 0));
            assert_eq!(Position::new(11, 10).top_middle(), Position::new(5, 0));
        }
    }
}
