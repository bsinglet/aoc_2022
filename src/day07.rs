use std::fs;
use regex::Regex;
use std::collections::HashMap;

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

fn parse_input(lines: &Vec<String>) -> (Vec<HashMap>, Vec<HashMap>) {
    let cd_command = Regex::new(r"^\$\s+cd\s+(\w+)").unwrap();
    let ls_command = Regex::new(r"^\$\s+ls").unwrap();
    let directory_line = Regex::new(r"^dir\s+(\d+)").unwrap();
    let file_line = Regex::new(r"^(\d+)\s+(.+)").unwrap();
    let directories: Vec<HashMap> = Vec::new();
    let files: Vec<HashMap> = Vec::new();
    let mut current_directory = "/";
    let mut current_depth = 0;
    let mut index: usize = 0;
    while index < lines.len()-1 {
        if ls_command.search(lines[index]) {
            // the next unknown many lines are files or directories inside of current_directory
            loop {
                if directory_line.search(lines[index]) {
                    let directory: HashMap<&str, &str> = HashMap::from([
                        ("parent", current_directory),
                        ("depth", current_depth),
                        ("size", "0"),
                        ("name", directory_line.captures(&lines[index]).get(1)),
                        ]);
                    directories.push(directory);
                }else if file_line.search(lines[index]) {
                    let file: HashMap<&str, &str> = HashMap::from([
                        ("parent", current_directory),
                        ("depth", current_depth),
                        ("size", file_line.captures(&lines[index]).get(1)),
                        ("name", file_line.captures(&lines[index]).get(2)),
                    ]);
                }
            }
        }else if cd_command.search(lines[index]) {
            match cd_command.captures(&lines[index]).get(1) {
                "/" => {
                    current_directory = "/";
                    current_depth = 0;
                },
                ".." => {
                    current_directory = 
                    current_directory -= 1;
                },
                _ => {
                    current_directory += "/" + cd_command.captures(&lines[index]).get(1);
                    current_depth += 1;       
                }
            }
        }else {
            println!("Unrecognized input: {}", lines[index]);
        }
        index += 1;
    }
    (directories, files)
}

fn process_lines(lines: &Vec<String>) -> i32 {
    /*
     Takes the command-line history of a device, determines what directories
     had AT MOST 100k bytes, then sums up the total size of those directories.

     See Part 1 of https://adventofcode.com/2022/day/7
     */
    // parse the input into directories and files
     let (directories, files) = parse_input(&lines);

    // calculate the sizes of all of the subdirectories, starting from the 
    // lowest levels up
    for each_file in files.sort_by_key(|x| x["depth"]).reverse() {
        // apply file sizes to directory_sizes
    }
    // add the sizes of child directories to parent directories
    for each_directory in directories.sort_by_key(|x| x["depth"]).reverse() {
        // stop at root level
        if each_directory["depth"] = 0 {
            break;
        }
        // add this directory's size to its parent directory
    }

    // sum the size of the directories that are 100kB or less
    let mut total_of_small_directories: i32 = 0;
    for each_directory in directories.sort_by_key(|&x| x["size"]).reverse() {
        if each_directory["size"] <= 100000 {
            total_of_small_directories += each_directory["size"];
        }
    }
    total_of_small_directories
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_lines_short() {
        let lines = read_lines("day07_input_short.txt");
        assert_eq!(process_lines(&lines), 7);
    }
}

pub fn main() {
    let result = read_lines("day07_input_short.txt");
    println!("Day 7:");
    println!("Part 1 - The sum of the total sizes of those directories is: {}", process_lines(&result));
}