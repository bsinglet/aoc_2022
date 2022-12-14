use std::fs;
use std::fmt;
use std::str::FromStr;
use std::collections::VecDeque;
use regex::Regex;
use num::bigint::{BigInt, Sign};

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
    inventory: VecDeque<i128>,
    operation: OperationType,
    argument0: ArgumentType,
    argument1: ArgumentType,
    argument0_int: i128,
    argument1_int: i128,
    test_divisible_by: i128,
    true_destination: i128,
    false_destination: i128,
}

#[derive(Debug)]
pub struct Monkey2 {
    inventory: VecDeque<BigInt>,
    operation: OperationType,
    argument0: ArgumentType,
    argument1: ArgumentType,
    argument0_int: BigInt,
    argument1_int: BigInt,
    test_divisible_by: BigInt,
    true_destination: usize,
    false_destination: usize,
}

/*
 This part is overkill, but it's useful for making sure the input parsed correctly.
 */
impl fmt::Display for Monkey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result: String = "".to_string();
        result += "Carrying items: ";
        for each_item in &self.inventory {
            result += &format!("{}, ", each_item).as_str();
        }
        result += "\nOperation: new = ";
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
    let starting_line = Regex::new(r"^\s*Starting\s+items:\s+(\d+)((,\s+\d+)+)?\s*$").unwrap();
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
        monkey.inventory.push_back(i128::from_str(starting_line.captures(&lines[index]).unwrap().get(1).unwrap().as_str()).unwrap());
        if starting_line.captures(&lines[index]).unwrap().get(3).is_some() {
            //println!("{}", starting_line.captures(&lines[index]).unwrap().get(2).unwrap().as_str());
            for each_item in starting_line.captures(&lines[index]).unwrap().get(2).unwrap().as_str().split(", ") {
                if each_item.trim() != "" {
                    monkey.inventory.push_back(i128::from_str(each_item).unwrap());
                }
            }
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
            monkey.argument0_int = i128::from_str(operation_captures.get(1).unwrap().as_str()).unwrap();
        }
        monkey.argument1 = match operation_captures.get(3).unwrap().as_str() {
            "new" => ArgumentType::New,
            "old" => ArgumentType::Old,
            _     => ArgumentType::Int,
        };
        if monkey.argument1 == ArgumentType::Int {
            monkey.argument1_int = i128::from_str(operation_captures.get(3).unwrap().as_str()).unwrap();
        }
        index += 1;
        monkey.test_divisible_by = i128::from_str(test_line.captures(&lines[index]).unwrap().get(1).unwrap().as_str()).unwrap();
        index += 1;
        monkey.true_destination = i128::from_str(true_line.captures(&lines[index]).unwrap().get(1).unwrap().as_str()).unwrap();
        index += 1;
        monkey.false_destination = i128::from_str(false_line.captures(&lines[index]).unwrap().get(1).unwrap().as_str()).unwrap();
        // skip the blank line between monkeys
        monkeys.push(monkey);
        index += 2;
    }
    monkeys
}

