use std::fs;
use std::str::FromStr;
use std::mem::swap;
use std::cmp::Ordering;
use regex::Regex;
use std::collections::HashMap;

#[derive(Clone, Copy, Debug, PartialEq)]
enum CaveEnum {
    Rock,
    Sand,
}

fn _cave_hashmap_to_string(hashmap: &HashMap<(usize, usize), CaveEnum>) -> String {
    let mut result: String = "".to_string();
    let mut point_map: Vec<String> = vec![];
    for y in 0..12 {
        let mut line: String = "".to_string();
        for x in 486..512 {
            let c: char = match hashmap.get(&(x, y)) {
                Some(x) => {
                    match x {
                        CaveEnum::Rock => '#',
                        CaveEnum::Sand => 'o',
                    }
                },
                None => '.',
            };
            line.push(c);
        }
        line.push('\n');
        point_map.push(line);
    }
    for each in point_map {
        result = result + &each;
    }
    result
}

fn read_lines(filename: &str) -> Vec<String> {
    /*
    Open a text file and return a Vector of Strings representing the individual
    lines.
    */
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file.");
    let mut lines: Vec<String> = Vec::new();
    for each in contents.split_terminator('\n').filter(|x| !x.is_empty()).collect::<Vec<&str>>() {
        lines.push(each.trim().to_string());
    }
    lines
}

fn parse_point(point: String) -> (i32, i32) {
    let point_regex = Regex::new(r"^(\d+),(\d+)$").unwrap();
    let point_captures = point_regex.captures(point.as_str()).unwrap();
    (i32::from_str(point_captures.get(1).unwrap().as_str()).unwrap(), 
    i32::from_str(point_captures.get(2).unwrap().as_str()).unwrap())
}

enum Direction {
    Down,
    Right,
}

fn draw_lines(line: String, hashmap: HashMap<(usize, usize), CaveEnum>) -> HashMap<(usize, usize), CaveEnum> {
    /*
    Takes an input of 2 or more points (represented as x-y coordinates 
    separated by commas), with points separated by "->" arrows.

    For example, 2,3 -> 2, 5 -> 5,5 represents two line segments

    Populates all of the points in those line segments on the hashmap provided
    as CaveEnum::Rock.
    */
    let points: Vec<String> = line.split(" -> ").map(|s| s.to_string()).collect();
    let mut result_hashmap = hashmap.clone();
    for index in 0..points.len()-1 {
        let mut start: (i32, i32) = parse_point(points[index].clone());
        let mut end: (i32, i32) = parse_point(points[index+1].clone());
        let mut direction: Direction = Direction::Down;
        match start.1.cmp(&end.1) {
            Ordering::Less => {
                
            },
            Ordering::Greater => {
                swap(&mut start, &mut end);
            },
            Ordering::Equal => {
                match start.0.cmp(&end.0) {
                    Ordering::Less => {
                        direction = Direction::Right;
                    },
                    Ordering::Greater => {
                        direction = Direction::Right;
                        swap(&mut start, &mut end);
                    },
                    Ordering::Equal => {
                        continue;
                    }
                }
            }
        }
        match direction {
            Direction::Down => {
                for y in start.1..end.1+1 {
                    //println!("Adding point ({}, {})", start.0, y);
                    result_hashmap.insert((start.0 as usize, y as usize), CaveEnum::Rock);
                }
            },
            Direction::Right => {
                for x in start.0..end.0+1 {
                    //println!("Adding point ({}, {})", x, start.1);
                    result_hashmap.insert((x as usize, start.1 as usize), CaveEnum::Rock);
                }
            },
        }
    }
    result_hashmap
}

fn next_point_below(falling_sand: (usize, usize), hashmap: &HashMap<(usize, usize), CaveEnum>) -> ((usize, usize), bool) {
    let mut found = false;
    let mut highest_point: (usize, usize) = falling_sand;

    for each_point in hashmap.keys() {
        if each_point.0 == falling_sand.0 && each_point.1 > falling_sand.1 {
            if !found || each_point.1 < highest_point.1 {
                highest_point.0 = each_point.0;
                highest_point.1 = each_point.1;
                found = true;
            }
        }
    }
    (highest_point, found)
}

fn get_maximum_height(hashmap: &HashMap<(usize, usize), CaveEnum>) -> usize {
    let mut heighest_point: (usize, usize) = *hashmap.keys().last().unwrap();

    for each_point in hashmap.keys() {
        if each_point.1 > heighest_point.1 {
            heighest_point = (each_point.0, each_point.1);
        }
    }

    heighest_point.1
}

