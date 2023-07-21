use std::fs;
use std::collections::{VecDeque, HashMap, HashSet};

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

fn parse_input(mut lines: Vec<String>) -> (Vec<Vec<char>>, (usize, usize), (usize, usize)) {
    /*
    Takes the raw input, recording the locations of 'S' and 'E', the starting
    and ending locations, respectively. Returns these locations as well as a 2D
    Vec heightmap. 
    */
    let mut height_map: Vec<Vec<char>> = Vec::new();
    let mut starting_location: (usize, usize) = (usize::MAX, usize::MAX);
    let mut goal_location: (usize, usize) = (usize::MAX, usize::MAX);

    for line_index in 0..lines.len() {
        height_map.push(Vec::new());
        if lines[line_index].contains('S') {
            starting_location = (lines[line_index].find('S').unwrap() as usize, (height_map.len() - 1) as usize);
        }
        if lines[line_index].contains('E') {
            goal_location = (lines[line_index].find('E').unwrap() as usize, (height_map.len() - 1) as usize);
        }
    }

    (height_map, starting_location, goal_location)
}

fn get_neighbors(current_node: (usize, usize), height_map: Vec<Vec<char>>) -> Vec<(usize, usize)> {
    /*
    Get the list of locations you can step up to or down to from the current
    node.
    */
    let mut neighbors: Vec<(usize, usize)> = Vec::new();
    let height: char = height_map[current_node.1 as usize][current_node.0 as usize];
    let mut target_height: char; 
    if current_node.0 - 1 > 0 {
        target_height = height_map[current_node.1 as usize][current_node.0 as usize - 1];
        if target_height as usize <= height as usize {
            neighbors.push((current_node.0 - 1, current_node.1));
        }
    }
    if current_node.0 + 1 < height_map[current_node.1 as usize].len() {
        target_height = height_map[current_node.1 as usize][current_node.0 as usize + 1];
        if target_height as usize <= (height as usize) + 1 {
            neighbors.push((current_node.0 + 1, current_node.1));
        }
    }
    if current_node.1 - 1 > 0 {
        target_height = height_map[current_node.1 as usize - 1][current_node.0 as usize];
        if target_height as usize <= (height as usize) + 1 {
            neighbors.push((current_node.0, current_node.1 - 1));
        }
    }
    if current_node.1 + 1 < height_map.len() {
        target_height = height_map[current_node.1 as usize + 1][current_node.0 as usize];
        if target_height as usize <= (height as usize) + 1 {
            neighbors.push((current_node.0, current_node.1 + 1));
        }
    }

    neighbors
}


fn breadth_first_search(height_map: Vec<Vec<char>>, starting_location: (usize, usize), goal_location: (usize, usize)) -> i32 {
    /*
    Exploiting the fact that this map is an unweighted graph, a simple breadth-first search is the best way to find the optimal path.
    */
    let mut shortest_path_length: i32 = 0;
    let mut visited_nodes: HashSet<(usize, usize)> = HashSet::new();
    let mut distance_table: HashMap<(usize, usize), i32> = HashMap::new();

    let starting_node: (usize, usize) = (starting_location.0, starting_location.1);

    // initialize our distance table
    for y in 0..height_map.len() {
        for x in 0..height_map[y].len() {
            // set distance to 0 for starting_location, everything else set to i32::max 
            if x == starting_location.0 && y == starting_location.1 {
                distance_table.insert((x, y), 0);                
            }else {
                distance_table.insert((x, y), i32::MAX);
            }
        }
    }





    shortest_path_length
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
    let mut height_map: Vec<Vec<char>> = Vec::new();
    let mut shortest_path_length: i32 = i32::max_value();
    let starting_location: (usize, usize);
    let goal_location: (usize, usize);

    (height_map, starting_location, goal_location) = parse_input(lines.clone());

    shortest_path_length = breadth_first_search(height_map, starting_location, goal_location);
    
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
        let height_map: Vec<Vec<char>>;
        let starting_location: (usize, usize);
        let goal_location: (usize, usize);
        (height_map, starting_location, goal_location) = parse_input(lines);
        assert_eq!(starting_location, (0, 0));
        assert_eq!(goal_location, (5, 2));
        assert_eq!(height_map, vec![vec!['S', 'a', 'b', 'q', 'p', 'o', 'n', 'm'],
                                    vec!['a', 'b', 'c', 'r', 'y', 'x', 'x', 'l'],
                                    vec!['a', 'c', 'c', 's', 'z', 'E', 'x', 'k'],
                                    vec!['a', 'c', 'c', 't', 'u', 'v', 'w', 'j'],
                                    vec!['a', 'b', 'd', 'e', 'f', 'g', 'h', 'i']]);
    }
}

pub fn main() {
    let result = read_lines("day12_input_short.txt");
    println!("Day 12:");
    println!("Part 1 - The fewest number of steps required is: {}", process_lines(&result));
}