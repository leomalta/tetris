use crate::blocks::Blocks;
use crate::geometry::*;
use crate::tetronimo::*;

#[allow(dead_code)]
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
    pub player: Vec<Position>,
    pub next: Vec<Position>,
    pub projection: Vec<Position>,
    pub blocks: Vec<Position>,
    pub scene_area: Position,
    pub score: u64,
}

#[derive(Debug)]
pub struct Tetris {
    // total score
    score: u64,
    // area available for the game scene, represented by the bottom_right poisition
    scene_area: Position,
    // player tetronimo
    tetronimo: Tetronimo,
    // next tetronimo to come
    next: Tetronimo,
    // stash of dropped blocks
    dropped: Blocks,
}

impl Tetris {
    pub fn new(scene_area: Position) -> Self {
        Self {
            scene_area,
            score: 0,
            // create the first random tetronimo at the top of the area
            tetronimo: Tetronimo::random_at_top(scene_area),
            // create the next tetronimo
            next: Tetronimo::random_at_top(scene_area),
            // create the stash of dropped blocks at the bottom of the area
            dropped: Blocks::new(scene_area.y),
        }
    }

    pub fn reset(&mut self) {
        *self = Self::new(self.scene_area)
    }

    /// Run a game event and return the recomended time interval based on player level, or None if Game Over
    pub fn run(&mut self, event: Event) -> Option<std::time::Duration> {
        match event {
            Event::MoveLeft => self.move_tetronimo(Direction::Left),
            Event::MoveRight => self.move_tetronimo(Direction::Right),
            Event::MoveUp => self.move_tetronimo(Direction::Up),
            Event::MoveDown => self.move_tetronimo(Direction::Down),
            Event::Rotate => self.move_tetronimo(Direction::Rotate),
            Event::Drop => self.drop(),
        }
        // if there is still room to tetronimo to drop, return time interval to next auto tick
        (self.get_distance_to_drop() > 0).then_some(self.calculate_interval())
    }

    /// Return the Display state, i.e. the position of all blocks along
    /// with the remainder of data/stats needed to display the game
    pub fn get_display_state(&self) -> DisplayState {
        DisplayState {
            // Get the collection of blocks representing the player tetronimo
            player: self.tetronimo.now().collect(),
            // Get the collection of blocks representing the next tetronimo
            // transform it to center in a 4x4 area ('next' drawing panel area)
            next: self
                .next
                .transform(Direction::Rotate, 0, Position::new(4, 4))
                .now()
                .collect(),
            // projection of the player tetronimo at the top of the stack
            projection: self.build_projection().now().collect(),
            // all the blocks in the dropped stack
            blocks: self.dropped.get_blocks(),
            scene_area: self.scene_area,
            score: self.score,
        }
    }

    /// Return the distance (in blocks) from the player tetronimo to the top of the stash dropped blocks
    fn get_distance_to_drop(&self) -> u8 {
        self.dropped.distance_to(&self.tetronimo)
    }

    /// Return the player level based on the current score
    fn calculate_interval(&self) -> std::time::Duration {
        let base_timer = 1500;
        let player_level = (self.score / 100) as u32;
        std::time::Duration::from_millis(base_timer / (player_level as u64 + 3))
    }

    /// Move the player tetronimo in a given direction
    fn move_tetronimo(&mut self, direction: Direction) {
        match direction {
            // if player tetronimo touches the dropped stash, add its blocks to the dropped ones
            Direction::Down if (self.get_distance_to_drop() == 1) => {
                self.dropped.add(&self.tetronimo);
                // update the score, according to the removed/cleared lines
                let cleared_lines = self.dropped.clear_completed_rows(self.scene_area.x) as u32;
                self.score += 2u64.pow(cleared_lines) - 1;
                // take the next tetronimo (already instantiated) and create a new one in its place
                self.tetronimo =
                    std::mem::replace(&mut self.next, Tetronimo::random_at_top(self.scene_area));
            }
            _ => {
                // update the tetronimo with its transformed instance
                let moved_tetronimo = self.tetronimo.transform(direction, 1, self.scene_area);
                // Check if the new tetronimo collides with the stack of dropped
                // reject the new tetronimo if the cse as movement is invalid
                if self.dropped.distance_to(&moved_tetronimo) > 0 {
                    self.tetronimo = moved_tetronimo;
                }
            }
        }
    }

    /// Causes the player tetronimo to drop in the stash of dropped blocks
    fn drop(&mut self) {
        // update the tetronimo with the projection,
        // i.e, distance to dropped stash is equal to 1
        self.tetronimo = self.build_projection();
        // move tetronimo down, it will add it to the the stash of dropped
        self.move_tetronimo(Direction::Down);
    }

    /// Returns the projection of the player tetronimo on the stash of dropped blocks
    fn build_projection(&self) -> Tetronimo {
        self.tetronimo.transform(
            Direction::Down,
            self.get_distance_to_drop().saturating_sub(1),
            self.scene_area,
        )
    }
}
