use rand::RngCore;

use crate::geometry::*;
use crate::prototype::*;

pub type Tetronimo = [Block; 4];

#[derive(Debug)]
pub enum Direction {
    Left,
    Right,
    Top,
    Bottom,
    Rotate,
}

#[derive(Debug, Clone, Copy)]
pub struct TetronimoRepresentation {
    reference_position: Block,
    rotation: u8,
    prototype: TetronimoPrototype,
}

impl TetronimoRepresentation {
    pub fn rand(area: Area) -> Self {
        let rng = &mut rand::thread_rng();
        let reference_position = Block {
            x: (area.min_x + area.max_x) / 2,
            y: area.min_y,
        };
        let rotation = rng.next_u32() as u8 % 4;
        let prototype = PROTOTYPES[rng.next_u32() as usize % PROTOTYPES.len()];
        let reference_position = fix_position(prototype, rotation, reference_position, area);
        Self {
            reference_position,
            rotation,
            prototype,
        }
    }

    pub fn shift(&self, direction: Direction, step: u8, area: Area) -> Self {
        let mut reference_position = self.reference_position;
        let mut rotation = self.rotation;
        match direction {
            Direction::Left => reference_position.x = 0.max(reference_position.x as i8 - step as i8) as u8,
            Direction::Right => reference_position.x += step,
            Direction::Top => reference_position.y= 0.max(reference_position.y as i8 - step as i8) as u8,
            Direction::Bottom => reference_position.y += step,
            Direction::Rotate => rotation = (rotation + step) % 4,
        }
        Self {
            reference_position: fix_position(self.prototype, rotation, reference_position, area),
            rotation,
            ..*self
        }
    }

    pub fn lowest_line(&self) -> u8 {
        (bottom(self.prototype, self.rotation) + self.reference_position.y as i8) as u8
    }

    pub fn now(&self) -> Tetronimo {
        let mut result = Tetronimo::default();
        rotate_prototype(self.prototype, self.rotation)
            .iter()
            .enumerate()
            .for_each(|(pos, &(x, y))| {
                result[pos] = Block {
                    x: (self.reference_position.x as i8 + x) as u8,
                    y: (self.reference_position.y as i8 + y) as u8,
                }
            });
        result
    }
}
