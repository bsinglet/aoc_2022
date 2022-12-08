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
     Given a list of pairs of ranges, count how many ranges in the same pair 
     fully contain each other.
     
     See Part 1 of https://adventofcode.com/2022/day/4
     */
    let mut fully_contained: i32 = 0;
    for each_line in lines {
        let commas: String = each_line.replace("-", ",");
        let mut my_strings: Vec<&str> = commas.split(",").collect();
        let mut numbers: Vec<i32> = Vec::new();
        for each in my_strings {
            numbers.push(i32::from_str(each).unwrap());
        }
        if numbers[1] - numbers[0] > numbers[3] - numbers[2] {
            // the first range is bigger, check if it contains the second range
            if numbers[0] <= numbers[2] && numbers[1] >= numbers[3] {
                println!("({},{}) fully contains ({},{})", numbers[0], numbers[1], numbers[2], numbers[3]);
                fully_contained += 1;
            }
        }else {
            // the second range is bigger.
            if numbers[2] <= numbers[0] && numbers[3] >= numbers[1] {
                println!("({},{}) fully contains ({},{})", numbers[2], numbers[3], numbers[0], numbers[1]);
                fully_contained += 1;
            }
        }
    }
    fully_contained
}

fn ranges_overlap(a: i32, b: i32, c: i32, d: i32) -> i32 {
    let mut contained = 0;
    if (a <= c && c <= b) ||
        (a <= d && d <= b) ||
        (c <= a && a <= d) ||
        (c <= b && b <= d) {
            println!("({}, {}) and ({}, {}) overlap", a, b, c, d);
            contained += 1;
        }
    contained
}

fn process_lines2(lines: &Vec<String>) -> i32 {
    /*
     Given a list of pairs of ranges, count how many pairs overlap at all.
     
     See Part 2 of https://adventofcode.com/2022/day/4
     */
    let mut contained: i32 = 0;
    let mut pairs: Vec<Vec<i32>> = Vec::new();
    for each_line in lines {
        let commas: String = each_line.replace("-", ",");
        let mut my_strings: Vec<&str> = commas.split(",").collect();
        let mut numbers: Vec<i32> = Vec::new();
        for each in my_strings {
            numbers.push(i32::from_str(each).unwrap());
        }
        pairs.push(numbers);
    }

    for (index, each_pair) in pairs.iter().enumerate() {
        let (range_1_start, range_1_end, range_2_start, range_2_end) 
            = (pairs[index][0], pairs[index][1], pairs[index][2], pairs[index][3]);
        for other_index in index+1..pairs.len() {
            let (a, b, c, d) = (pairs[other_index][0], pairs[other_index][1], 
                pairs[other_index][2], pairs[other_index][3]);
            println!("Before: {}", contained);
            contained += (ranges_overlap(range_1_start, range_1_end, a, b) | ranges_overlap(range_1_start, range_1_end, c, d)
                | ranges_overlap(range_2_start, range_2_end, a, b) | ranges_overlap(range_2_start, range_2_end, c, d));
            println!("After: {}", contained);
        }
        /*
        if numbers[1] - numbers[0] > numbers[3] - numbers[2] {
            // the first range is bigger, check if it contains the second range
            if numbers[0] <= numbers[2] && numbers[1] >= numbers[3] {
                println!("({},{}) fully contains ({},{})", numbers[0], numbers[1], numbers[2], numbers[3]);
                fully_contained += 1;
            }
        }else {
            // the second range is bigger.
            if numbers[2] <= numbers[0] && numbers[3] >= numbers[1] {
                println!("({},{}) fully contains ({},{})", numbers[2], numbers[3], numbers[0], numbers[1]);
                fully_contained += 1;
            }
        }*/
    }
    contained
}

pub fn main() {
    let result = read_lines("day04_input.txt");
    println!("Day 4:");
    println!("Part 1 - The number of fully contained ranges is: {}", process_lines(&result));
    //println!("Part 2 - The number of pairs that overlap at all is: {}", process_lines2(&result));
    println!("");
}
