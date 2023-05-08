use std::ops::Range;

// Honestly I fully expected this to be a function that already existed for Ranges
fn range_contains_range<T: PartialOrd>(this: &Range<T>, other: &Range<T>) -> bool {
    this.start <= other.start && this.end >= other.end
}

fn get_range(s: &str) -> Range<i32> {
    let (left, right) = s.split_once('-').expect("Invalid input given for get_range");
    left.parse().unwrap()..right.parse().unwrap()
}

fn does_pair_fully_contain(left: &str, right: &str) -> bool {
    let left_range = get_range(left);
    let right_range = get_range(right);
    range_contains_range(&left_range, &right_range) || range_contains_range(&right_range, &left_range)
}

fn parse_assignment_pair(s: &&str) -> bool {
    let (left, right) = s.split_once(',').unwrap();
    does_pair_fully_contain(left, right)
}

// This one as well
fn range_overlaps_range<T: PartialOrd>(this: &Range<T>, other: &Range<T>) -> bool {
    range_contains_range(this, other)
    || this.start <= other.end && this.start >= other.start
    || this.end >= other.start && this.end <= other.end
}

fn does_pair_overlap(left: &str, right: &str) -> bool {
    let left_range = get_range(left);
    let right_range = get_range(right);
    range_overlaps_range(&left_range, &right_range)
}

fn parse_assignment_pair_updated(s: &&str) -> bool {
    let (left, right) = s.split_once(',').unwrap();
    does_pair_overlap(left, right)
}

#[test]
fn day_4_part_1() {
    let input = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";
    const EXPECTED_VALUE: usize = 2;
    let total = input.lines()
        .filter(parse_assignment_pair)
        .count();
    assert_eq!(EXPECTED_VALUE, total);
}

#[test]
fn day_4_part_2() {
    let input = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";
    const EXPECTED_VALUE: usize = 4;
    let total = input.lines()
        .filter(parse_assignment_pair_updated)
        .count();
    assert_eq!(EXPECTED_VALUE, total, "Calculated total was {}", total)
}

fn main() {
    println!("The number of assignment pairs where a range fully contains the other is {}", {
        std::fs::read_to_string("input/day4")
            .unwrap()
            .lines()
            .filter(parse_assignment_pair)
            .count()
    });

    println!("The number of assignment pairs that overlap is {}", {
        std::fs::read_to_string("input/day4")
            .unwrap()
            .lines()
            .filter(parse_assignment_pair_updated)
            .count()
    })
}