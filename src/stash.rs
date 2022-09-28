use crate::{geometry::Block, tetronimo::Tetronimo};

#[derive(Debug)]
pub struct Stash {
    content: Vec<Vec<u8>>,
    start: u8,
}

impl Stash {
    pub fn new(start: u8) -> Self {
        Self {
            content: vec![],
            start,
        }
    }

    pub fn add(&mut self, tetronimo: Tetronimo) {
        for block in tetronimo {
            assert!(self.start > block.y);
            while self.content.len() < (self.start - block.y) as usize {
                self.content.push(Vec::new());
            }
            self.content[(self.start - block.y - 1) as usize].push(block.x);
        }
    }

    pub fn remove_lines(&mut self, width: u8) -> u8 {
        let prev_size = self.content.len();
        self.content.retain(|line| line.len() < width as usize);
        (prev_size - self.content.len()) as u8
    }

    pub fn get_blocks(&self) -> Vec<Block> {
        self.content
            .iter()
            .enumerate()
            .flat_map(|(pos, line)| {
                line.iter()
                    .map(|&col| Block {
                        x: col,
                        y: self.start - pos as u8 - 1,
                    })
                    .collect::<Vec<_>>()
            })
            .collect()
    }

    pub fn distance_to(&self, tetronimo: Tetronimo) -> u8 {
        tetronimo
            .iter()
            .map(|block| self.get_highest(block.x, block.y) - block.y)
            .min()
            .unwrap() as u8
    }

    fn get_highest(&self, column: u8, row: u8) -> u8 {
        let limit = (self.start - row).min(self.content.len() as u8);
        self.start - limit
            + self.content[..limit as usize]
                .iter()
                .rev()
                .take_while(|line| !line.contains(&column))
                .count() as u8
    }
}
