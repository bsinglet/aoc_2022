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
        lines.push(each.to_string());
    }
    lines
}

fn process_lines(lines: &Vec<String>) -> i32 {
    /*
     Given a list of strings, representing the puzzle input, sum up consecutive
     values, sort in ascending order, and return the highest of these sums.
     
     This represents the most calories worth of food carried by any elf.
     See Part 1 of https://adventofcode.com/2022/day/1
     */
    let mut result: Vec<i32> = Vec::new();
    let mut accumulator: i32 = 0;
    for each_line in lines {
        // when we hit a blank line, we're done with the current elf, so add
        // the running sum to the Vector
        if each_line.trim() == "" {
            result.push(accumulator);
            accumulator = 0;
        }else {
            accumulator += i32::from_str(each_line.trim()).unwrap();
        }
    }
    result.sort();
    // return the highst sum in the list
    result[result.len() - 1]
}

fn process_lines2(lines: &Vec<String>) -> i32 {
    /*
     Similar to process_lines(), except for part 2 we want to add together 
     the three largest sums.

     This represents the sum of the calories carried by the three elves 
     carrying the most calories.
     See Part 1 of https://adventofcode.com/2022/day/1
     */
    let mut result: Vec<i32> = Vec::new();
    let mut accumulator: i32 = 0;
    for each_line in lines {
        // when we hit a blank line, we're done with the current elf, so add
        // the running sum to the Vector
        if each_line.trim() == "" {
            result.push(accumulator);
            accumulator = 0;
        }else {
            accumulator += i32::from_str(each_line.trim()).unwrap();
        }
    }
    // add the result from the last elf
    if accumulator > 0 {
        result.push(accumulator);
    }
    result.sort();
    // return the sum of the three highst sums in the list
    result[result.len() - 3] + result[result.len() - 2] + result[result.len() - 1]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_lines() {
        let result = read_lines("day01_input_short.txt");
        assert_eq!(process_lines(&result), 24000);
    }

    #[test]
    fn test_process_lines2() {
        let result = read_lines("day01_input_short.txt");
        assert_eq!(process_lines2(&result), 45000);
    }
}

pub fn main() {
    let result = read_lines("day01_input.txt");
    println!("Day 1:");
    println!("Part 1 - The most calories being carried by an elf is: {}", process_lines(&result));
    println!("Part 2 - The sum of the calories carried by the top three elves is: {}", process_lines2(&result));
    println!("");
}
