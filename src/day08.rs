use std::fs;
use std::str::FromStr;

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
    Takes a grid of tree heights and determines how many are visible from
    outside the grid.

    See Part 1 of https://adventofcode.com/2022/day/8
    */
    let mut visible_trees: i32 = 0;
    let mut tree_grid: Vec<Vec<(i32, bool)>> = Vec::new();
    // populate the tree grid
    for each_line in lines {
        let mut next_row: Vec<(i32, bool)> = Vec::new();
        for each_tree in each_line.chars() {
            next_row.push((i32::from_str(each_tree.to_string().as_str()).unwrap(), false));
        }
        tree_grid.push(next_row);
    }

    // mark trees as visible looking from North
    for x_index in 0..tree_grid.len() {
        let mut previous_height: i32 = -1;
        for y_index in 0..tree_grid.len() {
            if tree_grid[y_index][x_index].0 > previous_height {
                //println!("({}, {}) is visible because height {} > height {}", x_index, y_index,
                //    tree_grid[y_index][x_index].0, previous_height);
                previous_height = tree_grid[y_index][x_index].0;
                tree_grid[y_index][x_index].1 = true;
            }
        }
    }

    // mark trees as visible looking from South
    for x_index in 0..tree_grid.len() {
        let mut previous_height: i32 = -1;
        for y_index in 0..tree_grid.len() {
            let y_offset = tree_grid.len() - 1 - y_index;
            if tree_grid[y_offset][x_index].0 > previous_height {
                previous_height = tree_grid[y_offset][x_index].0;
                tree_grid[y_offset][x_index].1 = true;
            }
        }
    }

    // mark trees as visible looking from East
    for y_index in 0..tree_grid.len() {
        let mut previous_height: i32 = -1;
        for x_index in 0..tree_grid.len() {
            if tree_grid[y_index][x_index].0 > previous_height {
                previous_height = tree_grid[y_index][x_index].0;
                tree_grid[y_index][x_index].1 = true;
            }
        }
    }

    // mark trees as visible looking from West
    for y_index in 0..tree_grid.len() {
        let mut previous_height: i32 = -1;
        for x_index in 0..tree_grid.len() {
            let x_offset = tree_grid.len() - 1 - x_index;
            if tree_grid[y_index][x_offset].0 > previous_height {
                previous_height = tree_grid[y_index][x_offset].0;
                tree_grid[y_index][x_offset].1 = true;
            }
        }
    }

    // count the visible trees
    for y_index in 0..tree_grid.len() {
        for x_index in 0..tree_grid.len() {
            if tree_grid[y_index][x_index].1 {
                visible_trees += 1;
                //print!("T");
            }else {
                //print!("F");
            }
        }
        //println!("");
    }

    visible_trees
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_lines_short() {
        let lines = read_lines("day08_input_short.txt");
        assert_eq!(process_lines(&lines), 21);
    }

    #[test]
    fn test_process_lines_full() {
        let lines = read_lines("day08_input.txt");
        assert_eq!(process_lines(&lines), 1533);
    }
}

pub fn main() {
    let result = read_lines("day08_input_short.txt");
    println!("Day 8:");
    println!("Part 1 - The number of visible trees: {}", process_lines(&result));
}