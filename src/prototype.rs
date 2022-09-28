use crate::geometry::{Area, Block};

pub type TetronimoPrototype = [(i8, i8); 4];

pub const PROTOTYPES: [TetronimoPrototype; 7] = [
    [(-2, 0), (-1, 0), (0, 0), (1, 0)],   // I
    [(-1, 0), (0, 0), (0, -1), (1, -1)],  // S
    [(-1, -1), (0, 0), (0, -1), (1, 0)],  // Z
    [(-1, -1), (-1, 0), (0, 0), (0, -1)], // 0
    [(-1, -1), (-1, 0), (0, 0), (1, 0)],  // J
    [(1, -1), (-1, 0), (0, 0), (1, 0)],   // L
    [(-1, 0), (0, 0), (0, -1), (1, 0)],   // T
];

fn rotate(rotation: u8) -> i8 {
    (1 - (rotation & 1) as i8) * (1 - (rotation & 2) as i8)
}
fn shift(rotation: u8) -> i8 {
    ((rotation as i8) & 2) >> 1
}

pub fn rotate_prototype(mut prototype: TetronimoPrototype, rotation: u8) -> TetronimoPrototype {
    for cell in &mut prototype {
        *cell = (
            (cell.0 * rotate(rotation) + cell.1 * rotate((rotation as i8 - 1) as u8))
                - shift(rotation),
            (cell.0 * rotate(rotation + 1) + cell.1 * rotate(rotation)) - shift(rotation + 1),
        );
    }
    prototype
}

pub fn left(prototype: TetronimoPrototype, rotation: u8) -> i8 {
    rotate_prototype(prototype, rotation)
        .iter()
        .min()
        .unwrap()
        .0
}

pub fn right(prototype: TetronimoPrototype, rotation: u8) -> i8 {
    rotate_prototype(prototype, rotation)
        .iter()
        .max()
        .unwrap()
        .0
}

pub fn top(prototype: TetronimoPrototype, rotation: u8) -> i8 {
    rotate_prototype(prototype, rotation)
        .iter()
        .min_by(|lhs, rhs| lhs.1.cmp(&rhs.1))
        .unwrap()
        .1
}

pub fn bottom(prototype: TetronimoPrototype, rotation: u8) -> i8 {
    rotate_prototype(prototype, rotation)
        .iter()
        .max_by(|lhs, rhs| lhs.1.cmp(&rhs.1))
        .unwrap()
        .1
}

pub fn fix_position(
    prototype: TetronimoPrototype,
    rotation: u8,
    position: Block,
    area: Area,
) -> Block {
    Block {
        x: (position.x as i8
            + (0.max(area.min_x as i8 - (position.x as i8 + left(prototype, rotation)))
                - 0.max(position.x as i8 + right(prototype, rotation) - area.max_x as i8 + 1)))
            as u8,
        y: (position.y as i8
            + (0.max(area.min_y as i8 - (position.y as i8 + top(prototype, rotation)))
                - 0.max(position.y as i8 + bottom(prototype, rotation) - area.max_y as i8 + 1)))
            as u8,
    }
}

#[cfg(test)]
mod tests {
    use crate::geometry::{Area, Block};
    use crate::prototype::{fix_position, PROTOTYPES};

    const AREA: Area = Area {
        min_x: 0,
        min_y: 0,
        max_x: 4,
        max_y: 4,
    };

    #[test]
    fn fix_position_test() {
        {
            let position = Block { x: 0, y: 0 };
            let result_x = Block { x: 2, y: 0 };
            assert_eq!(result_x, fix_position(PROTOTYPES[0], 0, position, AREA));
            let result_y = Block { x: 0, y: 2 };
            assert_eq!(result_y, fix_position(PROTOTYPES[0], 1, position, AREA));
        }
        {
            let position = Block { x: 4, y: 4 };
            let result_x = Block { x: 2, y: 3 };
            assert_eq!(result_x, fix_position(PROTOTYPES[0], 0, position, AREA));

            let result_y = Block { x: 3, y: 2 };
            assert_eq!(result_y, fix_position(PROTOTYPES[0], 1, position, AREA));
        }
    }
}
