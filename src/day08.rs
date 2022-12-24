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

fn check_to_north(tree_grid: &Vec<Vec<(i32, i32)>>, x_index: usize, y_index: usize) -> i32 {
    // check trees to the North
    let tree_height: i32 = tree_grid[y_index][x_index].0;
    let mut sub_score: i32 = 0;
    for inner_y_index in 1..y_index+1 {
        println!("Comparing ({}, {}) with ({}, {})", x_index, y_index, x_index, y_index - inner_y_index);
        if tree_grid[y_index - inner_y_index][x_index].0 < tree_height {
            sub_score += 1;
        }else {
            break;
        }
    }
    sub_score
}

fn check_to_south(tree_grid: &Vec<Vec<(i32, i32)>>, x_index: usize, y_index: usize) -> i32 {
    // check trees to the South
    let tree_height: i32 = tree_grid[y_index][x_index].0;
    let mut sub_score: i32 = 0;
    for inner_y_index in y_index+1..tree_grid.len() {
        if tree_grid[inner_y_index][x_index].0 < tree_height {
            sub_score += 1;
        }else {
            break;
        }
    }
    sub_score
}

fn check_to_east(tree_grid: &Vec<Vec<(i32, i32)>>, x_index: usize, y_index: usize) -> i32 {
    // check trees to the East
    let tree_height: i32 = tree_grid[y_index][x_index].0;
    let mut sub_score: i32 = 0;
    for inner_x_index in 1..x_index+1 {
        if tree_grid[y_index][x_index - inner_x_index].0 < tree_height {
            sub_score += 1;
        }else {
            break;
        }
    }
    sub_score
}

fn check_to_west(tree_grid: &Vec<Vec<(i32, i32)>>, x_index: usize, y_index: usize) -> i32 {
    // check trees to the West
    let tree_height: i32 = tree_grid[y_index][x_index].0;
    let mut sub_score: i32 = 0;
    for inner_x_index in 1..tree_grid.len()-1 {
        println!("Comparing ({}, {}) with ({}, {})", x_index, y_index, x_index - inner_x_index, y_index);
        if tree_grid[y_index][x_index - inner_x_index].0 < tree_height {
            sub_score += 1;
        }else {
            break;
        }
    }
    sub_score
}