fn parse_lines2(lines: &Vec<String>) -> Vec<Monkey2> {
    let mut monkeys: Vec<Monkey2> = Vec::new();

    let monkey_line = Regex::new(r"^\s*Monkey\s+(\d+):\s*$").unwrap();
    let starting_line = Regex::new(r"^\s*Starting\s+items:\s+(\d+)((,\s+\d+)+)?\s*$").unwrap();
    let operation_line = Regex::new(r"^\s*Operation:\s*new\s+=\s+(\w+)\s+(\+|\*|-|/)\s+(\w+)\s*$").unwrap();
    let test_line = Regex::new(r"^\s*Test:\s*divisible\s+by\s+(\d+)\s*$").unwrap();
    let true_line = Regex::new(r"^\s*If\s+true:\s+throw\s+to\s+monkey\s+(\d+)\s*$").unwrap();
    let false_line = Regex::new(r"^\s*If\s+false:\s+throw\s+to\s+monkey\s+(\d+)\s*$").unwrap();

    // initialize all the monkeys
    let mut index: usize = 0;
    while index < lines.len() {
        let mut monkey: Monkey2 = Monkey2{
            inventory: VecDeque::new(),
            operation: OperationType::Plus,
            argument0: ArgumentType::Old,
            argument1: ArgumentType::Old,
            argument0_int: BigInt::from_bytes_be(Sign::Minus, b"1"),
            argument1_int: BigInt::from_bytes_be(Sign::Minus, b"1"),
            test_divisible_by: BigInt::from_bytes_be(Sign::Minus, b"1"),
            true_destination: 0,
            false_destination: 0,
        };
        if !monkey_line.is_match(&lines[index]) {
            eprintln!("Expected monkey line at line {}", index);
            break;
        }
        index += 1;
        monkey.inventory.push_back(BigInt::parse_bytes(starting_line.captures(&lines[index]).unwrap().get(1).unwrap().as_str().as_bytes(), 10).unwrap());
        if starting_line.captures(&lines[index]).unwrap().get(3).is_some() {
            //println!("{}", starting_line.captures(&lines[index]).unwrap().get(2).unwrap().as_str());
            for each_item in starting_line.captures(&lines[index]).unwrap().get(2).unwrap().as_str().split(", ") {
                if each_item.trim() != "" {
                    monkey.inventory.push_back(BigInt::parse_bytes(each_item.as_bytes(), 10).unwrap());
                }
            }
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
            monkey.argument0_int = BigInt::parse_bytes(operation_captures.get(1).unwrap().as_str().as_bytes(), 10).unwrap();
        }
        monkey.argument1 = match operation_captures.get(3).unwrap().as_str() {
            "new" => ArgumentType::New,
            "old" => ArgumentType::Old,
            _     => ArgumentType::Int,
        };
        if monkey.argument1 == ArgumentType::Int {
            monkey.argument1_int = BigInt::parse_bytes(operation_captures.get(3).unwrap().as_str().as_bytes(), 10).unwrap();
        }
        index += 1;
        monkey.test_divisible_by = BigInt::parse_bytes(test_line.captures(&lines[index]).unwrap().get(1).unwrap().as_str().as_bytes(), 10).unwrap();
        index += 1;
        monkey.true_destination = usize::from_str(true_line.captures(&lines[index]).unwrap().get(1).unwrap().as_str()).unwrap();
        index += 1;
        monkey.false_destination = usize::from_str(false_line.captures(&lines[index]).unwrap().get(1).unwrap().as_str()).unwrap();
        // skip the blank line between monkeys
        monkeys.push(monkey);
        index += 2;
    }
    monkeys
}

fn process_lines(lines: &Vec<String>) -> i128 {
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
    let mut times_inspected_items: Vec<i128> = Vec::new();
    // initialize the list of monkeys using the challenge input
    let mut monkeys: Vec<Monkey> = parse_lines(&lines);
    for _ in 0..monkeys.len() {
        times_inspected_items.push(0);
    }

    // simulate 20 rounds of monkey business
    for _round in 0..20 {
        //println!("Round: {}", round);
        for monkey_index in 0..monkeys.len() {
            //println!("Simulating Monkey {}", monkey_index);
            while monkeys[monkey_index].inventory.len() > 0 {
                let mut worry_level: i128 = monkeys[monkey_index].inventory.pop_front().unwrap();
                times_inspected_items[monkey_index] += 1;
                // apply operation to worry level
                let argument_0: i128;
                let argument_1: i128;
                if monkeys[monkey_index].argument0 == ArgumentType::Old {
                    argument_0 = worry_level;
                }else {
                    argument_0 = monkeys[monkey_index].argument0_int;
                }
                if monkeys[monkey_index].argument1 == ArgumentType::Old {
                    argument_1 = worry_level;
                }else {
                    argument_1 = monkeys[monkey_index].argument1_int;
                }
                worry_level = match monkeys[monkey_index].operation {
                    OperationType::Plus => argument_0 + argument_1,
                    OperationType::Subtract => argument_0 - argument_1,
                    OperationType::Times => argument_0 * argument_1,
                    OperationType::Divide => argument_0 / argument_1,
                };
                // worry level divides by three after inspection and before testing
                worry_level = (f64::from(worry_level as i32) / 3.0).floor() as i128;
                // apply the monkey test to figure out which monkey to send the value to
                let destination: usize;
                if worry_level % monkeys[monkey_index].test_divisible_by == 0 {
                    destination = monkeys[monkey_index].true_destination as usize;
                }else {
                    destination = monkeys[monkey_index].false_destination as usize;
                }
                // send the item to that monkey
                monkeys[destination].inventory.push_back(worry_level);
            }
        }
    }

    times_inspected_items.sort();
    times_inspected_items.reverse();
    times_inspected_items[0] * times_inspected_items[1]
}

