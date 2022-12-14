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
    Emulate the communicator CPU, recording the signal strength of register X
    at the 20th, 60th, 100th, 140th, 180th, and 220th cycles. This CPU only has
    two operations (noop and addx), and one register: x.

    See Part 1 of https://adventofcode.com/2022/day/10
    */
    let mut cycle: i32 = 0;
    let mut register_x: i32 = 1;
    let mut recordings: Vec<i32> = Vec::new();
    let cycle_indices: Vec<i32> = vec![19, 59, 99, 139, 179, 219];
    for each_instruction in lines {
        if each_instruction != "noop" {
            let instruction = each_instruction.split(" ").next().unwrap();
            let value = i32::from_str(each_instruction.split(" ").skip(1).next().unwrap()).unwrap();
            if instruction != "addx" {
                eprintln!("Unrecognized instruction {}", instruction);
                break;
            }

            cycle += 1;
            if cycle_indices.contains(&cycle) {
                //println!("Storing signal {} at cycle {}", register_x, cycle);
                recordings.push(register_x * (cycle + 1));
            }
            cycle += 1;
            register_x += value;
            if cycle_indices.contains(&cycle) {
                //println!("Storing signal {} at cycle {}", register_x, cycle);
                recordings.push(register_x * (cycle + 1));
            }
        }else {
            // record Register X if we're at one of the key cycles
            //println!("Checking if cycle {} is in the list.", cycle);
            cycle += 1;
            if cycle_indices.contains(&cycle) {
                //println!("Storing signal {} at cycle {}", register_x, cycle);
                recordings.push(register_x * (cycle + 1));
            }
        }
    }

    recordings.into_iter().reduce(|x, y| x + y).unwrap()
}

fn draw_screen(pixels: Vec<char>) -> Vec<String> {
    let mut output: Vec<String> = Vec::new();
    for each_line in pixels.chunks(40) {
        output.push(each_line.into_iter().collect::<String>());
    }
    output
}

fn process_lines2(lines: &Vec<String>) -> Vec<String> {
    /*

    See Part 2 of https://adventofcode.com/2022/day/10
    */
    let mut cycle: i32 = 0;
    let mut register_x: i32 = 1;
    let mut pixels: Vec<char> = Vec::new();
    // initialize the display
    for _ in 0..240 {
        pixels.push('.');
    }
    // process the instructions
    for each_instruction in lines {
        if (cycle % 40) == register_x - 1 || (cycle % 40) == register_x || (cycle % 40) == register_x + 1 {
            //println!("Storing signal {} at cycle {}", register_x, cycle);
            pixels[cycle as usize] = '#';
        }else {
            pixels[cycle as usize] = '.';
        }

        if each_instruction != "noop" {
            let instruction = each_instruction.split(" ").next().unwrap();
            let value = i32::from_str(each_instruction.split(" ").skip(1).next().unwrap()).unwrap();
            //println!("Cycle {}, register_x {}: {} {}", cycle, register_x, instruction, value);
            if instruction != "addx" {
                eprintln!("Unrecognized instruction {}", instruction);
                break;
            }
            cycle += 1;
            if (cycle % 40) == register_x - 1 || (cycle % 40) == register_x || (cycle % 40) == register_x + 1 {
                //println!("Storing signal {} at cycle {}", register_x, cycle);
                pixels[cycle as usize] = '#';
            }else {
                pixels[cycle as usize] = '.';
            }
            cycle += 1;
            register_x += value;
            if (cycle % 40) == register_x - 1 || (cycle % 40) == register_x || (cycle % 40) == register_x + 1 {
                //println!("Storing signal {} at cycle {}", register_x, cycle);
                pixels[cycle as usize] = '#';
            }else {
                pixels[cycle as usize] = '.';
            }
        }else {
            //println!("Checking if cycle {} is in the list.", cycle);
            cycle += 1;
        }
    }

    draw_screen(pixels)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_lines_short() {
        let lines = read_lines("day10_input_short.txt");
        assert_eq!(process_lines(&lines), 13140);
    }

    #[test]
    fn test_process_lines_full() {
        let lines = read_lines("day10_input.txt");
        assert_eq!(process_lines(&lines), 15360);
    }

    #[test]
    fn test_process_lines2_short() {
        let lines = read_lines("day10_input_short.txt");
        let output:Vec<String> = vec!["##..##..##..##..##..##..##..##..##..##..".to_string(),
                                      "###...###...###...###...###...###...###.".to_string(),
                                      "####....####....####....####....####....".to_string(),
                                      "#####.....#####.....#####.....#####.....".to_string(),
                                      "######......######......######......####".to_string(),
                                      "#######.......#######.......#######.....".to_string()];
        assert_eq!(process_lines2(&lines), output);
    }

    #[test]
    fn test_process_lines2_full() {
        let lines = read_lines("day10_input.txt");
        let output:Vec<String> = vec!["###..#..#.#....#..#...##..##..####..##..".to_string(),
                                      "#..#.#..#.#....#..#....#.#..#....#.#..#.".to_string(),
                                      "#..#.####.#....####....#.#......#..#..#.".to_string(),
                                      "###..#..#.#....#..#....#.#.##..#...####.".to_string(),
                                      "#....#..#.#....#..#.#..#.#..#.#....#..#.".to_string(),
                                      "#....#..#.####.#..#..##...###.####.#..#.".to_string()];
        assert_eq!(process_lines2(&lines), output);
    }
}

pub fn main() {
    let result = read_lines("day10_input.txt");
    println!("Day 10:");
    println!("Part 1 - The sum of these six signal strengths is: {}", process_lines(&result));
    println!("Part 2 - The CRT screen outputs:");
    for each_line in process_lines2(&result) {
        println!("{}", each_line);
    }
}