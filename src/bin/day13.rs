use std::cmp::Ordering;
use advent_of_code2022::utils::split_lines_group;

#[derive(Debug)]
enum PacketItem {
    Integer(i32),
    List(String)
}

/// turns n into a string and surrounds with "\[" and "\]"
fn listify(n: i32) -> String {
    String::from("[") + &n.to_string() + "]"
}

/// Gets slice that has removed `n` characters from start and end
fn trim_length<'a>(s: &'a str, n: usize) -> &'a str {
    &s[n..s.len() - n]
}

fn parse_packet(s: &str) -> Vec<PacketItem> {
    trim_length(s, 1)
    .split({
        let mut depth = 0;
        move |c| {
            match c {
                '[' => { depth += 1; false },
                ']' => { depth -= 1; false },
                ',' if depth == 0 => true,
                _ => false
            }
        }
    })
    .filter_map(|item| {
        match item.parse::<i32>() {
            Ok(num) => Some(PacketItem::Integer(num)),
            // This is necessary to correctly parse zero-length lists
            Err(_) if item.len() > 0 => Some(PacketItem::List(item.to_string())),
            Err(_) => None
        }
    })
    .collect::<Vec<PacketItem>>()
}

// returns Ordering::Less if correct, Ordering::Greater if incorrect, and Ordering::Equal if both items are of same value
fn compare_packets(left: &str, right: &str) -> std::cmp::Ordering {
    let left = parse_packet(left);
    let right = parse_packet(right);
    let max = left.len().max(right.len());
    for i in 0..max + 1 {
        let comparison = match (left.get(i), right.get(i)) {
            (Some(PacketItem::Integer(l)), Some(PacketItem::Integer(r))) => l.cmp(r),
            (Some(PacketItem::Integer(l)), Some(PacketItem::List(r))) => compare_packets(&listify(*l), r),
            (Some(PacketItem::List(l)), Some(PacketItem::Integer(r))) => compare_packets(l, &listify(*r)),
            (Some(PacketItem::List(l)), Some(PacketItem::List(r))) => compare_packets(l, r),
            (None, Some(_)) => Ordering::Less,
            (Some(_), None) => Ordering::Greater,
            (None, None) => Ordering::Equal
        };

        if !(comparison == Ordering::Equal) {
            return  comparison;
        }
    }
    
    Ordering::Equal
}

#[test]
fn day_13_part_1() {
    let pairs = std::fs::read_to_string("input/day13test").expect("file should exist");
    let expected_results = [
        Ordering::Less,
        Ordering::Less,
        Ordering::Greater,
        Ordering::Less,
        Ordering::Greater,
        Ordering::Less,
        Ordering::Greater,
        Ordering::Greater
    ];
    let expected_correct_sum = 13;
    let mut calculated_correct_sum = 0;
    for (index, pair) in split_lines_group(&pairs, 3).iter().enumerate() {
        let mut l = pair.lines();
        let (left, right) = (l.next().expect("pair should have left item"), l.next().expect("pair should have right item"));
        let result = compare_packets(left, right);
        assert_eq!(expected_results[index], result);
        if result == Ordering::Less {
            calculated_correct_sum += index + 1;
        }
    }

    assert_eq!(expected_correct_sum, calculated_correct_sum);
}

#[test]
fn day_13_part_2() {
    let input = std::fs::read_to_string("input/day13test").expect("file should exist");
    let mut packets = input.lines().filter(|line| line.len() > 0).collect::<Vec<&str>>();
    // "Divisor packets"
    packets.push("[[2]]");
    packets.push("[[6]]");
    packets.sort_by(|a, b| compare_packets(a, b));
    let product = packets.iter().enumerate().filter_map(|(index, packet)| {
        match packet {
            &"[[2]]" | &"[[6]]" => Some(index + 1),
            _ => None
        }
    })
    .product::<usize>();

    assert_eq!(140, product);
}

fn main() {
    let pairs = std::fs::read_to_string("input/day13").expect("file should exist");
    let sum = split_lines_group(&pairs, 3)
        .iter()
        .enumerate()
        .filter_map(|(index, s)| {
            let mut lines = s.lines();
            let left = lines.next().expect("pair should have left item");
            let right = lines.next().expect("pair should have right item");
            match compare_packets(left, right) {
                Ordering::Less => Some(index + 1),
                _ => None
            }
        })
        .sum::<usize>();
    
    println!("The sum of indices of pairs that are ordered correctly is {}", sum);

    let mut packets = pairs.lines().filter(|line| line.len() > 0).collect::<Vec<&str>>();
    // "Divisor packets"
    packets.push("[[2]]");
    packets.push("[[6]]");
    packets.sort_by(|a, b| compare_packets(a, b));
    let product = packets.iter().enumerate().filter_map(|(index, packet)| {
        match packet {
            &"[[2]]" | &"[[6]]" => Some(index + 1),
            _ => None
        }
    })
    .product::<usize>();

    println!("The product of the indices of the divisor packets is {}", product);
}