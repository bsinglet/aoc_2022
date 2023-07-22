use std::fs;

#[derive(Clone)]
enum PacketElement {
    List(Vec<PacketElement>),
    Number(i32),
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
    let mut return_val: i32 = 0;
    if left < right {
        return_val = 1;
    }else if left > right {
        return_val = -1;
    }
    return return_val
}

fn recursive_compare(left: PacketElement, right: PacketElement) -> i32 {
    match (left, right) {
        (PacketElement::Number(left_int), PacketElement::Number(right_int)) => {
            return trinary_compare(left_int, right_int);
        }
        (PacketElement::List(left_list), PacketElement::Number(right_int)) => {
            return recursive_compare(PacketElement::List(left_list), PacketElement::List(vec![PacketElement::Number(right_int)]));
        }
        (PacketElement::Number(left_int), PacketElement::List(right_list)) => {
            return recursive_compare(PacketElement::List(vec![PacketElement::Number(left_int)]), PacketElement::List(right_list));
        }
        (PacketElement::List(left_list), PacketElement::List(right_list)) => {
            for index in 0..left_list.len() {
                let my_result: i32 = recursive_compare(left_list[index].clone(), right_list[index].clone());
                if my_result != 0 {
                    return my_result;
                }
            }
            return trinary_compare(left_list.len() as i32, right_list.len() as i32);
        }
    }
}


fn parse_packet(raw_packet: String) -> PacketElement {
    let my_packet = PacketElement::List(Vec::<PacketElement>::new());
    // finish parsing packet
    my_packet
}


fn process_lines(lines: &Vec<String>) -> i32 {
    let mut pair_index: i32 = 0;
    let mut pair_indices_sum: i32 = 0;

    for packet_pair in lines.chunks(2) {
        pair_index += 1;
        // println!("{}",packet_pair[0]);
        // println!("{}",packet_pair[1]);

        // check if the packets in this pair are correctly ordered
        if recursive_compare(parse_packet(packet_pair[0].clone()), parse_packet(packet_pair[1].clone())) == 1 {
            pair_indices_sum += pair_index;
        }
    }

    pair_indices_sum
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
    fn test_trinary_compare_01() {
        assert_eq!(trinary_compare(3, 4), 1);
        assert_eq!(trinary_compare(4, 3), -1);
        assert_eq!(trinary_compare(4, 4), 0);
    }

    #[test]
    fn test_PacketElement_enum_01() {
        let sub_list: PacketElement = PacketElement::List(vec![PacketElement::Number(2)]);
        if let PacketElement::List(my_list) = sub_list {
            if let PacketElement::Number(my_val) = my_list[0] {
                println!("sub_list[0]: {}", my_val);
            }
        }
    }
}

pub fn main() {
    let result = read_lines("day13_input_short.txt");
    println!("Day 13:");
    println!("Part 1 - The sum of the indices of the pairs in the right order is: {}", process_lines(&result));
}