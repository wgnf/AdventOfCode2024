mod puzzles;

use std::{fs, io};
use std::path::Path;
use std::time::{Duration, Instant};

use crate::puzzles::{Puzzle, PuzzleRegistry};

fn main() {
    println!("...::: ADVENT OF CODE 2024 :::...");

    let mut puzzles = PuzzleRegistry::get_all();

    println!("Found following puzzles, which one do you want to solve?");

    for index in 0..puzzles.len() {
        let puzzle_number = index + 1;
        let puzzle = &puzzles[index];
        let puzzle_name = puzzle.get_puzzle_name();

        println!("[{puzzle_number}] '{puzzle_name}'");
    }

    let mut input_buffer = String::new();
    io::stdin().read_line(&mut input_buffer).expect("Failed to read line");

    let puzzle_number = input_buffer.trim().parse::<usize>().unwrap();
    let puzzle_index = puzzle_number - 1;
    let mut puzzle = &mut puzzles[puzzle_index];
    let puzzle_name = puzzle.get_puzzle_name();

    println!("Using puzzle '{puzzle_name}'");

    solve_puzzle(&mut puzzle, puzzle_number);
}

fn solve_puzzle(puzzle: &mut Box<dyn Puzzle>, puzzle_number: usize) {
    let example_result1 = puzzle.get_expected_example_result_part1();
    let example_result2 = puzzle.get_expected_example_result_part2();
    
    solve_part(1, puzzle_number, &mut *puzzle, example_result1);
    solve_part(2, puzzle_number, &mut *puzzle, example_result2);
}

fn solve_part(part_number: usize, puzzle_number: usize, puzzle: &mut Box<dyn Puzzle>, expected_sample_result: String) {
    println!("=== PART {part_number} ===");

    solve_example(part_number, puzzle_number, puzzle, expected_sample_result);
    solve_input(part_number, puzzle_number, puzzle);
}

fn solve_example(part_number: usize, puzzle_number: usize, puzzle: &mut Box<dyn Puzzle>, expected_sample_result: String) {
    println!(".. EXAMPLE ..");

    let example_file = format!("./src/inputs/example{puzzle_number}.txt");
    if !Path::new(example_file.as_str()).exists() {
        panic!("Example file '{example_file}' does not exist.");
    }

    let contents_example = read_lines_from_file(example_file.as_str());
    puzzle.init(contents_example);

    let (result, duration) = solve_using_puzzle(part_number, puzzle);
    
    let duration_ms = duration.as_millis();
    let result_correct = result == expected_sample_result;
    let result_icon = if result_correct { "✅" } else { "❌" };
    
    println!("===================================================");
    println!("expected: {expected_sample_result}");
    println!("actual  : {result} {result_icon}");
    println!("took    : {duration_ms:#?}ms");
    println!("===================================================");
    
    if !result_correct {
        panic!("THAT WAS WRONG STUPID");
    }
}

fn solve_input(part_number: usize, puzzle_number: usize, puzzle: &mut Box<dyn Puzzle>,) {
    println!(".. INPUT ..");

    let input_file = format!("./src/inputs/input{puzzle_number}.txt");
    if !Path::new(input_file.as_str()).exists() {
        panic!("Input file '{input_file}' does not exist.");
    }

    let contents_input = read_lines_from_file(input_file.as_str());
    puzzle.init(contents_input);

    let (result, duration) = solve_using_puzzle(part_number, puzzle);

    let duration_ms = duration.as_millis();

    println!("===================================================");
    println!("result  : {result}");
    println!("took    : {duration_ms:#?}ms");
    println!("===================================================");
}

fn solve_using_puzzle(part_number: usize, puzzle: &Box<dyn Puzzle>) -> (String, Duration) {
    let now = Instant::now();
    
    let result = if part_number == 1 {
        puzzle.solve_part1()
    } else if part_number == 2 {
        puzzle.solve_part2()
    } else {
        panic!("unexpected part number {part_number}")
    };
    
    let elapsed = now.elapsed();
    (result, elapsed)
}

fn read_lines_from_file(file_path: &str) -> Vec<String> {
    let contents = fs::read_to_string(file_path)
        .expect("Something went wrong reading the file");

    let lines = contents
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(String::from)
        .collect::<Vec<String>>();
    
    lines
}