fn process_lines(lines: &Vec<String>) -> i32 {
    /*

    See Part 1 of https://adventofcode.com/2022/day/14
    */
    let mut hashmap: HashMap<(usize, usize), CaveEnum> = HashMap::<(usize, usize), CaveEnum>::new();
    let mut units_sand_rested: i32 = 0;
    let mut falling_sand: (usize, usize);
    let mut abyss_reached: bool = false;
    
    // parse the input lines to find the walls of the cave
    for each_line in lines {
        hashmap = draw_lines(each_line.clone(), hashmap);
    }

    // simulate sand falling until one lands in the abyss
    while !abyss_reached {
        // each new grain of sand starts at (500, 0)
        let mut found_rest: bool = false;
        falling_sand = (500, 0);
        //println!("Dropping {}th grain of sand", units_sand_rested);
        //println!("{}", _cave_hashmap_to_string(&hashmap));
        //println!("Total solid particles on on map {}", hashmap.len());
    
        while !found_rest {
            // see if the sand falls into the abyss
            let (next_point, found) = next_point_below(falling_sand, &hashmap);
            //println!("Next spot to land on ({},{})", next_point.0, next_point.1);
            if !found {
                abyss_reached = true;
                break;
            }
            if next_point == (500, 0) {
                panic!("Sand backed up to the top, which shouldn't be possible.");
            }

            // the sand lands on top of the next point found
            falling_sand.0 = next_point.0; 
            falling_sand.1 = next_point.1 - 1;

            // look for an opening to the lower left or lower right
            if !hashmap.contains_key(&(falling_sand.0 - 1, falling_sand.1 + 1)) {
                falling_sand.0 -= 1;
                falling_sand.1 += 1;
                //println!("Grain of sand slid down-left to ({}, {})", falling_sand.0, falling_sand.1);
            }else if !hashmap.contains_key(&(falling_sand.0 + 1, falling_sand.1 + 1)) {
                falling_sand.0 += 1;
                falling_sand.1 += 1;
                //println!("Grain of sand slid down-right to ({}, {})", falling_sand.0, falling_sand.1);
            }else {
                // came to rest
                found_rest = true;
                units_sand_rested += 1;
            }
        }
        // register the new grain of sand
        if !abyss_reached {
            //println!("Grain of sand rested at ({}, {})", falling_sand.0, falling_sand.1);
            hashmap.insert(falling_sand, CaveEnum::Sand);
        }
    }

    units_sand_rested
}

