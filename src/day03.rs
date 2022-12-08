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
     Given a list of strings, representing the contents of the elves'
     rucksacks, add up the priorities of the rucksacks.
     
     See Part 1 of https://adventofcode.com/2022/day/3
     */
    let mut priority_sum: i32 = 0;
    for each_line in lines {
        let (half1, half2) = each_line.split_at(each_line.len() / 2);
        //println!("First half is: {}", half1);
        //println!("Second half is: {}", half2);
        let unique: HashSet<char> = half1.chars().collect();
        let mut common: char = 'a';
        for each in unique.intersection(&half2.chars().collect::<HashSet<char>>()) {
            common = *each;
            break;
        }
        let mut score: i32 = i32::try_from(common as u32).unwrap();
        if score < i32::try_from('a' as u32).unwrap() {
            score = score + 27 - i32::try_from('A' as u32).unwrap();
        }else {
            score = score + 1 - i32::try_from('a' as u32).unwrap();
        }
        //println!("Shared character is {}, with a score of {}.", common, score);
        priority_sum += score;
    }
    priority_sum
}

fn process_lines2(lines: &Vec<String>) -> i32 {
    /*
     Given a list of strings, representing the contents of the elves'
     rucksacks, find the shared items between every group of three 
     rucksacks. Add up the priorities of these items.
     
     See Part 2 of https://adventofcode.com/2022/day/3
     */
    let mut priority_sum: i32 = 0;
    for each_group in lines.rchunks(3) {
        let mut lines: Vec<String> = Vec::new();
        lines.push(each_group[0].clone());
        lines.push(each_group[1].clone());
        lines.push(each_group[2].clone());
        let unique: HashSet<char> = lines[0].chars().collect();
        let mut common: char = ' ';
        for each_common in unique.intersection(&lines[1].chars().collect::<HashSet<char>>()) {
            if lines[2].chars().collect::<Vec<char>>().contains(each_common) {
                common = *each_common;
            }
        }
        let mut score: i32 = i32::try_from(common as u32).unwrap();
        if score < i32::try_from('a' as u32).unwrap() {
            score = score + 27 - i32::try_from('A' as u32).unwrap();
        }else {
            score = score + 1 - i32::try_from('a' as u32).unwrap();
        }
        //println!("Shared character is {}, with a score of {}.", common, score);
        priority_sum += score;
    }
    priority_sum
}

pub fn main() {
    let result = read_lines("day03_input.txt");
    println!("Day 3:");
    println!("Part 1 - The sum of the priorities of these items is: {}", process_lines(&result));
    println!("Part 2 - The sum of the badge priorities is: {}", process_lines2(&result));
    println!("");
}
