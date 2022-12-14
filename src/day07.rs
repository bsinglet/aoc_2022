use std::fs;
use std::str::FromStr;
use regex::Regex;

#[derive(Debug)]
pub struct FileOrDirectory {
    name: String,
    parent: String,
    depth: i32,
    size: i32,
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
        let get_parent = Regex::new(r"^(/\w+/(\w+/)*)\w+/$").unwrap();
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
    // don't forget to initialize "/"!
    directories.push(FileOrDirectory{
        parent: "/".to_string(),
        depth: -1,
        size: 0,
        name: "/".to_string(),
    });
    // process the command history
    while index < lines.len()-1 {
        if ls_command.is_match(&lines[index].as_str()) {
            index += 1;
            // the next unknown many lines are files or directories inside of current_directory
            loop {
                if index > lines.len()-1 {
                    break;
                }
                if directory_line.is_match(&lines[index].as_str()) {
                    let directory = FileOrDirectory{
                        parent: current_directory.clone(),
                        depth: current_depth,
                        size: 0,
                        name: construct_full_directory_path(&current_directory, &directory_line.captures(&lines[index]).unwrap().get(1).unwrap().as_str()),
                    };
                    directories.push(directory);
                }else if file_line.is_match(&lines[index].as_str()) {
                    let file = FileOrDirectory{
                        parent: current_directory.clone(),
                        depth: current_depth,
                        size: i32::from_str(file_line.captures(&lines[index]).unwrap().get(1).unwrap().as_str()).unwrap(),
                        name: construct_full_path(&current_directory, &file_line.captures(&lines[index]).unwrap().get(2).unwrap().as_str()),
                    };
                    files.push(file);
                }else {
                    break;
                }
                index += 1;
            }
        }else if cd_command.is_match(&lines[index].as_str()) {
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
                    let new_directory = go_down_one_level(&current_directory, &cd_command.captures(&lines[index]).unwrap().get(1).unwrap().as_str());
                    current_directory = new_directory.clone();
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
    println!("Parsing input.");
    let (mut directories, mut files) = parse_input(&lines);
    println!("Done parsing input.");

    // calculate the sizes of all of the subdirectories, starting from the 
    // lowest levels up
    files.sort_by_key(|x| x.depth);
    files.reverse();
    for each_file in files {
        // apply file sizes to directory_sizes
        let mut directory_index = 0;
        while directory_index < directories.len() {
            if directories[directory_index].name == each_file.parent {
                break;
            }
            directory_index += 1;
        }
        if directory_index >= directories.len() {
            eprintln!("Couldn't find directory matching the name {}", each_file.parent);
        }
        directories[directory_index].size += each_file.size;
    }

    // add the sizes of child directories to parent directories
    directories.sort_by_key(|x| x.depth);
    directories.reverse();
    let mut each_directory_index = 0;
    while each_directory_index < directories.len() {
        // stop at root level
        if directories[each_directory_index].depth == -1 {
            break;
        }
        // add this directory's size to its parent directory
        let mut directory_index = 0;
        while directory_index < directories.len() {
            if directories[directory_index].name == directories[each_directory_index].parent {
                break;
            }
            directory_index += 1;
        }
        if directory_index >= directories.len() {
            eprintln!("Couldn't find directory matching the name {}", directories[each_directory_index].parent);
        }
        directories[directory_index].size += directories[each_directory_index].size;
        each_directory_index += 1;
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

fn process_lines2(lines: &Vec<String>) -> i32 {
    /*
     Takes the command-line history of a device, determines the recursive 
     sizes of all directories, then determines the smallest directory to delete
     such that / will be no larger than 30_000_000 bytes.

     See Part 2 of https://adventofcode.com/2022/day/7
     */
    // parse the input into directories and files
    println!("Parsing input.");
    let (mut directories, mut files) = parse_input(&lines);
    println!("Done parsing input.");

    // calculate the sizes of all of the subdirectories, starting from the 
    // lowest levels up
    files.sort_by_key(|x| x.depth);
    files.reverse();
    for each_file in files {
        // apply file sizes to directory_sizes
        let mut directory_index = 0;
        while directory_index < directories.len() {
            if directories[directory_index].name == each_file.parent {
                break;
            }
            directory_index += 1;
        }
        if directory_index >= directories.len() {
            eprintln!("Couldn't find directory matching the name {}", each_file.parent);
        }
        directories[directory_index].size += each_file.size;
    }

    // add the sizes of child directories to parent directories
    directories.sort_by_key(|x| x.depth);
    directories.reverse();
    let mut each_directory_index = 0;
    while each_directory_index < directories.len() {
        // stop at root level
        if directories[each_directory_index].depth == -1 {
            break;
        }
        // add this directory's size to its parent directory
        let mut directory_index = 0;
        while directory_index < directories.len() {
            if directories[directory_index].name == directories[each_directory_index].parent {
                break;
            }
            directory_index += 1;
        }
        if directory_index >= directories.len() {
            eprintln!("Couldn't find directory matching the name {}", directories[each_directory_index].parent);
        }
        directories[directory_index].size += directories[each_directory_index].size;
        each_directory_index += 1;
    }

    // sum the size of the directories that are 100kB or less
    directories.sort_by_key(|x| x.size);
    let mut smallest_suitable_size: i32 = 30000000 - (70000000 - directories[directories.len()-1].size);
    for each_directory in directories {
        if each_directory.size >= smallest_suitable_size {
            smallest_suitable_size = each_directory.size;
            break;
        }
    }
    smallest_suitable_size
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_lines_short() {
        let lines = read_lines("day07_input_short.txt");
        assert_eq!(process_lines(&lines), 95437);
    }

    #[test]
    fn test_process_lines_full() {
        let lines = read_lines("day07_input.txt");
        assert_eq!(process_lines(&lines), 1453349);
    }

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
    fn test_go_up_one_level_04() {
        let current_directory: String = "/bfqzjjct/cgcqpjpn/phslrcw/jnzjq/".to_string();
        assert_eq!(go_up_one_level(&current_directory.as_str()), "/bfqzjjct/cgcqpjpn/phslrcw/".to_string());
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

    #[test]
    fn test_process_lines2_short() {
        let lines = read_lines("day07_input_short.txt");
        assert_eq!(process_lines2(&lines), 24933642);
    }

    #[test]
    fn test_process_lines2_full() {
        let lines = read_lines("day07_input.txt");
        assert_eq!(process_lines2(&lines), 2948823);
    }

}

pub fn main() {
    let result = read_lines("day07_input.txt");
    println!("Day 7:");
    println!("Part 1 - The sum of the total sizes of those directories is: {}", process_lines(&result));
    println!("Part 2 - The smallest directory we can delete to free up enough space is {}", 
        process_lines2(&result));
}