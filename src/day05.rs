use std::fs;
use std::str::FromStr;
use regex::Regex;

fn read_lines(filename: &str) -> (Vec<String>, Vec<String>) {
    /*
    Open a text file and return a tuple of two Vectors of Strings representing
    the stacks of crates and the desired moves.
    */
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file.");
    let mut lines: Vec<String> = Vec::new();
    let mut moves: Vec<String> = Vec::new();
    let mut half1: bool = true;
    for each in contents.split_terminator("\n").collect::<Vec<&str>>() {
        if half1 {
            if each != "" {
                lines.push(each.to_string());
            }else {
                half1 = false;
            }
        }else {
            moves.push(each.to_string());
        }
    }
    (lines, moves)
}

fn process_lines(lines: &Vec<String>, moves: &Vec<String>) -> String {
    /*
     Given an initial arrangement of crates and a list of movements for them,
     return the list of crates on top of each stack.
     
     See Part 1 of https://adventofcode.com/2022/day/5
     */
    let mut return_value: Vec<char> = Vec::new();
    let mut stacks: Vec<Vec<char>> = Vec::new();
    for _index in 0..((lines[0].len() - 3) / 4)+1 {
        stacks.push(Vec::new());
    } 
    // populate the initial states of the stacks
    for each_line in &lines[0..lines.len()-1] {
        for each_stack in 0..stacks.len() {
            //println!("Trying to add {} to stack {}", each_line, each_stack);
            let this_crate: char = each_line.chars().nth((each_stack*4)+1).unwrap();
            if this_crate != ' ' {
                stacks[each_stack].push(this_crate);
            }
        }
    }
    // reverse the stacks
    for each_stack in 0..stacks.len() {
        stacks[each_stack].reverse();
    }
    // print the stacks
    /*for each_stack in 0..stacks.len() {
        println!("Stack {}: {}", each_stack, stacks[each_stack].iter().collect::<String>());
    }*/
    // parse the move set
    let re = Regex::new(r"^\s*move\s+(\d+)\s+from\s+(\d+)\s+to\s+(\d+)\s*$").unwrap();
    let mut move_tuples: Vec<(i32, i32, i32)> = Vec::new();
    for each_move in moves {
        let move_captures = re.captures(each_move).unwrap();
        move_tuples.push((i32::from_str(move_captures.get(1).unwrap().as_str()).unwrap(),
        i32::from_str(move_captures.get(2).unwrap().as_str()).unwrap()-1,
        i32::from_str(move_captures.get(3).unwrap().as_str()).unwrap()-1));
        continue;
    }
    // carry out the moves
    for each_move in move_tuples {
        let mut temp_stack: Vec<char> = Vec::new();
        for _index in 0..each_move.0 {
            let value = stacks[each_move.1 as usize].pop().unwrap();
            //println!("Adding value {}", value);
            temp_stack.push(value);
        }
        for each_crate in 0..temp_stack.len() {
            stacks[each_move.2 as usize].push(temp_stack[each_crate]);
            //println!("{}", each_crate);
        }
    }

    // print the stacks again
    /*for each_stack in 0..stacks.len() {
        println!("Stack {}: {}", each_stack, stacks[each_stack].iter().collect::<String>());
    }*/

    // reverse the stacks, so that the top crate is at index 0 on each stack
    for each_stack in 0..stacks.len() {
        stacks[each_stack].reverse();
    }

    // calculate the final return value
    for each_stack in 0..stacks.len() {
        if stacks[each_stack].len() > 0 {
            return_value.push(stacks[each_stack][0]);
        }
    }

    return_value.iter().collect::<String>()
}

