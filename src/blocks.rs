use crate::{geometry::Position, tetronimo::Tetronimo};

#[derive(Debug)]
pub struct Blocks {
    // bottom line of the stash
    start: u8,
    // vector of LINES of dropped block stacks
    content: Vec<Vec<u8>>,
}

impl Blocks {
    pub fn new(start: u8) -> Self {
        Self {
            start,
            content: Vec::new(),
        }
    }

    /// Add the blocks of tetronimo to the stash
    pub fn add(&mut self, tetronimo: &Tetronimo) {
        for block in tetronimo.now() {
            // Insert new lines if needed
            while self.content.len() < (self.start - block.y) as usize {
                self.content.push(Vec::new());
            }
            // add the block to the corresponding line in the stack
            self.content[(self.start - block.y - 1) as usize].push(block.x);
        }
    }

    /// Clear completed rows, returning the number of lines removed
    pub fn clear_completed_rows(&mut self, width: u8) -> u8 {
        let prev_size = self.content.len();
        self.content.retain(|line| line.len() < width as usize);
        (prev_size - self.content.len()) as u8
    }

    /// Get the vector of all the block postions in the stash
    pub fn get_blocks(&self) -> Vec<Position> {
        self.content
            .iter()
            .enumerate()
            .flat_map(|(pos, line)| {
                line.iter()
                    .map(|&col| Position {
                        x: col,
                        y: self.start - pos as u8 - 1,
                    })
                    .collect::<Vec<_>>()
            })
            .collect()
    }

    /// Return the distance from the tetronimo to the top of the stash
    pub fn distance_to(&self, tetronimo: &Tetronimo) -> u8 {
        tetronimo
            .now()
            .map(|block| self.get_highest(block.x, block.y) - block.y)
            .min()
            .unwrap() as u8
    }

    /// Get the highest row in a column stack, below a certain thresold
    fn get_highest(&self, column: u8, threshold: u8) -> u8 {
        let limit = (self.start - threshold).min(self.content.len() as u8);
        self.start - limit
            + self.content[..limit as usize]
                .iter()
                .rev()
                .take_while(|line| !line.contains(&column))
                .count() as u8
    }
}
