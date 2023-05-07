
fn calculate_priority(c: &char) -> i32 {
    match c {
        'a'..='z' => *c as i32 - 96,
        'A'..='Z' => *c as i32 - 38,
        _ => panic!("Invalid character given")
    }
}

fn get_rucksack_type(sack: &str) -> char {
    let (left, right) = sack.split_at(sack.len() / 2);
    left.chars()
        .filter(|c| right.contains(*c))
        .next()
        .expect("Both compartments did not share a type")
}

fn get_badge_type(first: &str, second: &str, third: &str) -> char {
    first.chars()
        .filter(|c| second.contains(*c))
        .filter(|c| third.contains(*c))
        .next()
        .expect("Group of elves did not share a badge type.")
}

#[test]
fn day_2_part_1() {
    let input = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";
    const EXPECTED_VALUES: [(char, i32); 6] = [
        ('p', 16),
        ('L', 38),
        ('P', 42),
        ('v', 22),
        ('t', 20),
        ('s', 19)
    ];
    let mut total = 0;
    for (index, line) in input.lines().enumerate() {
        let sack_type = get_rucksack_type(line);
        assert_eq!(EXPECTED_VALUES[index].0, sack_type);
        let priority = calculate_priority(&sack_type);
        assert_eq!(EXPECTED_VALUES[index].1, priority);
        total += priority;
    }
    assert_eq!(157, total)
}

#[test]
fn day_2_part_2() {
    let input = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";
    const EXPECTED_TYPES: [char; 2] = ['r', 'Z'];
    let mut lines = input.lines();
    let mut total = 0;
    for index in 0..input.lines().count() / 3 {
        // Inputs are assumed to be divisible by 3 as the elves
        // travel in groups of 3
        let first = lines.next().unwrap();
        let second = lines.next().unwrap();
        let third = lines.next().unwrap();
        let badge_type = get_badge_type(&first, &second, &third);
        assert_eq!(EXPECTED_TYPES[index], badge_type);
        total += calculate_priority(&badge_type);
    }
    assert_eq!(70, total);
}

fn main() {
    println!("The sum of priorities of the rucksacks' item types is {}", {
        let mut total = 0;
        for line in std::fs::read_to_string("input/day3").unwrap().lines() {
            total +=  calculate_priority(&get_rucksack_type(line));
        }
        total
    });

    println!("The sum of priorities of the badge types is {}", {
        let input = std::fs::read_to_string("input/day3").unwrap();
        let mut lines = input.lines(); 
        let mut total = 0;
        for _ in 0..input.lines().count() / 3 {
            // Inputs are assumed to be divisible by 3 as the elves
            // travel in groups of 3
            let first = lines.next().unwrap();
            let second = lines.next().unwrap();
            let third = lines.next().unwrap();
            let badge_type = get_badge_type(&first, &second, &third);
            total += calculate_priority(&badge_type);
        }
        total
    });
}