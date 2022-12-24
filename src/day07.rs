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
    let result: String;
    let one_deep: Regex = Regex::new(r"^/\w+/$").unwrap();
    if one_deep.is_match(&current_directory) {
        result = "/".to_string();
    }else {
        let get_parent = Regex::new(r"^(/\w+/(\w/)*)\w/$").unwrap();
        result = get_parent.captures(current_directory).unwrap().get(1).unwrap().as_str().to_string()
    }
    result
}

fn go_down_one_level(current_directory: &str, sub_directory: &str) -> String {
    let result: String;
    if current_directory == "/" {
        result = "/".to_string() + &sub_directory.to_string() + "/";
    }else {
        result = current_directory.to_string() + &sub_directory.to_string() + "/";
    }
    result.to_string()
}

fn construct_full_path(current_directory: &str, filename: &str) -> String {
    let result: String = current_directory.to_string() + &filename.to_string();
    result.to_string()
}

fn construct_full_directory_path(current_directory: &str, filename: &str) -> String {
    let result: String = current_directory.to_string() + &filename.to_string() + "/";
    result.to_string()
}

fn parse_input(lines: &Vec<String>) -> (Vec<FileOrDirectory>, Vec<FileOrDirectory>) {
    let cd_command = Regex::new(r"^\$\s+cd\s+(/|\.\.|\w+)").unwrap();
    let ls_command = Regex::new(r"^\$\s+ls").unwrap();
    let directory_line = Regex::new(r"^dir\s+(\w+)").unwrap();
    let file_line = Regex::new(r"^(\d+)\s+(.+)").unwrap();
    let mut directories: Vec<FileOrDirectory> = Vec::new();
    let mut files: Vec<FileOrDirectory> = Vec::new();
    let mut current_directory = "/".to_string();
    let mut current_depth = 0;
    let mut index: usize = 0;
    while index < lines.len()-1 {
        if ls_command.is_match(&lines[index].as_str()) {
            //println!("Recognized ls line {} on line {}", &lines[index], index);
            index += 1;
            // the next unknown many lines are files or directories inside of current_directory
            loop {
                if index > lines.len()-1 {
                    break;
                }
                if directory_line.is_match(&lines[index].as_str()) {
                    //println!("Recognized subdirectory {}", &lines[index]);
                    let directory = FileOrDirectory{
                        parent: current_directory.clone(),
                        depth: current_depth,
                        size: 0,
                        name: construct_full_directory_path(&current_directory, &directory_line.captures(&lines[index]).unwrap().get(1).unwrap().as_str()),
                        children: Vec::new(),
                    };
                    directories.push(directory);
                }else if file_line.is_match(&lines[index].as_str()) {
                    //println!("Recognized file listing {}", &lines[index]);
                    let file = FileOrDirectory{
                        parent: current_directory.clone(),
                        depth: current_depth,
                        size: i32::from_str(file_line.captures(&lines[index]).unwrap().get(1).unwrap().as_str()).unwrap(),
                        name: construct_full_path(&current_directory, &file_line.captures(&lines[index]).unwrap().get(2).unwrap().as_str()),
                        children: Vec::new(),
                    };
                    files.push(file);
                }else {
                    //println!("Done reading directory contents with new line {} on line {}", &lines[index], index);
                    break;
                }
                index += 1;
            }
        }else if cd_command.is_match(&lines[index].as_str()) {
            //println!("Recognized cd line {}", &lines[index]);
            match cd_command.captures(&lines[index]).unwrap().get(1).unwrap().as_str() {
                "/" => {
                    current_directory = "/".to_string().clone();
                    current_depth = 0;
                },
                ".." => {
                    let new_directory = go_up_one_level(&current_directory);
                    current_directory = new_directory.clone();
                    current_depth -= 1;
                },
                _ => {
                    //println!("Moving down from {}", &current_directory);
                    let new_directory = go_down_one_level(&current_directory, &cd_command.captures(&lines[index]).unwrap().get(1).unwrap().as_str());
                    current_directory = new_directory.clone();
                    //println!("to {}", &current_directory);
                    current_depth += 1;       
                }
            }
            index += 1;
        }else {
            println!("Unrecognized input {} on line {}", &lines[index], index);
        }
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
    println!("Done parsing input.");

    for file in &files {
        println!("File {} with parent {}", file.name, file.parent);
    }

    for directory in &directories {
        println!("Directory {} with parent {}", directory.name, directory.parent);
    }

    // calculate the sizes of all of the subdirectories, starting from the 
    // lowest levels up
    files.sort_by_key(|x| x.depth);
    files.reverse();
    for _each_file in files {
        // apply file sizes to directory_sizes
    }
    // add the sizes of child directories to parent directories
    directories.sort_by_key(|x| x.depth);
    directories.reverse();
    for each_directory in &directories {
        // stop at root level
        if each_directory.depth == 0 {
            break;
        }
        // add this directory's size to its parent directory
    }

    // sum the size of the directories that are 100kB or less
    let mut total_of_small_directories: i32 = 0;
    directories.sort_by_key(|x| x.size);
    directories.reverse();
    for each_directory in directories {
        if each_directory.size <= 100000 {
            total_of_small_directories += each_directory.size;
        }
    }
    total_of_small_directories
}

#[cfg(test)]
mod tests {
    use super::*;

    /*#[test]
    fn test_process_lines_short() {
        let lines = read_lines("day07_input_short.txt");
        assert_eq!(process_lines(&lines), 7);
    }*/

    #[test]
    fn test_go_up_one_level_01() {
        let current_directory: String = "/a/b/".to_string();
        assert_eq!(go_up_one_level(&current_directory.as_str()), "/a/".to_string());
    }

    #[test]
    fn test_go_up_one_level_02() {
        let current_directory: String = "/a/".to_string();
        assert_eq!(go_up_one_level(&current_directory.as_str()), "/".to_string());
    }

    #[test]
    fn test_go_up_one_level_03() {
        let current_directory: String = "/a/b/c/".to_string();
        assert_eq!(go_up_one_level(&current_directory.as_str()), "/a/b/".to_string());
    }

    #[test]
    fn test_down_one_level_01() {
        let current_directory: String = "/".to_string();
        let sub_directory: String = "a".to_string();
        assert_eq!(go_down_one_level(&current_directory.as_str(), &sub_directory.as_str()), "/a/".to_string());
    }

    #[test]
    fn test_down_one_level_02() {
        let current_directory: String = "/a/".to_string();
        let sub_directory: String = "b".to_string();
        assert_eq!(go_down_one_level(&current_directory.as_str(), &sub_directory.as_str()), "/a/b/".to_string());
    }

    #[test]
    fn test_down_one_level_03() {
        let current_directory: String = "/a/b/".to_string();
        let sub_directory: String = "c".to_string();
        assert_eq!(go_down_one_level(&current_directory.as_str(), &sub_directory.as_str()), "/a/b/c/".to_string());
    }

}

pub fn main() {
    let result = read_lines("day07_input_short.txt");
    println!("Day 7:");
    println!("Part 1 - The sum of the total sizes of those directories is: {}", process_lines(&result));
}