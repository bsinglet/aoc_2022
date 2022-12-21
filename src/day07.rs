use std::fs;
use std::str::FromStr;
use regex::Regex;

#[derive(Debug)]
pub struct FileOrDirectory {
    name: String,
    parent: String,
    depth: i32,
    size: i32,
    children: Vec<String>,
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

fn go_up_one_level(current_directory: &str) -> String {
    /*
    Takes a string representing a certain path, returns the path above it.
    For example, given "/usr/bin" it returns "/usr/" 
    */
    let get_parent = Regex::new(r"/(\w+/)*/\w+$").unwrap();
    get_parent.captures(current_directory).unwrap().get(1).unwrap().as_str().to_string()
}

fn go_down_one_level(current_directory: &str, sub_directory: &str) -> String {
    let result: String = current_directory.to_string() + &"/".to_string() + &sub_directory.to_string();
    result.to_string()
}

fn parse_input(lines: &Vec<String>) -> (Vec<FileOrDirectory>, Vec<FileOrDirectory>) {
    let cd_command = Regex::new(r"^\$\s+cd\s+(\w+)").unwrap();
    let ls_command = Regex::new(r"^\$\s+ls").unwrap();
    let directory_line = Regex::new(r"^dir\s+(\d+)").unwrap();
    let file_line = Regex::new(r"^(\d+)\s+(.+)").unwrap();
    let mut directories: Vec<FileOrDirectory> = Vec::new();
    let mut files: Vec<FileOrDirectory> = Vec::new();
    let mut current_directory = "/";
    let mut current_depth = 0;
    let mut index: usize = 0;
    while index < lines.len()-1 {
        if ls_command.is_match(&lines[index].as_str()) {
            // the next unknown many lines are files or directories inside of current_directory
            loop {
                if directory_line.is_match(&lines[index].as_str()) {
                    let directory = FileOrDirectory{
                        parent: current_directory.to_string(),
                        depth: current_depth,
                        size: 0,
                        name: directory_line.captures(&lines[index]).unwrap().get(1).unwrap().as_str().to_string(),
                        children: Vec::new(),
                    };
                    directories.push(directory);
                }else if file_line.is_match(&lines[index].as_str()) {
                    let file = FileOrDirectory{
                        parent: current_directory.to_string(),
                        depth: current_depth,
                        size: i32::from_str(file_line.captures(&lines[index]).unwrap().get(1).unwrap().as_str()).unwrap(),
                        name: file_line.captures(&lines[index]).unwrap().get(2).unwrap().as_str().to_string(),
                        children: Vec::new(),
                    };
                    files.push(file);
                }
            }
        }else if cd_command.is_match(&lines[index].as_str()) {
            match cd_command.captures(&lines[index]).unwrap().get(1).unwrap().as_str() {
                "/" => {
                    current_directory = "/";
                    current_depth = 0;
                },
                ".." => {
                    let new_directory = go_up_one_level(&current_directory);
                    current_directory = &new_directory.as_str();
                    current_depth -= 1;
                },
                _ => {
                    let new_directory = go_down_one_level(&current_directory, &cd_command.captures(&lines[index]).unwrap().get(1).unwrap().as_str());
                    current_directory = &new_directory.as_str();
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
     let (mut directories, mut files) = parse_input(&lines);

    // calculate the sizes of all of the subdirectories, starting from the 
    // lowest levels up
    files.sort_by_key(|x| x.depth);
    for _each_file in files { //.reverse() {
        // apply file sizes to directory_sizes
    }
    // add the sizes of child directories to parent directories
    directories.sort_by_key(|x| x.depth);
    for each_directory in directories {//.reverse() {
        // stop at root level
        if each_directory.depth == 0 {
            break;
        }
        // add this directory's size to its parent directory
    }

    // sum the size of the directories that are 100kB or less
    let mut total_of_small_directories: i32 = 0;
    directories.sort_by_key(|&x| x.size);
    for each_directory in directories {//.reverse() {
        if each_directory.size <= 100000 {
            total_of_small_directories += each_directory.size;
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