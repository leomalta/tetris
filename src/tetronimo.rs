use crate::geometry::{Direction, Position};
use lazy_static::lazy_static;
use rand::RngCore;
use std::cmp::{max, min};

#[derive(Debug, Clone)]
pub struct Tetronimo {
    position: Position,
    prototype: TetronimoPrototype,
}

impl Tetronimo {
    /// Returns a tetronimo positioned at the top middle of the scene_area
    pub fn random_at_top(scene_area: Position) -> Self {
        let rng = &mut rand::thread_rng();
        let prototype =
            PROTOTYPES[rng.next_u32() as usize % PROTOTYPES.len()].rotate(rng.next_u32() as i8 % 4);
        Self {
            position: Tetronimo::repostion(&prototype, scene_area.top_middle(), scene_area),
            prototype,
        }
    }

    /// Returns a transformed version of the tetronimo
    pub fn transform(&self, direction: Direction, step: u8, scene_area: Position) -> Self {
        let mut next_prot = self.prototype.clone();
        let mut next_pos = self.position;
        match direction {
            Direction::Left => next_pos.x = next_pos.x.saturating_sub(step),
            Direction::Right => next_pos.x = next_pos.x.saturating_add(step),
            Direction::Up => next_pos.y = next_pos.y.saturating_sub(step),
            Direction::Down => next_pos.y = next_pos.y.saturating_add(step),
            Direction::Rotate => {
                next_prot = next_prot.rotate(step as i8);
            }
        }
        Self {
            position: Tetronimo::repostion(&next_prot, next_pos, scene_area),
            prototype: next_prot,
        }
    }

    /// Returns an iterator over the blocks of the tetronimo at the current position
    pub fn now(&self) -> impl Iterator<Item = Position>{
        self.prototype
            .blocks
            .map(|block| Position {
                x: (self.position.x as i8 + block.0) as u8,
                y: (self.position.y as i8 + block.1) as u8,
            })
            .into_iter()
    }

    /// Returns the position of the tetronimo so all its blocks are inside the scene_area
    fn repostion(
        prototype: &TetronimoPrototype,
        current_pos: Position,
        scene_area: Position,
    ) -> Position {
        Position {
            x: repos_horizontal(
                current_pos.x as i8,
                scene_area,
                prototype.limits.left,
                prototype.limits.right,
            ),
            y: repos_vertical(
                current_pos.y as i8,
                scene_area,
                prototype.limits.top,
                prototype.limits.bottom,
            ),
        }
    }
}

fn repos_horizontal(x_pos: i8, scene_area: Position, left: i8, right: i8) -> u8 {
    let left_correction = min(0, x_pos + left);
    let right_correction = max(0, 1 + x_pos + right - scene_area.x as i8);
    (x_pos - left_correction - right_correction) as _
}

fn repos_vertical(y_pos: i8, scene_area: Position, top: i8, bottom: i8) -> u8 {
    let top_correction = min(0, y_pos + top);
    let bottom_correction = max(0, 1 + y_pos + bottom - scene_area.y as i8);
    (y_pos - top_correction - bottom_correction) as _
}

lazy_static! {
    static ref PROTOTYPES: [TetronimoPrototype; 7] = [
        TetronimoPrototype::from([(-2, 0), (-1, 0), (0, 0), (1, 0)]), // I
        TetronimoPrototype::from([(-1, 0), (0, 0), (0, -1), (1, -1)]), // S
        TetronimoPrototype::from([(-1, -1), (0, 0), (0, -1), (1, 0)]), // Z
        TetronimoPrototype::from([(-1, -1), (-1, 0), (0, 0), (0, -1)]), // 0
        TetronimoPrototype::from([(-1, -1), (-1, 0), (0, 0), (1, 0)]), // J
        TetronimoPrototype::from([(1, -1), (-1, 0), (0, 0), (1, 0)]), // L
        TetronimoPrototype::from([(-1, 0), (0, 0), (0, -1), (1, 0)]), // T
    ];
}

type PrototypeBlocks = [(i8, i8); 4];

#[derive(Debug, Clone)]
struct TetronimoPrototype {
    blocks: PrototypeBlocks,
    limits: ProtoTypeLimits,
}

impl TetronimoPrototype {
    fn from(blocks: [(i8, i8); 4]) -> Self {
        Self {
            blocks,
            limits: ProtoTypeLimits::from(&blocks),
        }
    }

    /// Rotates the tetronimo 90 degrees clockwise a given amount of times (step)
    fn rotate(&self, step: i8) -> Self {
        Self::from(self.blocks.map(|cell| rotate_block_position(cell, step)))
    }
}

const fn rotate_block_position(cell: (i8, i8), step: i8) -> (i8, i8) {
    (
        (cell.0 * turn(step) + cell.1 * turn(step - 1)) - shift(step),
        (cell.0 * turn(step + 1) + cell.1 * turn(step)) - shift(step + 1),
    )
}

const fn turn(rotation: i8) -> i8 {
    (1 - (rotation & 1) as i8) * (1 - (rotation & 2) as i8)
}
const fn shift(rotation: i8) -> i8 {
    ((rotation as i8) & 2) >> 1
}

#[derive(Debug, Clone)]
struct ProtoTypeLimits {
    left: i8,
    right: i8,
    top: i8,
    bottom: i8,
}

impl ProtoTypeLimits {
    fn from(blocks: &PrototypeBlocks) -> Self {
        let limits = Self {
            left: i8::MAX,
            right: i8::MIN,
            top: i8::MAX,
            bottom: i8::MIN,
        };
        blocks
            .iter()
            .fold(limits, |acc_limit, &block| acc_limit.update(block))
    }

    fn update(&self, block: (i8, i8)) -> Self {
        Self {
            left: min(self.left, block.0),
            right: max(self.right, block.0),
            top: min(self.top, block.1),
            bottom: max(self.bottom, block.1),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        geometry::Position,
        tetronimo::{Tetronimo, PROTOTYPES},
    };

    #[test]
    fn fix_position_test() {
        let scene_area = Position::new(4, 4);

        {
            let position = Position::new(0, 0);
            assert_eq!(
                Position::new(2, 0),
                Tetronimo::repostion(&PROTOTYPES[0], position, scene_area)
            );
            assert_eq!(
                Position::new(0, 2),
                // PROTOTYPES[0].rotate(1).repostion(position, scene_area)
                Tetronimo::repostion(&PROTOTYPES[0].rotate(1), position, scene_area)
            );
        }
        {
            let position = Position::new(4, 4);
            assert_eq!(
                Position::new(2, 3),
                Tetronimo::repostion(&PROTOTYPES[0], position, scene_area)
            );
            assert_eq!(
                Position::new(3, 2),
                Tetronimo::repostion(&PROTOTYPES[0].rotate(1), position, scene_area)
            );
        }
    }
}
