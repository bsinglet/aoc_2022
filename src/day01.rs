use std::fs;
use std::str::FromStr;

fn read_lines(filename: &str) -> Vec<String> {
    let mut contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file.");
    let mut lines: Vec<String> = Vec::new();
    for each in contents.split_terminator("\n").collect::<Vec<&str>>() {
        lines.push(each.to_string());
    }
    lines
}

fn process_lines(lines: &Vec<String>) -> i32 {
    /**
     Given a list of strings, representing the puzzle input, sum up consecutive
     values, sort in ascending order, and return the highest of these sums.
     
     This represents the most calories worth of food carried by any elf.
     See Part 1 of https://adventofcode.com/2022/day/1
     */
    let mut result: Vec<i32> = Vec::new();
    let mut accumulator: i32 = 0;
    for each_line in lines {
        if each_line.trim() == "" {
            result.push(accumulator);
            accumulator = 0;
        }else {
            accumulator += i32::from_str(each_line.trim()).unwrap();
        }
    }
    result.sort();
    result[result.len() - 1]
}

fn process_lines2(lines: &Vec<String>) -> i32 {
    /**
     Similar to process_lines(), except for part 2 we want to add together 
     the three largest sums.

     This represents the sum of the calories carried by the three elves 
     carrying the most calories.
     See Part 1 of https://adventofcode.com/2022/day/1
     */
    let mut result: Vec<i32> = Vec::new();
    let mut accumulator: i32 = 0;
    for each_line in lines {
        if each_line.trim() == "" {
            result.push(accumulator);
            accumulator = 0;
        }else {
            accumulator += i32::from_str(each_line.trim()).unwrap();
        }
    }
    result.sort();
    result[result.len() - 3] + result[result.len() - 2] + result[result.len() - 1]
}

pub fn main() {
    let result = read_lines("day01_input.txt");
    println!("Part 1 - The most calories being carried by an elf is: {}", process_lines(&result));
    println!("Part 2 - The sum of the calories carried by the top three elves is: {}", process_lines2(&result));
}
