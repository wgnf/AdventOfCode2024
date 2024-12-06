pub trait Puzzle {
    fn get_puzzle_name(&self) -> String;
    fn get_expected_example_result_part1(&self) -> String;
    fn get_expected_example_result_part2(&self) -> String;
    
    fn init(&mut self, input_lines: Vec<String>);

    fn solve_part1(&self) -> String;
    fn solve_part2(&self) -> String;
}