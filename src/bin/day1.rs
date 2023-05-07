#[derive(Debug, PartialEq)]
struct Elf {
    number: usize,
    calories: i32
}

// Take input, turn into index (+1) of nth elf and how much food they're carrying
fn parse_input(input: &String) -> Vec<Elf> {
    let mut elves: Vec<Elf> = vec![Elf{ number: 1, calories: 0 }];
    let mut index = 0;
    for line in input.lines() {
        if line == "" {
            index += 1;
            elves.push(Elf{ number: index + 1, calories: 0 });
        } else {
            elves[index].calories += line.parse::<i32>().expect("s will always be a number");
        }
    }
    elves
}

fn get_top_elf(input: &String) -> Elf {
    parse_input(input).into_iter().max_by(|a, b| a.calories.cmp(&b.calories)).unwrap()
}

fn top_three_elves(input: &String) -> (Elf, Elf, Elf) {
    let mut elves = parse_input(input);
    elves.sort_by(|a, b| {
        b.calories.cmp(&a.calories)
    });
    let mut top = elves.into_iter().take(3);
    (top.next().unwrap(), top.next().unwrap(), top.next().unwrap())
}

fn part_1(filename: &str) -> Elf {
    let input = std::fs::read_to_string(filename).expect("Invalid filename given");

    get_top_elf(&input)
}

fn part_2(filename: &str) -> (Elf, Elf, Elf) {
    let input = std::fs::read_to_string(filename).expect("Invalid filename given");
    
    top_three_elves(&input)
}

#[test]
fn day_1_part_1() {
    assert_eq!(24000, part_1("input/day1test").calories);
}

#[test] 
fn day_1_part_2() {
    let (first, second, third) = part_2("input/day1test");
    assert_eq!(Elf{ number: 4, calories: 24000 }, first);
    assert_eq!(Elf{ number: 3, calories: 11000 }, second);
    assert_eq!(Elf{ number: 5, calories: 10000 }, third);
    assert_eq!(45000, first.calories + second.calories + third.calories);
}

fn main() {
    println!("The most calories an elf is carrying is {}", part_1("input/day1").calories);
    println!("The total number of calories carried by the three elves carrying the most calories is {}", {
        let (first, second, third) = part_2("input/day1");
        first.calories + second.calories + third.calories
    });
}