fn process_lines2(lines: &Vec<String>, moves: &Vec<String>) -> String {
    /*
     This is the same as process_lines(), except when multiple crates are moved
     in the same step, they're moved together without changing their order.
     
     See Part 2 of https://adventofcode.com/2022/day/5
     */
    let mut return_value: Vec<char> = Vec::new();
    let mut stacks: Vec<Vec<char>> = Vec::new();
    for _index in 0..((lines[0].len() - 3) / 4)+1 {
        stacks.push(Vec::new());
    } 
    // populate the initial states of the stacks
    for each_line in &lines[0..lines.len()-1] {
        for each_stack in 0..stacks.len() {
            //println!("Trying to add {} to stack {}", each_line, each_stack);
            let this_crate: char = each_line.chars().nth((each_stack*4)+1).unwrap();
            if this_crate != ' ' {
                stacks[each_stack].push(this_crate);
            }
        }
    }
    // reverse the stacks
    for each_stack in 0..stacks.len() {
        stacks[each_stack].reverse();
    }
    // parse the move set
    let re = Regex::new(r"^\s*move\s+(\d+)\s+from\s+(\d+)\s+to\s+(\d+)\s*$").unwrap();
    let mut move_tuples: Vec<(i32, i32, i32)> = Vec::new();
    for each_move in moves {
        let move_captures = re.captures(each_move).unwrap();
        move_tuples.push((i32::from_str(move_captures.get(1).unwrap().as_str()).unwrap(),
        i32::from_str(move_captures.get(2).unwrap().as_str()).unwrap()-1,
        i32::from_str(move_captures.get(3).unwrap().as_str()).unwrap()-1));
        continue;
    }
    // carry out the moves
    for each_move in move_tuples {
        // get the crates to be moved, without changing their order
        let temp_stack: Vec<char> = stacks[each_move.1 as usize][stacks[each_move.1 as usize].len()-each_move.0 as usize..].to_vec();
        // add to destination stack
        for each_crate in 0..temp_stack.len() {
            stacks[each_move.2 as usize].push(temp_stack[each_crate]);
            //println!("{}", each_crate);
        }
        // remove from origin stack
        stacks[each_move.1 as usize] = stacks[each_move.1 as usize][..stacks[each_move.1 as usize].len()-each_move.0 as usize].to_vec();
    }

    // print the stacks again
    for each_stack in 0..stacks.len() {
        println!("Stack {}: {}", each_stack, stacks[each_stack].iter().collect::<String>());
    }

    // reverse the stacks, so that the top crate is at index 0 on each stack
    for each_stack in 0..stacks.len() {
        stacks[each_stack].reverse();
    }

    // calculate the final return value
    for each_stack in 0..stacks.len() {
        if stacks[each_stack].len() > 0 {
            return_value.push(stacks[each_stack][0]);
        }
    }

    return_value.iter().collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_lines_short() {
        let (lines, moves) = read_lines("day05_input_short.txt");
        assert_eq!(process_lines(&lines, &moves),
    "CMZ".to_string());
    }

    #[test]
    fn test_process_lines_short_step01() {
        let (lines, _moves) = read_lines("day05_input_short.txt");
        let moves: Vec<String> = vec!["move 1 from 2 to 1".to_string()];
        assert_eq!(process_lines(&lines, &moves),
    "DCP".to_string());
    }

    #[test]
    fn test_process_lines_short_step02() {
        let (lines, _moves) = read_lines("day05_input_short.txt");
        let moves: Vec<String> = vec!["move 1 from 2 to 1".to_string(), "move 3 from 1 to 3".to_string()];
        assert_eq!(process_lines(&lines, &moves),
    "CZ".to_string());
    }

    #[test]
    fn test_process_lines_short_step03() {
        let (lines, _moves) = read_lines("day05_input_short.txt");
        let moves: Vec<String> = vec!["move 1 from 2 to 1".to_string(), "move 3 from 1 to 3".to_string(), "move 2 from 2 to 1".to_string()];
        assert_eq!(process_lines(&lines, &moves),
    "MZ".to_string());
    }

    #[test]
    fn test_process_lines_short_step04() {
        let (lines, _moves) = read_lines("day05_input_short.txt");
        let moves: Vec<String> = vec!["move 1 from 2 to 1".to_string(), "move 3 from 1 to 3".to_string(), "move 2 from 2 to 1".to_string(),
            "move 1 from 1 to 2".to_string()];
        assert_eq!(process_lines(&lines, &moves),
    "CMZ".to_string());
    }

    #[test]
    fn test_process_lines_full() {
        let (lines, moves) = read_lines("day05_input.txt");
        assert_eq!(process_lines(&lines, &moves),
    "ZSQVCCJLL".to_string());
    }

    #[test]
    fn test_process_lines2_short() {
        let (lines, moves) = read_lines("day05_input_short.txt");
        assert_eq!(process_lines2(&lines, &moves),
    "MCD".to_string());
    }

    #[test]
    fn test_process_lines2_short_step01() {
        let (lines, _moves) = read_lines("day05_input_short.txt");
        let moves: Vec<String> = vec!["move 1 from 2 to 1".to_string()];
        assert_eq!(process_lines2(&lines, &moves),
    "DCP".to_string());
    }

    #[test]
    fn test_process_lines2_short_step02() {
        let (lines, _moves) = read_lines("day05_input_short.txt");
        let moves: Vec<String> = vec!["move 1 from 2 to 1".to_string(), "move 3 from 1 to 3".to_string()];
        assert_eq!(process_lines2(&lines, &moves),
    "CD".to_string());
    }

    #[test]
    fn test_process_lines2_short_step03() {
        let (lines, _moves) = read_lines("day05_input_short.txt");
        let moves: Vec<String> = vec!["move 1 from 2 to 1".to_string(), "move 3 from 1 to 3".to_string(), "move 2 from 2 to 1".to_string()];
        assert_eq!(process_lines2(&lines, &moves),
    "CD".to_string());
    }

    #[test]
    fn test_process_lines2_short_step04() {
        let (lines, _moves) = read_lines("day05_input_short.txt");
        let moves: Vec<String> = vec!["move 1 from 2 to 1".to_string(), "move 3 from 1 to 3".to_string(), "move 2 from 2 to 1".to_string(),
            "move 1 from 1 to 2".to_string()];
        assert_eq!(process_lines2(&lines, &moves),
    "MCD".to_string());
    }
}

pub fn main() {
    let (lines, moves) = read_lines("day05_input.txt");
    println!("Day 5:");
    println!("Part 1 - The crates that end up on top of each stack are: {}", process_lines(&lines, &moves));
    println!("Part 2 - The crates that end up on top of each stack are: {}", process_lines2(&lines, &moves));
    println!("");
}