fn process_lines2(lines: &Vec<String>) -> i32 {
    /*
    Takes a grid of tree heights and determines the scenic score of each tree,
    returning the highest score found. To calculate the scenic score of a tree,
    first find the distance to the closest tree of the same height or taller in
    each direction, then multiply those four distances. This means that any
    tree on an edge will have a score of 0, because at least one of their
    distances will be 0. 

    See Part 2 of https://adventofcode.com/2022/day/8
    */
    let mut heighest_score: i32 = 0;
    let mut tree_grid: Vec<Vec<(i32, i32)>> = Vec::new();
    // populate the tree grid
    for each_line in lines {
        let mut next_row: Vec<(i32, i32)> = Vec::new();
        for each_tree in each_line.chars() {
            next_row.push((i32::from_str(each_tree.to_string().as_str()).unwrap(), 0));
        }
        tree_grid.push(next_row);
    }

    // visit each tree in the grid
    for x_index in 1..tree_grid.len() {
        for y_index in 1..tree_grid.len() {
            let mut tree_scores: (i32, i32, i32, i32) = (0, 0, 0, 0);

            tree_scores.1 += check_to_north(&tree_grid, x_index, y_index);
            tree_scores.3 += check_to_south(&tree_grid, x_index, y_index);
            tree_scores.0 += check_to_east(&tree_grid, x_index, y_index);
            tree_scores.2 += check_to_west(&tree_grid, x_index, y_index);
            tree_grid[y_index][x_index].1 = tree_scores.0 * tree_scores.1 * tree_scores.2 * tree_scores.3;

            if tree_grid[y_index][x_index].1 > heighest_score {
                heighest_score = tree_grid[y_index][x_index].1;
            }
        }
    }

    heighest_score
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

    #[test]
    fn test_process_lines2_short() {
        let lines = read_lines("day08_input_short.txt");
        assert_eq!(process_lines2(&lines), 8);
    }

    /*#[test]
    fn test_process_lines2_full() {
        let lines = read_lines("day08_input.txt");
        assert_eq!(process_lines2(&lines), 8);
    }*/

    /*#[test]
    fn test_process_lines2_01() {
        let lines: Vec<String> = vec!["00000".to_string(),
                                      "00000".to_string(),
                                      "00000".to_string(),
                                      "00000".to_string(),
                                      "00000".to_string()];
        assert_eq!(process_lines2(&lines), 8);
    }*/

    #[test]
    fn test_check_to_north_01() {
        let mut tree_grid: Vec<Vec<(i32, i32)>> = Vec::new();
        tree_grid.push(vec![(3, 0), (0, 0), (3, 0), (7, 0), (3, 0)]);
        tree_grid.push(vec![(2, 0), (5, 0), (5, 0), (1, 0), (2, 0)]);
        tree_grid.push(vec![(6, 0), (5, 0), (3, 0), (3, 0), (2, 0)]);
        tree_grid.push(vec![(3, 0), (3, 0), (5, 0), (4, 0), (9, 0)]);
        tree_grid.push(vec![(3, 0), (5, 0), (3, 0), (9, 0), (0, 0)]);
        assert_eq!(check_to_north(&tree_grid, 2, 1), 1);
    }

    #[test]
    fn test_check_to_north_02() {
        let mut tree_grid: Vec<Vec<(i32, i32)>> = Vec::new();
        tree_grid.push(vec![(3, 0), (0, 0), (3, 0), (7, 0), (3, 0)]);
        tree_grid.push(vec![(2, 0), (5, 0), (5, 0), (1, 0), (2, 0)]);
        tree_grid.push(vec![(6, 0), (5, 0), (3, 0), (3, 0), (2, 0)]);
        tree_grid.push(vec![(3, 0), (3, 0), (5, 0), (4, 0), (9, 0)]);
        tree_grid.push(vec![(3, 0), (5, 0), (3, 0), (9, 0), (0, 0)]);
        assert_eq!(check_to_north(&tree_grid, 4, 3), 3);
    }

    #[test]
    fn test_check_to_south_01() {
        let mut tree_grid: Vec<Vec<(i32, i32)>> = Vec::new();
        tree_grid.push(vec![(3, 0), (0, 0), (3, 0), (7, 0), (3, 0)]);
        tree_grid.push(vec![(2, 0), (5, 0), (5, 0), (1, 0), (2, 0)]);
        tree_grid.push(vec![(6, 0), (5, 0), (3, 0), (3, 0), (2, 0)]);
        tree_grid.push(vec![(3, 0), (3, 0), (5, 0), (4, 0), (9, 0)]);
        tree_grid.push(vec![(3, 0), (5, 0), (3, 0), (9, 0), (0, 0)]);
        assert_eq!(check_to_south(&tree_grid, 3, 0), 3);
    }

    #[test]
    fn test_check_to_south_02() {
        let mut tree_grid: Vec<Vec<(i32, i32)>> = Vec::new();
        tree_grid.push(vec![(3, 0), (0, 0), (3, 0), (7, 0), (3, 0)]);
        tree_grid.push(vec![(2, 0), (5, 0), (5, 0), (1, 0), (2, 0)]);
        tree_grid.push(vec![(6, 0), (5, 0), (3, 0), (3, 0), (2, 0)]);
        tree_grid.push(vec![(3, 0), (3, 0), (5, 0), (4, 0), (9, 0)]);
        tree_grid.push(vec![(3, 0), (5, 0), (3, 0), (9, 0), (0, 0)]);
        assert_eq!(check_to_south(&tree_grid, 4, 3), 1);
    }

    #[test]
    fn test_check_to_west_01() {
        let mut tree_grid: Vec<Vec<(i32, i32)>> = Vec::new();
        tree_grid.push(vec![(3, 0), (0, 0), (3, 0), (7, 0), (3, 0)]);
        tree_grid.push(vec![(2, 0), (5, 0), (5, 0), (1, 0), (2, 0)]);
        tree_grid.push(vec![(6, 0), (5, 0), (3, 0), (3, 0), (2, 0)]);
        tree_grid.push(vec![(3, 0), (3, 0), (5, 0), (4, 0), (9, 0)]);
        tree_grid.push(vec![(3, 0), (5, 0), (3, 0), (9, 0), (0, 0)]);
        assert_eq!(check_to_west(&tree_grid, 4, 1), 1);
    }

    #[test]
    fn test_check_to_west_02() {
        let mut tree_grid: Vec<Vec<(i32, i32)>> = Vec::new();
        tree_grid.push(vec![(3, 0), (0, 0), (3, 0), (7, 0), (3, 0)]);
        tree_grid.push(vec![(2, 0), (5, 0), (5, 0), (1, 0), (2, 0)]);
        tree_grid.push(vec![(6, 0), (5, 0), (3, 0), (3, 0), (2, 0)]);
        tree_grid.push(vec![(3, 0), (3, 0), (5, 0), (4, 0), (9, 0)]);
        tree_grid.push(vec![(3, 0), (5, 0), (3, 0), (9, 0), (0, 0)]);
        assert_eq!(check_to_west(&tree_grid, 3, 0), 3);
    }
}

pub fn main() {
    let result = read_lines("day08_input_short.txt");
    println!("Day 8:");
    println!("Part 1 - The number of visible trees: {}", process_lines(&result));
    println!("Part 1 - The highest scenic score possible for any tree is: {}", process_lines2(&result));
}