use std::fs;

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

fn parse_input(mut lines: Vec<String>) -> (Vec<Vec<i32>>, (i32, i32), (i32, i32)) {
    /*
    Takes the raw input, recording the locations of 'S' and 'E', the starting
    and ending locations, respectively. Returns these locations as well as a 2D
    Vec heightmap. 
    */
    let mut height_map: Vec<Vec<i32>> = Vec::new();
    let mut starting_location: (i32, i32) = (-1, -1);
    let mut goal_location: (i32, i32) = (-1, -1);

    for line_index in 0..lines.len() {
        height_map.push(Vec::new());
        if lines[line_index].contains('S') {
            starting_location = (lines[line_index].find('S').unwrap() as i32, (height_map.len() - 1) as i32);
            lines[line_index] = lines[line_index].replace('S', "a");
        }
        if lines[line_index].contains('E') {
            goal_location = (lines[line_index].find('E').unwrap() as i32, (height_map.len() - 1) as i32);
            lines[line_index] = lines[line_index].replace('E', "z");
        }
        height_map[line_index] = lines[line_index].chars().map(|x| ((x as u32) - ('a' as u32)) as i32).collect();
    }

    (height_map, starting_location, goal_location)
}

fn process_lines(lines: &Vec<String>) -> i32 {
    /*
    Determines the fewest number of steps required to move from the current
    position to the location with the best signal. Heights range from 'a' to
    'z' (lowest to highest.) 'S' is the starting position, with a height of
    'a', 'E' is the end position, with a height of 'z'. You can only move along
    the grid, not diagonally, and you can drop any number of levels down but
    can only move one level of height up per square. 

    See Part 1 of https://adventofcode.com/2022/day/12
    */
    let mut height_map: Vec<Vec<i32>> = Vec::new();
    let mut shortest_path_length: i32 = 0;
    let starting_location: (i32, i32);
    let goal_location: (i32, i32);

    (height_map, starting_location, goal_location) = parse_input(lines.clone());
    
    shortest_path_length
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_lines_short() {
        let lines = read_lines("day12_input_short.txt");
        assert_eq!(process_lines(&lines), 31);
    }

    #[test]
    fn test_process_lines_full() {
        let lines = read_lines("day12_input.txt");
        assert_eq!(process_lines(&lines), -1);
    }

    #[test]
    fn test_parse_input_01() {
        // very simple unit test to make sure the parse_input() function
        // correctly finds the starting and ending locations, as well as the
        // heights of each square.
        let lines = read_lines("day12_input_short.txt");
        let height_map: Vec<Vec<i32>>;
        let starting_location: (i32, i32);
        let goal_location: (i32, i32);
        (height_map, starting_location, goal_location) = parse_input(lines);
        assert_eq!(starting_location, (0, 0));
        assert_eq!(goal_location, (5, 2));
        assert_eq!(height_map, vec![vec![0, 0, 1, 16, 15, 14, 13, 12], 
                                    vec![0, 1, 2, 17, 24, 23, 23, 11], 
                                    vec![0, 2, 2, 18, 25, 25, 23, 10], 
                                    vec![0, 2, 2, 19, 20, 21, 22, 9], 
                                    vec![0, 1, 3, 4, 5, 6, 7, 8]]);
    }
}

pub fn main() {
    let result = read_lines("day12_input_short.txt");
    println!("Day 12:");
    println!("Part 1 - The fewest number of steps required is: {}", process_lines(&result));
}