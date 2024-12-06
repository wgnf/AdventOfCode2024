use crate::puzzles::{Puzzle, Puzzle1};

pub struct PuzzleRegistry { }

impl PuzzleRegistry {
    pub fn get_all() -> Vec<Box<dyn Puzzle>> {
        let mut puzzles: Vec<Box<dyn Puzzle>> = Vec::new();

        puzzles.push(Box::new(Puzzle1::new()));

        puzzles
    }
}