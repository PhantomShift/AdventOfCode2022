// Alternative solution that uses a queue to keep track of unique characters instead of counting characters present in a slice
use std::collections::VecDeque;

fn get_data_start(msg: &str, offset: usize) -> usize {
    let mut recorded = VecDeque::new();
    for (index, char) in msg.chars().enumerate() {
        while recorded.contains(&char) {
            recorded.pop_front();
        }
        recorded.push_back(char);
        if recorded.len() < offset { continue }
        else { return index + 1 }
    }
    panic!("Never found start of data");
}

#[test]
fn day_6_part_1() {
    let test_cases = [
        ("bvwbjplbgvbhsrlpgdmjqwftvncz", 5),
        ("nppdvjthqldpwncqszvftbrmjlhg", 6),
        ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 10),
        ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 11)
    ];
    for (input, expected) in test_cases {
        assert_eq!(expected, get_data_start(input, 4));
    }
}

#[test]
fn day_6_part_2() {
    let test_cases = [
        ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 19),
        ("bvwbjplbgvbhsrlpgdmjqwftvncz", 23),
        ("nppdvjthqldpwncqszvftbrmjlhg", 23),
        ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 29),
        ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 26)
    ];
    for (input, expected) in test_cases {
        assert_eq!(expected, get_data_start(input, 14));
    }
}

fn main() {
    println!("The start of the packet is at {}", {
        get_data_start(std::fs::read_to_string("input/day6").unwrap().as_str(), 4)
    });

    println!("The start of the message is at {}", {
        get_data_start(std::fs::read_to_string("input/day6").unwrap().as_str(), 14)
    });
}