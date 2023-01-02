use std::fs;
use std::str::FromStr;
use regex::Regex;

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
    Determines the fewest number of steps required to move from the current
    position to the location with the best signal. 

    See Part 1 of https://adventofcode.com/2022/day/12
    */
    
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_lines_short() {
        let lines = read_lines("day12_input_short.txt");
        assert_eq!(process_lines(&lines), 31);
    }

    #[test]
    fn test_process_lines_full() {
        let lines = read_lines("day12_input.txt");
        assert_eq!(process_lines(&lines), -1);
    }
}

pub fn main() {
    let result = read_lines("day12_input_short.txt");
    println!("Day 12:");
    println!("Part 1 - The fewest number of steps required is: {}", process_lines(&result));
}