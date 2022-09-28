use crate::geometry::*;
use crate::stash::Stash;
use crate::tetronimo::*;

pub enum Event {
    MoveLeft,
    MoveRight,
    MoveDown,
    MoveUp,
    Rotate,
    Drop,
}

#[derive(Default)]
pub struct DisplayState {
    pub player: Vec<Block>,
    pub next: Vec<Block>,
    pub projection: Vec<Block>,
    pub blocks: Vec<Block>,
    pub score: u64,
}

#[derive(Debug)]
pub struct Game {
    pub active: bool,
    pub area: Area,
    distance: u8,
    tetronimo: TetronimoRepresentation,
    next: TetronimoRepresentation,
    score: u64,
    stash: Stash,
}

impl Game {
    pub fn new(area: Area) -> Self {
        let tetronimo = TetronimoRepresentation::rand(area);
        Self {
            active: false,
            area,
            distance: area.max_y - tetronimo.lowest_line(),
            tetronimo,
            next: TetronimoRepresentation::rand(area),
            score: 0,
            stash: Stash::new(area.max_y),
        }
    }

    pub fn reset(&mut self) {
        *self = Self::new(self.area)
    }

    pub fn run(&mut self, event: Event) {
        if self.active {
            match event {
                Event::MoveLeft => self.move_tetronimo(Direction::Left),
                Event::MoveRight => self.move_tetronimo(Direction::Right),
                Event::MoveUp => self.move_tetronimo(Direction::Top),
                Event::MoveDown => self.move_tetronimo(Direction::Bottom),
                Event::Rotate => self.move_tetronimo(Direction::Rotate),
                Event::Drop => self.drop(),
            }
        }
    }
    pub fn get_level(&self) -> u64 {
        (self.score / 100) as _
    }

    pub fn get_display_state(&self) -> DisplayState {
        let next = Vec::from(self.next.shift(Direction::Rotate, 0, Area::new(4, 4)).now());
        let projection = if self.distance > 0 {
            Vec::from(self.build_projection().now())
        } else {
            vec![]
        };

        DisplayState {
            player: Vec::from(self.tetronimo.now()),
            next,
            projection,
            blocks: self.stash.get_blocks(),
            score: self.score,
        }
    }

    fn move_tetronimo(&mut self, direction: Direction) {
        match direction {
            Direction::Left | Direction::Right | Direction::Top | Direction::Rotate => {
                let moved = self.tetronimo.shift(direction, 1, self.area);
                let distance = self.stash.distance_to(moved.now());
                if distance != 0 {
                    self.tetronimo = moved;
                    self.distance = distance;
                }
            }
            Direction::Bottom => {
                if self.distance == 1 {
                    self.stash.add(self.tetronimo.now());
                    self.score +=
                        (2u32.pow(self.stash.remove_lines(self.area.max_x) as u32) - 1) as u64;
                    self.tetronimo = self.next;
                    self.next = TetronimoRepresentation::rand(self.area);
                } else {
                    self.tetronimo = self.tetronimo.shift(direction, 1, self.area);
                }
                self.distance = self.stash.distance_to(self.tetronimo.now());
                self.active = self.distance != 0;
            }
        }
    }

    fn drop(&mut self) {
        self.tetronimo = self
            .tetronimo
            .shift(Direction::Bottom, self.distance - 1, self.area);
        self.distance = 1;
        self.move_tetronimo(Direction::Bottom);
    }

    fn build_projection(&self) -> TetronimoRepresentation {
        self.tetronimo
            .shift(Direction::Bottom, self.distance - 1, self.area)
    }
}
