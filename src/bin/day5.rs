enum CrateMoverVersion {
    NineThousand,
    NineThousandOne
}

struct Instruction {
    crates: usize,
    start: usize,
    dest: usize
}

impl Instruction {
    fn new(s: &str) -> Self {
        let nums = s.split_whitespace()
            .filter(|s| s.parse::<usize>().is_ok())
            .map(|s| s.parse::<usize>().expect("Filter checked for numerical"))
            .collect::<Vec<usize>>();
        assert_eq!(nums.len(), 3, "Did not receive three numbers from instruction string");
        Instruction { crates: nums[0], start: nums[1] - 1, dest: nums[2] - 1 }
    }
}

fn parse_crate_string(s: &&str) -> Vec<(usize, char)> {
    let mut crates = Vec::new();
    for (index, char) in s.chars().enumerate() {
        if char.is_alphabetic() {
            crates.push((index / 4, char));
        }
    }

    crates
}

fn parse_input(input: &str, version: CrateMoverVersion) -> String {
    let (initial_state, instructions): (Vec<&str>, Vec<&str>) = input.lines()
        .filter(|s| s.len() > 0)
        .partition(|s| {
        s.contains('[') || s.starts_with(' ')
    });
    let num_stacks = (initial_state.iter().next().unwrap().len() + 1) / 4;
    let initial_state = initial_state.iter().rev()
            .filter(|s| s.contains('['))
            .map(parse_crate_string);
    let mut crates: Vec<Vec<char>> = Vec::new();
    for _ in 0..num_stacks {
        crates.push(Vec::new());
    }
    for crate_info in initial_state {
        for (pos, char) in crate_info {
            crates[pos].push(char);
        }
    }

    for instruction in instructions.iter().map(|s| Instruction::new(s)) {
        match version {
            CrateMoverVersion::NineThousand => {
                for _ in 0..instruction.crates {
                    let to_move = crates[instruction.start].pop().expect("Attempt to move non-existent crate");
                    crates[instruction.dest].push(to_move);
                }
            }
            CrateMoverVersion::NineThousandOne => {
                let mut to_move = Vec::new();
                for _ in 0..instruction.crates {
                    to_move.push(crates[instruction.start].pop().expect("Attempt to move non-existent crate"))
                }
                for to_move in to_move.iter().rev() {
                    crates[instruction.dest].push(*to_move);
                }
            }
        }
    }
    
    let mut message = String::new();
    for stack in crates {
        match stack.last() {
            Some(char) => message.push_str(char.to_string().as_str()),
            None => message.push_str(" ")
        }
    }
    
    message
}

#[test]
fn day_5_part_1() {
    const TEST_INPUT: &str =
"    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    const EXPECTED_CRATES: &str = "CMZ";
    
    assert_eq!(EXPECTED_CRATES, parse_input(TEST_INPUT, CrateMoverVersion::NineThousand));
}

#[test]
fn day_5_part_2() {
    const TEST_INPUT: &str =
"    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    const EXPECTED_CRATES: &str = "MCD";
    
    assert_eq!(EXPECTED_CRATES, parse_input(TEST_INPUT, CrateMoverVersion::NineThousandOne));
}

fn main() {
    println!("Top crates of each stack according to instruction ver. 9000 should be {}", {
        parse_input(std::fs::read_to_string("input/day5").unwrap().as_str(), CrateMoverVersion::NineThousand)
    });

    println!("Top crates of each stack according to instruction ver. 9001 should be {}", {
        parse_input(std::fs::read_to_string("input/day5").unwrap().as_str(), CrateMoverVersion::NineThousandOne)
    });
}