use std::fs;
use std::collections::VecDeque;

#[derive(Debug)]
#[derive(Clone)]
pub struct Node {
    x: i32,
    y: i32,
    distance: i32,
    parent: (i32, i32),
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }

    fn ne(&self, other: &Self) -> bool {
        self.x != other.x || self.y != other.y
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

fn heuristic_function(starting_location: (i32, i32), starting_height: i32, 
    goal_location: (i32, i32), goal_height: i32,) -> i32 {
    /*
    Gives an estimated distance between two points on a grid.
    */
    let mut distance: i32 = (starting_location.0 - goal_location.0).abs() 
        + (starting_location.1 - goal_location.1).abs();
    // difference in height is an important factor, but this is a pretty naive 
    // way to handle it...
    distance += goal_height - starting_height;
    distance
}

fn get_neighbors(current_node: Node, height_map: Vec<Vec<i32>>) -> Vec<(i32, i32)> {
    /*
    Get the list of locations you can step up to or down to from the current
    node.
    */
    let mut neighbors: Vec<(i32, i32)> = Vec::new();
    let height: i32 = height_map[current_node.y as usize][current_node.x as usize];
    let mut target_height: i32; 
    if current_node.x - 1 > 0 {
        target_height = height_map[current_node.y as usize][current_node.x as usize - 1];
        if target_height <= height + 1 {
            neighbors.push((current_node.x - 1, current_node.y));
        }
    }
    if current_node.x + 1 < height_map[current_node.y as usize].len() as i32 {
        target_height = height_map[current_node.y as usize][current_node.x as usize + 1];
        if target_height <= height + 1 {
            neighbors.push((current_node.x + 1, current_node.y));
        }
    }
    if current_node.y - 1 > 0 {
        target_height = height_map[current_node.y as usize - 1][current_node.x as usize];
        if target_height <= height + 1 {
            neighbors.push((current_node.x, current_node.y - 1));
        }
    }
    if current_node.y + 1 < height_map.len() as i32 {
        target_height = height_map[current_node.y as usize + 1][current_node.x as usize];
        if target_height <= height + 1 {
            neighbors.push((current_node.x, current_node.y + 1));
        }
    }

    neighbors
}

fn get_index(each_neighbor: (i32, i32), unvisited_nodes: VecDeque<Node>) -> i32 {
    /* 
    Find the index in a VecDequeue of Nodes matching a coordinate pair.
    Returns -1 if it's not present.
    */
    let mut located_index: i32 = -1;
    for index in 0..unvisited_nodes.len() {
        if unvisited_nodes[index].x == each_neighbor.0 && unvisited_nodes[index].y == each_neighbor.1 {
            located_index = index as i32;
            break;
        }
    }

    located_index
}

fn dijkstra_search(height_map: Vec<Vec<i32>>, starting_location: (i32, i32), goal_location: (i32, i32)) -> i32 {
    let mut shortest_path_length: i32 = i32::max_value();
    /*
    Attempting to adapt Dijkstra's algorithm to the parameters of this
    challenge.
    */
    let mut unvisited_nodes: VecDeque<(Node)> = VecDeque::new();
    let mut starting_node: Node = Node{
        x: starting_location.0,
        y: starting_location.1,
        distance: 0,
        parent: (-1, -1),
    };

    // initialize all the nodes
    for y in 0..height_map.len() {
        for x in 0..height_map[y].len() {
            // starting location goes at the front of the unvisited nodes list
            if x as i32 == starting_location.0 && y as i32 == starting_location.1 {
                unvisited_nodes.push_front(starting_node);
                continue;
            }

            unvisited_nodes.push_back(Node{
                x: x as i32,
                y: y as i32,
                distance: i32::max_value(),
                parent: (-1, -1),
            });
        }
    }

    // the list of unvisited nodes should always be sorted by distance. At this
    // point, though, that just means the starting node is at the front, and
    // all other nodes follow in an arbitrary order.
    
    // visit each node in the queue, determining distance from the starting location
    while unvisited_nodes.len() > 0 {
        let current_node = unvisited_nodes.pop_front().unwrap();

        for each_neighbor in get_neighbors(current_node.clone(), height_map.clone()) {
            let neighbor_index: i32 = get_index(each_neighbor, unvisited_nodes.clone());
            if neighbor_index == -1 {
                continue;
            }

            // evaluate this neighbor
            let alternative_distance: i32 = current_node.distance + 1;
            if alternative_distance < unvisited_nodes[neighbor_index as usize].distance {
                unvisited_nodes[neighbor_index as usize].distance = alternative_distance;
                unvisited_nodes[neighbor_index as usize].parent = (current_node.x, current_node.y);
            }
        }

        // re-sort unvisited nodes so it's still in ascending order of distances.
        let mut temporary_list: Vec<Node> = unvisited_nodes.clone().into_iter().collect::<Vec<Node>>();
        temporary_list.sort_by_key(|x| x.distance);
        unvisited_nodes = temporary_list.into_iter().collect();
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
    let mut height_map: Vec<Vec<i32>> = Vec::new();
    let mut shortest_path_length: i32 = i32::max_value();
    let starting_location: (i32, i32);
    let goal_location: (i32, i32);

    (height_map, starting_location, goal_location) = parse_input(lines.clone());

    shortest_path_length = dijkstra_search(height_map, starting_location, goal_location);
    
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