fn my_lcm(mut divisors: Vec<BigInt>) -> BigInt {
    divisors.sort();
    let mut multiplicand: BigInt = BigInt::parse_bytes("1".as_bytes(), 10).unwrap();
    let mut all_matched: bool;
    loop {
        all_matched = true;
        for index in 1..divisors.len() {
            if (&divisors[0] * &multiplicand) % &divisors[index] != BigInt::parse_bytes("0".as_bytes(), 10).unwrap() {
                all_matched = false;
                break;
            }
        }
        if all_matched {
            break;
        }
        multiplicand += BigInt::parse_bytes("1".as_bytes(), 10).unwrap();
    }
    &divisors[0] * multiplicand
}

fn process_lines2(lines: &Vec<String>) -> i128 {
    /*
    The same as process_lines(), except the worry level never gets divided by
    3, and we simulate for 10000 rounds instead of 20 rounds.

    See Part 2 of https://adventofcode.com/2022/day/11
    */
    let mut times_inspected_items: Vec<i128> = Vec::new();
    let mut divisors: Vec<BigInt> = Vec::new();
    let modulus_limit: BigInt;
    // initialize the list of monkeys using the challenge input
    let mut monkeys: Vec<Monkey2> = parse_lines2(&lines);
    for index in 0..monkeys.len() {
        times_inspected_items.push(0);
        divisors.push(monkeys[index].test_divisible_by.clone());
    }

    // if we just let the worry level grow unrestricted, we'll end up with 
    // numbers on the order of 10^10_000. But we don't need to know the worry
    // levels, we just need to know which divisibility tests they satisfy. We 
    // can do this by finding the least common multiple of all the divisors, 
    // and taking the modulus of the worry level and that LCM.
    modulus_limit = my_lcm(divisors);
    println!("Modulus limit is: {}", modulus_limit);

    // simulate 10000 rounds of monkey business
    for _round in 0..10000 {
        for monkey_index in 0..monkeys.len() {
            //println!("Simulating Monkey {}", monkey_index);
            while monkeys[monkey_index].inventory.len() > 0 {
                let mut worry_level: BigInt = monkeys[monkey_index].inventory.pop_front().unwrap();
                times_inspected_items[monkey_index] += 1;
                // apply operation to worry level
                let argument_0: BigInt;
                let argument_1: BigInt;
                if monkeys[monkey_index].argument0 == ArgumentType::Old {
                    argument_0 = worry_level.clone();
                }else {
                    argument_0 = monkeys[monkey_index].argument0_int.clone();
                }
                if monkeys[monkey_index].argument1 == ArgumentType::Old {
                    argument_1 = worry_level.clone();
                }else {
                    argument_1 = monkeys[monkey_index].argument1_int.clone();
                }
                worry_level = match monkeys[monkey_index].operation {
                    OperationType::Plus => argument_0 + argument_1,
                    OperationType::Subtract => argument_0 - argument_1,
                    OperationType::Times => {
                        argument_0 * argument_1
                    },
                    OperationType::Divide => argument_0 / argument_1,
                };
                // we no longer divide the worry level by three
                // but we can keep the worry levels from going to the 10,000th 
                // power by using our modulus_limit value
                worry_level = worry_level % &modulus_limit;
                // apply the monkey test to figure out which monkey to send the value to
                let destination: usize;
                if &worry_level % monkeys[monkey_index].test_divisible_by.clone() == BigInt::parse_bytes("0".as_bytes(), 10).unwrap() {
                    destination = monkeys[monkey_index].true_destination as usize;
                }else {
                    destination = monkeys[monkey_index].false_destination as usize;
                }
                // send the item to that monkey
                monkeys[destination].inventory.push_back(worry_level);
            }
        }
    }

    times_inspected_items.sort();
    times_inspected_items.reverse();
    times_inspected_items[0] * times_inspected_items[1]
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
        assert_eq!(process_lines(&lines), 108240);
    }

    #[test]
    fn test_process_lines2_short() {
        let lines = read_lines("day11_input_short.txt");
        assert_eq!(process_lines2(&lines), 2713310158);
    }

    #[test]
    fn test_process_lines2_full() {
        let lines = read_lines("day11_input.txt");
        assert_eq!(process_lines2(&lines), 25712998901);
    }
}

pub fn main() {
    let result = read_lines("day11_input.txt");
    println!("Day 11:");
    println!("Part 1 - The level of monkey business after 20 rounds of stuff-slinging simian shenanigans is: {}", process_lines(&result));
    println!("Part 2 - The level of monkey business after 10000 rounds of stuff-slinging simian shenanigans is: {}", process_lines2(&result));
}