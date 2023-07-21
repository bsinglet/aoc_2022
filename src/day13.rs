use std::fs;
use std::collections::{VecDeque, HashMap, HashSet};

fn read_lines(filename: &str) -> Vec<String> {
    /*
    Open a text file and return a Vector of Strings representing the individual
    lines.
    */
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file.");
    let mut lines: Vec<String> = Vec::new();
    for each in contents.split_terminator("\n").filter(|x| x.len() > 0).collect::<Vec<&str>>() {
        lines.push(each.trim().to_string());
    }
    lines
}


fn process_lines(lines: &Vec<String>) -> i32 {
    let mut pair_index: i32 = 0;
    let mut pair_indices_sum: i32 = 0;

    for packet_pair in lines.chunks(2) {
        pair_index += 1;
        println!("{}",packet_pair[0]);
        println!("{}",packet_pair[1]);

        // check if the packets in this pair are correctly ordered
        // skip to the next pair if at any time you detect they are out of order
        
        // if the pairs have matched thus fair, add their index to our sum
        pair_indices_sum += pair_index;
    }

    pair_indices_sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_lines_short() {
        let lines = read_lines("day13_input_short.txt");
        assert_eq!(process_lines(&lines), 13);
    }

    #[test]
    fn test_process_lines_full() {
        let lines = read_lines("day13_input.txt");
        assert_eq!(process_lines(&lines), -1);
    }

    #[test]
    fn test_parse_input_01() {
        // very simple unit test to make sure the parse_input() function
        // correctly finds the starting and ending locations, as well as the
        // heights of each square.
    }
}

pub fn main() {
    let result = read_lines("day13_input_short.txt");
    println!("Day 13:");
    println!("Part 1 - The sum of the indices of the pairs in the right order is: {}", process_lines(&result));
}