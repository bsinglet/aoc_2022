use std::fs;

fn read_lines(filename: &str) -> Vec<(char, char)> {
    /*
    Open a text file and return a Vector of Strings representing the individual
    lines.
    */
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file.");
    let mut lines: Vec<(char, char)> = Vec::new();
    for each in contents.split_terminator("\n").collect::<Vec<&str>>() {
        lines.push((each.chars().nth(0).unwrap(), each.chars().nth(2).unwrap()));
    }
    lines
}

fn process_lines(lines: &Vec<(char, char)>) -> i32 {
    /*
     Given a list of structs representing the move your opponent will play and
     the move you should play for each round, calculate what your score should
     be if you follow the strategy guide and it goes according to plan.
     A/X - Rock (1 point)
     B/Y - Paper (2 points)
     C/Z - Scissors (3 points)
     Score for each round = (points for the move) 
       + (0 if you lost the round, 3 for a tie, and 6 if you won the round)

     See Part 1 of https://adventofcode.com/2022/day/2
     */
    let mut score: i32 = 0;
    for (move1, move2) in lines {
        // println!("Opponent will play {} and you should respond with {}", move1, move2);
        match move1 {
            'A' => match move2 {
                // rock vs rock == draw
                'X' => score += 1 + 3,
                'Y' => score += 2 + 6,
                'Z' => score += 3 + 0,
                _   => println!("Invalid response move: {}", move2),
            },
            'B' => match move2 {
                // paper vs rock == lose
                'X' => score += 1 + 0,
                'Y' => score += 2 + 3,
                'Z' => score += 3 + 6,
                _   => println!("Invalid response move: {}", move2),
            },
            'C' => match move2 {
                // scissors vs rock == win
                'X' => score += 1 + 6,
                'Y' => score += 2 + 0,
                'Z' => score += 3 + 3,
                _   => println!("Invalid response move: {}", move2),
            },
            _ => println!("Invalid opponent move: {}", move1),
        };
    }
    score
}

fn process_lines2(lines: &Vec<(char, char)>) -> i32 {
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
        // println!("Opponent will play {} and you should respond with {}", move1, move2);
        match move1 {
            'A' => match move2 {
                // loss against rock means you played scissors
                'X' => score += 3 + 0,
                'Y' => score += 1 + 3,
                'Z' => score += 2 + 6,
                _   => println!("Invalid outcome: {}", move2),
            },
            'B' => match move2 {
                // loss against paper means you played rock
                'X' => score += 1 + 0,
                'Y' => score += 2 + 3,
                'Z' => score += 3 + 6,
                _   => println!("Invalid outcome: {}", move2),
            },
            'C' => match move2 {
                // loss against scissors means you played paper
                'X' => score += 2 + 0,
                'Y' => score += 3 + 3,
                'Z' => score += 1 + 6,
                _   => println!("Invalid outcome: {}", move2),
            },
            _ => println!("Invalid opponent move: {}", move1),
        };
    }
    score
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_lines() {
        let result = read_lines("day02_input_short.txt");
        assert_eq!(process_lines(&result), 15);
    }

    #[test]
    fn test_process_lines2() {
        let result = read_lines("day02_input_short.txt");
        assert_eq!(process_lines2(&result), 12);
    }
}

pub fn main() {
    let result = read_lines("day02_input.txt");
    println!("Day 2:");
    println!("Part 1 - Your score if you follow the strategy guide should be: {}", process_lines(&result));
    println!("Part 2 - Your score if you follow the strategy guide should be: {}", process_lines2(&result));
    println!("");
}
