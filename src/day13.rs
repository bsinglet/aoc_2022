use std::fs;
use std::str::FromStr;

#[derive(Clone)]
enum PacketElement {
    List(Vec<PacketElement>),
    Number(i32),
}

fn print_packet_element(packet: PacketElement) -> String {
    /*
    Takes a PacketElement enum and recursively converts it into a String. This
    was essential for testing Part 1, as well as locating the divider packets
    in the sorted list in Part 2. 
    */
    let mut output: String;
    match packet {
        PacketElement::Number(num) => {
            //println!("Returning {}", num);
            output = num.to_string();
        },
        PacketElement::List(list) => {
            let mut elements: Vec<String> = Vec::<String>::new();
            //println!("Length of list is {}", list.len());
            for each_element in list {
                //println!("Looking at list element");
                elements.push(print_packet_element(each_element));
            }
            output = elements.join(",");
            output = "[".to_owned() + &output + "]";
        }
    }
    output.to_string()
}

fn read_lines(filename: &str) -> Vec<String> {
    /*
    Open a text file and return a Vector of Strings representing the individual
    lines.
    */
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file.");
    let mut lines: Vec<String> = Vec::new();
    for each in contents.split_terminator("\n").filter(|x| x.len() > 0).collect::<Vec<&str>>() {
        lines.push(each.trim().to_string());
    }
    lines
}


fn trinary_compare(left: i32, right: i32) -> i32 {
    /*
    Comparison logic for recursive_compare(). 1 means correctly ordered, -1 is
    incorrectly ordered, 0 is neutral.
    */
    let mut return_val: i32 = 0;
    if left < right {
        return_val = 1;
    }else if left > right {
        return_val = -1;
    }
    return return_val
}

fn minimum(a: usize, b:usize) -> usize {
    /*
    Return the smaller of the two parameters.
    */
    if a < b {
        return a;
    }
    b
}

fn recursive_compare(left: PacketElement, right: PacketElement) -> i32 {
    /*
    This function contains the real logic behind the process_lines() and 
    process_lines2() functions. It recursively compares the two packets based
    on the rules specified in Day 13, Part 1 (and reused in Part 2).
    */
    match (left, right) {
        // if comparing two numbers, left < right.
        (PacketElement::Number(left_int), PacketElement::Number(right_int)) => {
            //println!("Result based on Num vs Num comparison");
            return trinary_compare(left_int, right_int);
        },
        // if left is a list and right is a number, wrap right in a list and then compare again.
        (PacketElement::List(left_list), PacketElement::Number(right_int)) => {
            //println!("Result based on List vs Num comparison");
            return recursive_compare(PacketElement::List(left_list), PacketElement::List(vec![PacketElement::Number(right_int)]));
        },
        // if right is a list and left is a number, wrap left in a list and then compare again.
        (PacketElement::Number(left_int), PacketElement::List(right_list)) => {
            //println!("Result based on Num vs List comparison");
            return recursive_compare(PacketElement::List(vec![PacketElement::Number(left_int)]), PacketElement::List(right_list));
        },
        // when comparing two lists, check each index until you find a pair of elements that are not equal to each other
        (PacketElement::List(left_list), PacketElement::List(right_list)) => {
            for index in 0..minimum(left_list.len(), right_list.len()) {
                //println!("Checking index {} of {} and {}", index, left_list.len(), right_list.len());
                let my_result: i32 = recursive_compare(left_list[index].clone(), right_list[index].clone());
                if my_result != 0 {
                    //println!("Result based on recursive result {}", my_result);
                    return my_result;
                }
            }
            // all elements in the two lists were equal to each other, so 
            // they're in order if the left list is shorter than the right list.
            //println!("Result based on length {} vs {}", left_list.len(), right_list.len());
            return trinary_compare(left_list.len() as i32, right_list.len() as i32);
        }
    }
}


fn parse_packet(raw_packet: String) -> PacketElement {
    /*
    Take the string representation of a packet and convert it to a 
    PacketElement. This function uses recursive calls to simplify this process.
    */
    //println!("Parsing {}", raw_packet);
    if raw_packet.as_bytes()[0] as char != '[' {
        //println!("parse_packet() called on number {}", raw_packet);
        return PacketElement::Number(i32::from_str(&raw_packet).unwrap());
    }else {
        //println!("parse_packet() called on list {}", raw_packet);
    }
    //let mut my_packet = PacketElement::List(Vec::<PacketElement>::new());
    let mut elements: Vec<PacketElement> = Vec::<PacketElement>::new();
    let mut left_index: usize = 1;
    let mut right_index: usize = 1;
    let mut recursion_depth: usize = 0;
    while left_index < raw_packet.len() {
        //println!("Looking at {} at index {}", raw_packet.as_bytes()[right_index] as char, right_index);
        match raw_packet.as_bytes()[right_index] as char {
            ']' => {
                break;
            },
            '[' => {
                right_index = left_index + 1;
                recursion_depth += 1;
                while recursion_depth > 0 {
                    match raw_packet.as_bytes()[right_index] as char {
                        '[' => {
                            recursion_depth += 1;
                        },
                        ']' => {
                            recursion_depth -= 1;
                        }
                        _ => {
                            
                        }
                    };
                    right_index += 1;
                }
                //println!("Calling parse_packet for {}[{}..{}]", raw_packet, left_index, right_index);
                let my_sub_packet: PacketElement = parse_packet((raw_packet.as_str()[left_index..right_index]).to_string());
                elements.push(my_sub_packet);
                left_index = right_index + 1;
            },
            ',' => {
                elements.push(parse_packet((raw_packet.as_str()[left_index..right_index]).to_string()));
                left_index = right_index + 1;
            }
            _ => {

            }
        };
        right_index += 1;
    }
    if right_index > left_index {
        //println!("Catching single-entry in {}[{}..{}]", raw_packet, left_index, right_index);
        elements.push(parse_packet((raw_packet.as_str()[left_index..right_index]).to_string()));
    }
    let my_packet = PacketElement::List(elements);
    my_packet
}


