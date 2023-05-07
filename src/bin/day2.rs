
#[derive(PartialEq, Clone)]
enum Hand {
    Rock,
    Paper,
    Scissors
}

impl<'a> Hand {
    fn new(c: char) -> Self {
        match c {
            'A' | 'X' => Hand::Rock,
            'B' | 'Y' => Hand::Paper,
            'C' | 'Z' => Hand::Scissors,
            _ => panic!("Invalid character given for Hand")
        }
    }

    fn get_effective(self: &Self) -> Self {
        match self {
            Hand::Rock => Hand::Scissors,
            Hand::Paper => Hand::Rock,
            Hand::Scissors => Hand::Paper
        }
    }

    fn get_weakness(self: &Self) -> Self {
        match self {
            Hand::Rock => Hand::Paper,
            Hand::Paper => Hand::Scissors,
            Hand::Scissors => Hand::Rock
        }
    }

    fn value_instruction(opponent: &Self, instruction: char) -> i32 {
        let you = match instruction {
            'X' => opponent.get_effective(),
            'Y' => opponent.clone(),
            'Z' => opponent.get_weakness(),
            _ => panic!("Invalid character instruction given for value_instruction")
        };

        Hand::evaluate(opponent, &you)
    }

    fn get_value(self: &Self) -> i32 {
        match self {
            Hand::Rock => 1,
            Hand::Paper => 2,
            Hand::Scissors => 3
        }
    }

    fn winner(left: &'a Self, right: &'a Self) -> Option<&'a Self> {
        match (left, right) {
            (Hand::Rock, Hand::Paper) | (Hand::Paper, Hand::Scissors) | (Hand::Scissors, Hand::Rock) => {
                Some(right)
            }
            (Hand::Rock, Hand::Scissors) | (Hand::Paper, Hand::Rock) | (Hand::Scissors, Hand::Paper) => {
                Some(left)
            }
            (_, _) => None
        }
    }

    fn evaluate(opponent: &Self, you: &Self) -> i32 {
        let winner = Hand::winner(opponent, you);
        let outcome_bonus = if winner.is_none() { 3 } else if winner.unwrap() == you { 6 } else { 0 };

        outcome_bonus + you.get_value()
    }
}

fn parse_line_to_score(line: &str) -> i32 {
    let (left, right) = line.split_once(' ').expect("Ill-formatted lines given");
    let opponent = Hand::new(left.chars().next().unwrap());
    let you = Hand::new(right.chars().next().unwrap());
    Hand::evaluate(&opponent, &you)
}

fn parse_line_to_score_updated(line: &str) -> i32 {
    let (left, right) = line.split_once(' ').expect("Ill-formatted lines given");
    let opponent = Hand::new(left.chars().next().unwrap());
    let instruction = right.chars().next().unwrap();
    Hand::value_instruction(&opponent, instruction)
}

#[test]
fn day_2_part_1() {
    let input = "A Y
B X
C Z";
    const EXPECTED_SCORES: [i32; 3] = [8, 1, 6];
    let mut total = 0;
    for (index, line) in input.lines().enumerate() {
        let score = parse_line_to_score(line);
        assert_eq!(EXPECTED_SCORES[index], score);
        total += score;
    }
    assert_eq!(total, EXPECTED_SCORES.into_iter().sum());
}

#[test]
fn day_2_part_2() {
    let input = "A Y
B X
C Z";
    const EXPECTED_SCORES: [i32; 3] = [4, 1, 7];
    let mut total = 0;
    for (index, line) in input.lines().enumerate() {
        let score = parse_line_to_score_updated(line);
        assert_eq!(EXPECTED_SCORES[index], score);
        total += score;
    }
    assert_eq!(total, EXPECTED_SCORES.into_iter().sum());
}

fn main() {
    println!("The total score according to our initial assumption of how the strategy guide works would be {}", {
        let mut total = 0;
        for line in std::fs::read_to_string("input/day2").unwrap().lines() {
            total += parse_line_to_score(line)
        }
        total
    });
    println!("The total score according to how the strategy guide actually works is {}", {
        let mut total = 0;
        for line in std::fs::read_to_string("input/day2").unwrap().lines() {
            total += parse_line_to_score_updated(line)
        }
        total
    })
}