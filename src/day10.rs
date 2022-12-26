use std::fs;
use std::collections::HashSet;
use std::str::FromStr;

fn read_lines(filename: &str) -> Vec<String> {
    /*
    Open a text file and return a Vector of Strings representing the individual
    lines.
    */
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file.");
    let mut lines: Vec<String> = Vec::new();
    for each in contents.split_terminator("\n").collect::<Vec<&str>>() {
        lines.push(each.trim().to_string());
    }
    lines
}

fn process_lines(lines: &Vec<String>) -> i32 {
    /*
    Emulate the communicator CPU, recording the signal strength of register X
    at the 20th, 60th, 100th, 140th, 180th, and 220th cycles. This CPU only has
    two operations (noop and addx), and one register: x.

    See Part 1 of https://adventofcode.com/2022/day/10
    */
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_lines_short() {
        let lines = read_lines("day10_input_short.txt");
        assert_eq!(process_lines(&lines), 13140);
    }

    #[test]
    fn test_process_lines_full() {
        let lines = read_lines("day10_input.txt");
        assert_eq!(process_lines(&lines), -1);
    }
}

pub fn main() {
    let result = read_lines("day10_input_short.txt");
    println!("Day 10:");
    println!("Part 1 - The sum of these six signal strengths is: {}", process_lines(&result));
}