fn process_lines2(lines: &Vec<String>) -> i32 {
    /*
    The same as process_lines(), except now we're assuming there's a floor 
    infinitely long, two units below the lowest point in the input. Because y=0
    is the top of the cave, that means the floor is max(y)+2.

    The return value is the number of units of sand that come to rest until one 
    rests (500, 0), blocking the cave.

    See Part 2 of https://adventofcode.com/2022/day/14
    */
    let mut hashmap: HashMap<(usize, usize), CaveEnum> = HashMap::<(usize, usize), CaveEnum>::new();
    let mut units_sand_rested: i32 = 0;
    let mut falling_sand: (usize, usize);
    let mut ceiling_blocked: bool = false;
    let floor_height: usize;
    
    // parse the input lines to find the walls of the cave
    for each_line in lines {
        hashmap = draw_lines(each_line.clone(), hashmap);
    }

    floor_height = get_maximum_height(&hashmap) + 2;

    // simulate sand falling until one lands in the abyss
    while !ceiling_blocked {
        // each new grain of sand starts at (500, 0)
        let mut found_rest: bool = false;
        falling_sand = (500, 0);
        println!("Dropping {}th grain of sand", units_sand_rested);
        println!("{}", _cave_hashmap_to_string(&hashmap));
        //println!("Total solid particles on on map {}", hashmap.len());
    
        while !found_rest {
            // see if the sand falls into the abyss
            let (next_point, found) = next_point_below(falling_sand, &hashmap);
            println!("Next spot to land on ({},{})", next_point.0, next_point.1);
            if !found {
                falling_sand.1 = floor_height - 1;
                found_rest = true;
                break;
            }
            if next_point == (500, 0) && hashmap.contains_key(&(500, 1)) {
                ceiling_blocked = true;
                break;
            }

            // the sand lands on top of the next point found
            falling_sand.0 = next_point.0;
            falling_sand.1 = next_point.1 - 1;

            // look for an opening to the lower left or lower right
            if !hashmap.contains_key(&(falling_sand.0 - 1, falling_sand.1 + 1)) {
                falling_sand.0 -= 1;
                falling_sand.1 += 1;
                //println!("Grain of sand slid down-left to ({}, {})", falling_sand.0, falling_sand.1);
            }else if !hashmap.contains_key(&(falling_sand.0 + 1, falling_sand.1 + 1)) {
                falling_sand.0 += 1;
                falling_sand.1 += 1;
                //println!("Grain of sand slid down-right to ({}, {})", falling_sand.0, falling_sand.1);
            }else {
                // came to rest
                found_rest = true;
                units_sand_rested += 1;
            }
            if units_sand_rested > 79 {
                ceiling_blocked = true;
                println!("Units sand rested is {}, but sand particles is {}", units_sand_rested,
                hashmap.values().filter(|&x| x == &CaveEnum::Sand).collect::<Vec<&CaveEnum>>().len());
                todo!("Crutch to kill infinite loop.");
            }
        }
        // register the new grain of sand
        if !ceiling_blocked {
            //println!("Grain of sand rested at ({}, {})", falling_sand.0, falling_sand.1);
            hashmap.insert(falling_sand, CaveEnum::Sand);
        }
    }

    units_sand_rested
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_lines_short() {
        let lines = read_lines("day14_input_short.txt");
        assert_eq!(process_lines(&lines), 24);
    }

    #[test]
    fn test_process_lines_full() {
        let lines = read_lines("day14_input.txt");
        assert_eq!(process_lines(&lines), 1078);
    }

    #[test]
    fn test_process_lines2_short() {
        let lines = read_lines("day14_input_short.txt");
        assert_eq!(process_lines(&lines), 93);
    }

    #[test]
    fn test_process_lines2_full() {
        let lines = read_lines("day14_input.txt");
        assert_eq!(process_lines2(&lines), -1);
    }

    #[test]
    fn test_process_lines_01() {
        let lines = vec!["510,10 -> 520,10".to_string()];
        assert_eq!(process_lines(&lines), 0);
    }

    #[test]
    fn test_process_lines_02() {
        let lines = vec!["499,10 -> 501,10".to_string()];
        assert_eq!(process_lines(&lines), 1);
    }

    #[test]
    fn test_process_lines_03() {
        let lines = vec!["499,9 -> 499,10".to_string(),
                                      "499,10 -> 500,10".to_string()];
        assert_eq!(process_lines(&lines), 0);
    }

    #[test]
    fn test_process_lines_04() {
        let lines = vec!["499,9 -> 499,10".to_string(),
                                      "499,10 -> 501,10".to_string()];
        assert_eq!(process_lines(&lines), 1);
    }

    #[test]
    fn test_process_lines2_01() {
        let lines = vec!["500,0 -> 500,0".to_string()];
        assert_eq!(process_lines2(&lines), 0);
    }

    #[test]
    fn test_process_lines2_02() {
        let lines = vec!["500,1 -> 500,1".to_string(),
                                      "499,0 -> 499,1".to_string(),
                                      "501,0 -> 501,1".to_string()];
        assert_eq!(process_lines2(&lines), 1);
    }

    #[test]
    fn test_process_lines2_03() {
        let lines = vec!["500,2 -> 500,2".to_string(),
                                      "499,0 -> 499,2".to_string(),
                                      "501,0 -> 501,2".to_string()];
        assert_eq!(process_lines2(&lines), 2);
    }

    #[test]
    fn test_process_lines2_04() {
        let lines = vec!["500,1 -> 500,1".to_string()];
        assert_eq!(process_lines2(&lines), 8);
    }

    #[test]
    fn test_parse_point_01() {
        assert_eq!(parse_point("500,500".to_string()), (500, 500));
        assert_eq!(parse_point("0,5".to_string()), (0, 5));
    }

    #[test]
    fn test_draw_lines_01() {
        let mut hashmap: HashMap<(usize, usize), CaveEnum> = HashMap::<(usize, usize), CaveEnum>::new();
        hashmap = draw_lines("500,0 -> 500,10".to_string(), hashmap);
        println!("Hashmap contains {} points.", hashmap.len());
        assert_eq!(hashmap.len(), 11);
        let cell_value: CaveEnum = *hashmap.get(&(500,0)).unwrap();
        assert_eq!(cell_value, *Some(&CaveEnum::Rock).unwrap());
    }

    #[test]
    fn test_draw_lines_02() {
        let mut hashmap: HashMap<(usize, usize), CaveEnum> = HashMap::<(usize, usize), CaveEnum>::new();
        hashmap = draw_lines("500,0 -> 500,1 -> 501,1".to_string(), hashmap);
        assert_eq!(hashmap.len(), 3);
        let cell_value: CaveEnum = *hashmap.get(&(501,1)).unwrap();
        assert_eq!(cell_value, *Some(&CaveEnum::Rock).unwrap());
    }

    #[test]
    fn test_draw_lines_03() {
        let mut hashmap: HashMap<(usize, usize), CaveEnum> = HashMap::<(usize, usize), CaveEnum>::new();
        hashmap = draw_lines("498,4 -> 498,6 -> 496,6".to_string(), hashmap);
        assert_eq!(hashmap.len(), 5);
        let cell_value: CaveEnum = *hashmap.get(&(497,6)).unwrap();
        assert_eq!(cell_value, *Some(&CaveEnum::Rock).unwrap());
    }

    #[test]
    fn test_next_point_below_01() {
        let mut hashmap: HashMap<(usize, usize), CaveEnum> = HashMap::<(usize, usize), CaveEnum>::new();
        hashmap = draw_lines("500,10 -> 509,10".to_string(), hashmap);
        let (destination, found) = next_point_below((500,0), &hashmap);
        assert!(found);
        assert_eq!(destination, (500,10));
    }
}

pub fn main() {
    let result = read_lines("day14_input_short.txt");
    println!("Day 14:");
    println!("Part 1 - A total of {} units of sand come to rest before sand starts flowing into the abyss below.", process_lines(&result));
    println!("Part 2 - A total of {} units of sand come to rest before the source of the sand becomes blocked", process_lines2(&result));
}