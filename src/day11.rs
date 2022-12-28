use std::fs;
use std::fmt;
use std::str::FromStr;
use std::collections::VecDeque;
use regex::Regex;

#[derive(Debug)]
#[derive(PartialEq)]
enum OperationType {
    Plus,
    Subtract,
    Times,
    Divide,
}

impl fmt::Display for OperationType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let result;
        result = match self {
            OperationType::Plus     => "+",
            OperationType::Subtract => "-",
            OperationType::Times    => "*",
            OperationType::Divide   => "/",
        };
        write!(f, "{}", result)
    }
}

#[derive(Debug)]
#[derive(PartialEq)]
enum ArgumentType {
    New,
    Old,
    Int,
}

impl fmt::Display for ArgumentType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let result;
        result = match self {
            ArgumentType::New => "new",
            ArgumentType::Old => "old",
            ArgumentType::Int => "int",
        };
        write!(f, "{}", result)
    }
}

#[derive(Debug)]
pub struct Monkey {
    inventory: VecDeque<i32>,
    operation: OperationType,
    argument0: ArgumentType,
    argument1: ArgumentType,
    argument0_int: i32,
    argument1_int: i32,
    test_divisible_by: i32,
    true_destination: i32,
    false_destination: i32,
}

/*
 This part is overkill, but it's useful for making sure the input parsed correctly.
 */
impl fmt::Display for Monkey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result: String = "".to_string();
        result += &format!("Carrying {} items\n", self.inventory.len()).as_str();
        result += "Operation: new = ";
        if self.argument0 == ArgumentType::Int {
            result += format!("{} ", self.argument0_int).as_str();
        }else {
            result += format!("{} ", self.argument0).as_str();
        }
        result += format!("{} ", self.operation).as_str();
        if self.argument1 == ArgumentType::Int {
            result += format!("{}\n", self.argument1_int).as_str();
        }else {
            result += format!("{}\n", self.argument1).as_str();
        }
        result += &format!("Test: divisible by {}\n", self.test_divisible_by).as_str();
        result += &format!("If true: throw to monkey {}\n", self.true_destination).as_str();
        result += &format!("If false: throw to monkey {}\n\n", self.false_destination).as_str();
        write!(f, "{}", result.as_str())
    }
}

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

fn parse_lines(lines: &Vec<String>) -> Vec<Monkey> {
    let mut monkeys: Vec<Monkey> = Vec::new();

    let monkey_line = Regex::new(r"^\s*Monkey\s+(\d+):\s*$").unwrap();
    let starting_line = Regex::new(r"^\s*Starting\sitems:\s*(\d+,?\s?)+\s*$").unwrap();
    let operation_line = Regex::new(r"^\s*Operation:\s*new\s+=\s+(\w+)\s+(\+|\*|-|/)\s+(\w+)\s*$").unwrap();
    let test_line = Regex::new(r"^\s*Test:\s*divisible\s+by\s+(\d+)\s*$").unwrap();
    let true_line = Regex::new(r"^\s*If\s+true:\s+throw\s+to\s+monkey\s+(\d+)\s*$").unwrap();
    let false_line = Regex::new(r"^\s*If\s+false:\s+throw\s+to\s+monkey\s+(\d+)\s*$").unwrap();

    // initialize all the monkeys
    let mut index: usize = 0;
    while index < lines.len() {
        let mut monkey: Monkey = Monkey{
            inventory: VecDeque::new(),
            operation: OperationType::Plus,
            argument0: ArgumentType::Old,
            argument1: ArgumentType::Old,
            argument0_int: -1,
            argument1_int: -1,
            test_divisible_by: -1,
            true_destination: -1,
            false_destination: -1,
        };
        if !monkey_line.is_match(&lines[index]) {
            eprintln!("Expected monkey line at line {}", index);
            break;
        }
        index += 1;
        for each_item in starting_line.captures(&lines[index]).unwrap().get(1).unwrap().as_str().split(",") {
            monkey.inventory.push_back(i32::from_str(each_item).unwrap());
        }
        index +=1;
        let operation_captures = operation_line.captures(&lines[index]).unwrap();
        monkey.operation = match operation_captures.get(2).unwrap().as_str() {
            "+" => OperationType::Plus,
            "-" => OperationType::Subtract,
            "*" => OperationType::Times,
            "/" => OperationType::Divide,
            _   => OperationType::Plus,
        };
        monkey.argument0 = match operation_captures.get(1).unwrap().as_str() {
            "new" => ArgumentType::New,
            "old" => ArgumentType::Old,
            _     => ArgumentType::Int,
        };
        if monkey.argument0 == ArgumentType::Int {
            monkey.argument0_int = i32::from_str(operation_captures.get(1).unwrap().as_str()).unwrap();
        }
        monkey.argument1 = match operation_captures.get(3).unwrap().as_str() {
            "new" => ArgumentType::New,
            "old" => ArgumentType::Old,
            _     => ArgumentType::Int,
        };
        if monkey.argument1 == ArgumentType::Int {
            monkey.argument1_int = i32::from_str(operation_captures.get(3).unwrap().as_str()).unwrap();
        }
        index += 1;
        monkey.test_divisible_by = i32::from_str(test_line.captures(&lines[index]).unwrap().get(1).unwrap().as_str()).unwrap();
        index += 1;
        monkey.true_destination = i32::from_str(true_line.captures(&lines[index]).unwrap().get(1).unwrap().as_str()).unwrap();
        index += 1;
        monkey.false_destination = i32::from_str(false_line.captures(&lines[index]).unwrap().get(1).unwrap().as_str()).unwrap();
        // skip the blank line between monkeys
        monkeys.push(monkey);
        index += 2;
    }
    monkeys
}

fn process_lines(lines: &Vec<String>) -> i32 {
    /*
    Determines the level of monkey business after 20 rounds of simulation. Each
    monkey starts with a certain number of items. The items only have a single
    property: their worry level. Each round, you evaluate each monkey in 
    numerical order. Each monkey inspects all of its items, in order, and 
    follows its own rules to change each items' worry lvel, and then decides
    which monkey to pass each item to.

    The level of monkey business is the number of times the two most active 
    monkeys inspected items.

    See Part 1 of https://adventofcode.com/2022/day/11
    */
    let mut times_inspected_items: Vec<i32> = Vec::new();
    // initialize the list of monkeys using the challenge input
    let mut monkeys: Vec<Monkey> = parse_lines(&lines);

    // simulate 20 rounds of monkey business
    for round in 0..20 {
        println!("Round: {}", round);
        for monkey_index in 0..monkeys.len() {
            println!("Simulating Monkey {}", monkey_index);
        }
    }

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_lines_short() {
        let lines = read_lines("day11_input_short.txt");
        assert_eq!(process_lines(&lines), 10605);
    }

    #[test]
    fn test_process_lines_full() {
        let lines = read_lines("day11_input.txt");
        assert_eq!(process_lines(&lines), -1);
    }
}

pub fn main() {
    let result = read_lines("day11_input_short.txt");
    println!("Day 11:");
    println!("Part 1 - The level of monkey business after 20 rounds of");
    println!("stuff-slinging simian shenanigans is: {}", process_lines(&result));
}