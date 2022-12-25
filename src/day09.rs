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

fn calculate_distance(head_x_pos: i32, head_y_pos: i32, tail_x_pos: i32, tail_y_pos: i32) -> i32 {
    let mut distance: i32  = 0;
    distance = (tail_x_pos - head_x_pos).pow(2) + (tail_y_pos - head_y_pos).pow(2);
    distance = (distance as f32).sqrt() as i32;
    distance
}

fn move_tail(head_x_pos: i32, head_y_pos: i32, tail_x_pos: i32, tail_y_pos: i32) -> (i32, i32, i32, i32) {
    let mut results = (0, 0, 0, 0);
    if calculate_distance(head_x_pos, head_y_pos, tail_x_pos, tail_y_pos) > 1 {
        // move diagonally
        if head_x_pos > tail_x_pos {
            results.2 = tail_x_pos + 1;
        }else {
            results.2 = tail_x_pos - 1;
        }
        if head_y_pos > tail_y_pos {
            results.3 = tail_y_pos + 1;
        }else {
            results.3 = tail_y_pos - 1;
        }
    }else {
        if head_x_pos > tail_x_pos {
            results.2 = tail_x_pos + 1;
        }else if head_x_pos < tail_x_pos {
            results.2 = tail_x_pos - 1;
        }else if head_y_pos > tail_y_pos {
            results.3 = tail_y_pos + 1;
        }else if head_y_pos < tail_y_pos {
            results.3 = tail_y_pos - 1;
        }
    }
    results
}

fn process_lines(lines: &Vec<String>) -> i32 {
    /*
    Counts the number of positions the tail visits at least once.

    See Part 1 of https://adventofcode.com/2022/day/9
    */
    let mut visited_positions: HashSet<(i32, i32)> = HashSet::new();
    let mut head_x_pos: i32 = 0;
    let mut head_y_pos: i32 = 0;
    let mut tail_x_pos: i32 = 0;
    let mut tail_y_pos: i32 = 0;
    for each_instruction in lines {
        let direction = each_instruction.split(" ").next().unwrap();
        let distance = i32::from_str(each_instruction.split(" ").skip(1).next().unwrap()).unwrap();
        println!("{} {}", direction, distance);
        for _move_index in 0..distance {
            match direction {
                "U" => head_y_pos -= 1,
                "D" => head_y_pos += 1,
                "L" => head_x_pos -= 1,
                "R" => head_x_pos += 1,
                _ => println!("Unrecognized direction {}", direction),
            };

            println!("New head position ({}, {})", head_x_pos, head_y_pos);

            // tail needs to catch up with head
            (head_x_pos, head_y_pos, tail_x_pos, tail_y_pos) = move_tail(head_x_pos, head_y_pos, tail_x_pos, tail_y_pos);

            // mark tail's current position as visited
            visited_positions.insert((tail_x_pos, tail_y_pos));
        }
    }

    visited_positions.len().try_into().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_lines_short() {
        let lines = read_lines("day09_input_short.txt");
        assert_eq!(process_lines(&lines), 13);
    }

    /*
    #[test]
    fn test_process_lines_full() {
        let lines = read_lines("day09_input.txt");
        assert_eq!(process_lines(&lines), 1533);
    }*/

    #[test]
    fn test_calculate_distance_01() {
        assert_eq!(calculate_distance(0, 0, 1, 1), 1);
    }

    #[test]
    fn test_calculate_distance_02() {
        assert_eq!(calculate_distance(-1, -1, 1, 1), 2);
    }

    #[test]
    fn test_move_tail_01() {
        assert_eq!(move_tail(0, 0, 0, 0), (0, 0, 0, 0));
    }

    #[test]
    fn test_move_tail_02() {
        assert_eq!(move_tail(0, 1, 0, 0), (0, 1, 0, 1));
    }

    #[test]
    fn test_move_tail_03() {
        assert_eq!(move_tail(1, 1, 0, 0), (1, 1, 1, 1));
    }

    #[test]
    fn test_move_tail_04() {
        assert_eq!(move_tail(5, 0, 0, 0), (5, 0, 1, 0));
    }

    #[test]
    fn test_move_tail_05() {
        assert_eq!(move_tail(0, 5, 0, 0), (0, 5, 0, 1));
    }

    #[test]
    fn test_move_tail_06() {
        assert_eq!(move_tail(0, 5, 0, -5), (0, 5, 0, -4));
    }

    #[test]
    fn test_move_tail_07() {
        assert_eq!(move_tail(0, -5, 0, 5), (0, -5, 0, 4));
    }
}

pub fn main() {
    let result = read_lines("day09_input_short.txt");
    println!("Day 9:");
    println!("Part 1 - The number of positions the tail visits at least once is: {}", process_lines(&result));
}