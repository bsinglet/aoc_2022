use std::fs;
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
     Calculate how many characters are read until detecting the
     start-of-packet marker.
     
     See Part 1 of https://adventofcode.com/2022/day/6
     */
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_lines_example_01() {
        let lines = read_lines("day06_input_short.txt");
        assert_eq!(process_lines(&lines), 7);
    }

    #[test]
    fn test_process_lines_example_02() {
        let lines = vec!["bvwbjplbgvbhsrlpgdmjqwftvncz".to_string()];
        assert_eq!(process_lines(&lines), 5);
    }

    #[test]
    fn test_process_lines_example_03() {
        let lines = vec!["nppdvjthqldpwncqszvftbrmjlhg".to_string()];
        assert_eq!(process_lines(&lines), 6);
    }

    #[test]
    fn test_process_lines_example_04() {
        let lines = vec!["nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".to_string()];
        assert_eq!(process_lines(&lines), 10);
    }

    #[test]
    fn test_process_lines_example_05() {
        let lines = vec!["zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".to_string()];
        assert_eq!(process_lines(&lines), 11);
    }
}

pub fn main() {
    let result = read_lines("day06_input_short.txt");
    println!("Day 6:");
    println!("Part 1 - The number of characters processed before the start-of-packet marker is: {}", process_lines(&result));
    println!("");
}