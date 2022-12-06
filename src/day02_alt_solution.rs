/*
Both this and day02.rs produce valid solutions for the puzzle. The difference
is that this ones uses math to make it more elegant, which would also make it
easier to scale to games like Rock, Paper, Scissors, Lizard, Spock.
*/
use std::fs;
use std::convert::TryFrom;

fn read_lines(filename: &str) -> Vec<(i32, i32)> {
    /*
    Open a text file and return a Vector of Strings representing the individual
    lines.
    */
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file.");
    let mut lines: Vec<(i32, i32)> = Vec::new();
    for each in contents.split_terminator("\n").collect::<Vec<&str>>() {
        let move1: i32 = i32::try_from(each.chars().nth(0).unwrap() as u32 - 'A' as u32).unwrap();
        let move2: i32 = i32::try_from(each.chars().nth(2).unwrap() as u32 - 'X' as u32).unwrap();
        lines.push((move1, move2));
    }
    lines
}

fn process_lines(lines: &Vec<(i32, i32)>) -> i32 {
    /*
     Given a list of structs representing the move your opponent will play and
     the move you should play for each round, calculate what your score should
     be if you follow the strategy guide and it goes according to plan.

     See Part 1 of https://adventofcode.com/2022/day/2
     */
    let mut score: i32 = 0;
    for (move1, move2) in lines {
        if move1 == move2 {
            //println!("{} ties with {}", move1, move2);
            score += (move2 + 1) + 3;
        // the math here is weird, but each move loses to the one after it. Rock
        // loses to paper, paper loses to scissors, scissors loses to rock.
        }else if (move1 + 1) % 3 == move2 % 3 {
            //println!("{} loses to {}", move1, move2);
            score += (move2 + 1) + 6;
        }else {
            //println!("{} beats with {}", move1, move2);
            score += (move2 + 1) + 0;
        }
    }
    score
}

fn process_lines2(lines: &Vec<(i32, i32)>) -> i32 {
    /*
     Given a list of structs representing the move your opponent will play and
     whether you should lose, draw, or win that round.
     Returns what your score should be if you follow the strategy guide and it
     goes according to plan.
     A - Rock (1 point)
     B - Paper (2 points)
     C - Scissors (3 points)
     X - Lose (0 points)
     Y - Draw (3 points)
     Z - Win (6 points)
     Score for each round = (points for the move) 
       + (0 if you lost the round, 3 for a tie, and 6 if you won the round)

     See Part 1 of https://adventofcode.com/2022/day/2
    */
    let mut score: i32 = 0;
    for (move1, move2) in lines {
        match move2 {
            // lose to opponent
            0 => score += ((move1 + 2) % 3) + 1 + 0,
            // tie with opponent
            1 => score += (move1 + 1) + 3,
            // win against opponent
            2 => score += ((move1 + 1) % 3) + 1 + 6,
            _ => println!("Invalid outcome: {}", move2),
        };
    }
    score
}

pub fn main() {
    let result = read_lines("day02_input.txt");
    println!("Day 2 (alternative solution):");
    println!("Part 1 - Your score if you follow the strategy guide should be: {}", process_lines(&result));
    println!("Part 2 - Your score if you follow the strategy guide should be: {}", process_lines2(&result));
    println!("");
}
