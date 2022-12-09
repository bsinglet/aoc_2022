use std::fs;
use std::str::FromStr;

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
    let return_value: String = ' '.to_string();
    let mut stacks: Vec<Vec<char>> = Vec::new();
    for index in 0..(lines[0].len() / 3) {
        stacks.push(Vec::new());
    } 
    // populate the initial states of the stacks
    for each_line in &lines[0..lines.len()-1] {
        for each_stack in 0..stacks.len() {
            let this_crate: char = each_line.chars().nth((each_stack*4)+1).unwrap();
            if this_crate != ' ' {
                stacks[each_stack].push(this_crate);
            }
        }
    }
    println!("Stack 0: {}", stacks[0].iter().collect::<String>());
    println!("Stack 1: {}", stacks[1].iter().collect::<String>());
    println!("Stack 2: {}", stacks[2].iter().collect::<String>());
    // parse the move set
    for each_move in moves {
        continue;
    }
    // carry out the moves

    // calculate the final return value

    return_value
}

pub fn main() {
    let (lines, moves) = read_lines("day05_input.txt");
    println!("Day 5:");
    println!("Part 1 - The crates that end up on top of each stack are: {}", process_lines(&lines, &moves));
    println!("");
}