fn process_lines(lines: &Vec<String>) -> i32 {
    /*
    This takes a list of packets which are lists holding any mixture of lists and integers (e.g., [], [[]], [2, [3,[4]]], etc). It then looks at pairs of packets, determining if they are in the right order or not based on multiple rules. The return value is the sum of the indices of all matching pairs, where the first two packets are index 1, the 3rd and 4th packets make up the pair at index 2, etc.

    See Part 1 of https://adventofcode.com/2022/day/13
    */
    let mut pair_index: i32 = 0;
    let mut pair_indices_sum: i32 = 0;

    // check each pair of packets to determine if they're in the right order
    for packet_pair in lines.chunks(2) {
        pair_index += 1; // we're counting from 1
        //println!("{}",packet_pair[0]);
        //println!("{}",packet_pair[1]);

        // check if the packets in this pair are correctly ordered
        if recursive_compare(parse_packet(packet_pair[0].clone()), parse_packet(packet_pair[1].clone())) == 1 {
            //println!("Pair {} match", pair_index);
            pair_indices_sum += pair_index;
        }else {
            //println!("Pair {} do NOT match", pair_index);
        }
    }

    pair_indices_sum
}

fn process_lines2(lines: &Vec<String>) -> i32 {
    /*
    Building on the logic of process_lines(), now we add two divider packets ([[2]] and [[6]]) to the end of the list of packets, then sort them using the rules from Part 1. The return value is the index (counting from 1) of divider packet [[2]] times the index of divider packet [[6]].

    See Part 2 of https://adventofcode.com/2022/day/13
    */
    let mut packet_vec: Vec<PacketElement>;
    let mut swapped: bool;

    // parse all the packets
    packet_vec = lines.into_iter().map(|x| parse_packet(x.clone())).collect();

    // put the divider packets in
    packet_vec.push(parse_packet("[[2]]".to_string()));
    packet_vec.push(parse_packet("[[6]]".to_string()));

    // bubble sort them
    loop {
        swapped = false;
        for index in 0..packet_vec.len()-1 {
            if recursive_compare(packet_vec[index].clone(), packet_vec[index+1].clone()) == -1 {
                // swap index and index + 1
                let temp: PacketElement = packet_vec[index].clone();
                packet_vec[index] = packet_vec[index+1].clone();
                packet_vec[index+1] = temp.clone();
                swapped = true;
                break;
            }else {
                //println!("Not swapping {} and {}", print_packet_element(packet_vec[index].clone()), print_packet_element(packet_vec[index+1].clone()));
            }
        }
        if !swapped {
            break;
        }
    }

    // Get the indices of the divider packets
    // It would be faster to use a loop and check both values at once, but this
    // is simpler and easier to troubleshoot
    let index_2: usize = packet_vec.iter().position(|x| print_packet_element(x.clone()) == "[[2]]").unwrap() + 1;
    let index_6: usize = packet_vec.iter().position(|x| print_packet_element(x.clone()) == "[[6]]").unwrap() + 1;

    println!("Found [[2]] at index {}", index_2);
    println!("Found [[6]] at index {}", index_6);

    (index_2 * index_6) as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_lines_short() {
        let lines = read_lines("day13_input_short.txt");
        assert_eq!(process_lines(&lines), 13);
    }

    #[test]
    fn test_process_lines_full() {
        let lines = read_lines("day13_input.txt");
        assert_eq!(process_lines(&lines), 5557);
    }

    #[test]
    fn test_process_lines2_short() {
        let lines = read_lines("day13_input_short.txt");
        assert_eq!(process_lines2(&lines), 140);
    }

    #[test]
    fn test_process_lines2_full() {
        let lines = read_lines("day13_input.txt");
        assert_eq!(process_lines2(&lines), 22425);
    }

    #[test]
    fn test_trinary_compare_01() {
        assert_eq!(trinary_compare(3, 4), 1);
        assert_eq!(trinary_compare(4, 3), -1);
        assert_eq!(trinary_compare(4, 4), 0);
    }

    #[test]
    fn test_packet_element_enum_01() {
        let sub_list: PacketElement = PacketElement::List(vec![PacketElement::Number(2)]);
        if let PacketElement::List(ref my_list) = sub_list {
            if let PacketElement::Number(my_val) = my_list[0] {
                //println!("sub_list[0]: {}", my_val);
                assert_eq!(my_val, 2);
            }else {
                assert!(false);
            }
        }
        assert_eq!(print_packet_element(sub_list), "[2]".to_string());
    }

    #[test]
    fn test_packet_element_enum_02() {
        let sub_list: PacketElement = PacketElement::List(vec![PacketElement::Number(2)]);
        if let PacketElement::List(mut my_list) = sub_list {
            my_list.push(PacketElement::Number(3));
            if let PacketElement::Number(my_val) = my_list[0] {
                //println!("sub_list[0]: {}", my_val);
                assert_eq!(my_val, 2);
            }else {
                assert!(false);
            }
            if let PacketElement::Number(my_val) = my_list[1] {
                //println!("sub_list[0]: {}", my_val);
                assert_eq!(my_val, 3);
            }else {
                assert!(false);
            }
        }
    }

    #[test]
    fn test_packet_element_enum_03() {
        let mut val: i32 = -1;
        let a_list: PacketElement = PacketElement::List(vec![PacketElement::Number(2)]);
        let sub_list: PacketElement = PacketElement::List(vec![a_list.clone()]);
        if let PacketElement::List(my_list) = sub_list.clone() {
            if let PacketElement::List(ref my_sub_list) = my_list[0] {
                if let PacketElement::Number(my_val) = my_sub_list[0] {
                    //println!("sub_list[0]: {}", my_val);
                    val = my_val;
                }
            }
        }
        assert_eq!(val, 2);
        assert_eq!(print_packet_element(sub_list), "[[2]]".to_string());
    }

    #[test]
    fn test_parse_packet_01() {
        let x: PacketElement = parse_packet("[]".to_string());
        if let PacketElement::List(ref my_list) = x {
            assert_eq!(my_list.len(), 0);
        }else {
            assert!(false);
        }
        assert_eq!(print_packet_element(x), "[]".to_string());
    }

    #[test]
    fn test_parse_packet_2() {
        let x: PacketElement = parse_packet("[[]]".to_string());
        if let PacketElement::List(ref my_list) = x {
            assert_eq!(my_list.len(), 1);
        }else {
            assert!(false);
        }
        assert_eq!(print_packet_element(x), "[[]]".to_string());
    }

    #[test]
    fn test_parse_packet_03() {
        let x: PacketElement = parse_packet("[[1]]".to_string());
        if let PacketElement::List(ref my_list) = x {
            assert_eq!(my_list.len(), 1);
        }else {
            assert!(false);
        }
        assert_eq!(print_packet_element(x), "[[1]]".to_string());
    }

    #[test]
    fn test_parse_packet_04() {
        let x: PacketElement = parse_packet("[[1,[2]]]".to_string());
        if let PacketElement::List(my_list) = x {
            assert_eq!(my_list.len(), 1);
            /*if let PacketElement::List(my_sub_list) = my_list[0] {
                assert_eq!(my_sub_list.len(), 2);
            }*/
        }
    }
    
    #[test]
    fn test_recursive_compare_01() {
        // [[1],[2,3,4]] vs [[1],4]
        let left_packet = parse_packet("[[1],[2,3,4]]".to_string());
        let right_packet = parse_packet("[[1],4]".to_string());
        assert_eq!(recursive_compare(left_packet.clone(), right_packet.clone()), 1);
        assert_eq!(recursive_compare(right_packet, left_packet), -1);
    }
    
    #[test]
    fn test_recursive_compare_02() {
        // [9] vs [[6]]
        let left_packet = parse_packet("[9]".to_string());
        let right_packet = parse_packet("[[6]]".to_string());
        assert_eq!(recursive_compare(left_packet.clone(), right_packet.clone()), -1);
        assert_eq!(recursive_compare(right_packet, left_packet), 1);
    }

    #[test]
    fn test_recursive_compare_03() {
        // [[6]] vs [7, 7, 7]
        let left_packet = parse_packet("[[6]]".to_string());
        let right_packet = parse_packet("[7,7,7]".to_string());
        assert_eq!(recursive_compare(left_packet.clone(), right_packet.clone()), 1);
        assert_eq!(recursive_compare(right_packet, left_packet), -1);
    }

    #[test]
    fn test_print_packet_element_01() {
        let packet: PacketElement = parse_packet("[[1],[2,3,4]]".to_string());
        let result: String = print_packet_element(packet);
        //println!("{}", result);
        assert_eq!(result, "[[1],[2,3,4]]");
    }
}

pub fn main() {
    let result = read_lines("day13_input.txt");
    println!("Day 13:");
    println!("Part 1 - The sum of the indices of the pairs in the right order is: {}", process_lines(&result));
    println!("Part 2 - The decoder key is: {}", process_lines2(&result));
}