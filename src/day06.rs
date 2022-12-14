use std::fs;
use std::collections::HashSet;

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
    let input = lines[0].clone();
    let mut result: i32 = 4;
    for index in 4..input.len() {
        let rolling_window: HashSet<char> = input[index-4..index].chars().collect();
        if rolling_window.len() == 4 {
            result = i32::try_from(index as u32).unwrap();
            break;
        }
    }
    result
}

fn process_lines2(lines: &Vec<String>) -> i32 {
    /*
     Calculate how many characters are read until detecting the
     start-of-message marker.

     See Part 2 of https://adventofcode.com/2022/day/6
     */
    let input = lines[0].clone();
    let mut result: i32 = 14;
    for index in 14..input.len() {
        let rolling_window: HashSet<char> = input[index-14..index].chars().collect();
        if rolling_window.len() == 14 {
            result = i32::try_from(index as u32).unwrap();
            break;
        }
    }
    result
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

    #[test]
    fn test_process_lines_full() {
        let lines = read_lines("day06_input.txt");
        assert_eq!(process_lines(&lines), 1892);
    }

    #[test]
    fn test_process_lines2_example_01() {
        let lines = read_lines("day06_input_short.txt");
        assert_eq!(process_lines2(&lines), 19);
    }

    #[test]
    fn test_process_lines2_example_02() {
        let lines = vec!["bvwbjplbgvbhsrlpgdmjqwftvncz".to_string()];
        assert_eq!(process_lines2(&lines), 23);
    }

    #[test]
    fn test_process_lines2_example_03() {
        let lines = vec!["nppdvjthqldpwncqszvftbrmjlhg".to_string()];
        assert_eq!(process_lines2(&lines), 23);
    }

    #[test]
    fn test_process_lines2_example_04() {
        let lines = vec!["nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".to_string()];
        assert_eq!(process_lines2(&lines), 29);
    }

    #[test]
    fn test_process_lines2_example_05() {
        let lines = vec!["zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".to_string()];
        assert_eq!(process_lines2(&lines), 26);
    }

    #[test]
    fn test_process_lines2_full() {
        let lines = read_lines("day06_input.txt");
        assert_eq!(process_lines2(&lines), 2313);
    }
}

pub fn main() {
    let result = read_lines("day06_input.txt");
    println!("Day 6:");
    println!("Part 1 - The number of characters processed before the start-of-packet marker is: {}", process_lines(&result));
    println!("Part 2 - The number of characters processed before the start-of-message marker is: {}", process_lines2(&result));
    println!("");
}