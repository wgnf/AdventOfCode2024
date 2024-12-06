use crate::puzzles::Puzzle;

pub struct Puzzle1 {
    left_elements: Vec<i32>,
    right_elements: Vec<i32>,
}

impl Puzzle1 {
    pub fn new() -> Puzzle1 {
        Puzzle1 {
            left_elements: Vec::new(),
            right_elements: Vec::new(),
        }
    }
}

impl Puzzle for Puzzle1 {
    fn get_puzzle_name(&self) -> String {
        "Day 1: Historian Hysteria".to_string()
    }

    fn get_expected_example_result_part1(&self) -> String { "11".to_string() }
    fn get_expected_example_result_part2(&self) -> String { "31".to_string() }

    fn init(&mut self, input_lines: Vec<String>) {
        self.left_elements = Vec::new();
        self.right_elements = Vec::new();

        for line in input_lines {
            let sub_parts = line.trim().split("   ").collect::<Vec<&str>>();

            self.left_elements.push(sub_parts[0].parse::<i32>().unwrap());
            self.right_elements.push(sub_parts[1].parse::<i32>().unwrap());
        }

        self.left_elements.sort();
        self.right_elements.sort();
    }

    fn solve_part1(&self) -> String {
        let mut total_distance = 0;

        for index in 0..self.left_elements.len() {
            let distance = (&self.left_elements[index] - &self.right_elements[index]).abs();

            total_distance += distance;
        }

        total_distance.to_string()
    }

    fn solve_part2(&self) -> String {
        let mut similarity_score = 0;

        for element in &self.left_elements {
            let occurrences = self.right_elements.iter().filter(|&x| *x == *element).count();
            let similarity = occurrences as i32 * element;

            similarity_score += similarity;
        }

        similarity_score.to_string()
    }
}
