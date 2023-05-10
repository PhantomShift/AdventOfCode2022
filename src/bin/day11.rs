use std::collections::VecDeque;

#[derive(Debug)]
enum Operation {
    Multiply(i64),
    Add(i64),
    Square
}

impl Operation {
    fn from_str(s: &str) -> Self {
        let symbols = s.split_whitespace().collect::<Vec<&str>>();
        match (symbols[0], symbols[1], symbols[2].parse::<i64>()) {
            ("old", "*", Ok(n)) => Operation::Multiply(n),
            ("old", "+", Ok(n)) => Operation::Add(n),
            ("old", "*", Err(_)) => Operation::Square,
            _ => panic!("Invalid string given for construction of Operation")
        }
    }
}

#[derive(Debug)]
struct Monkey {
    id: usize,
    items: VecDeque<i64>,
    operation: Operation,
    divisor: i64,
    on_true: usize,
    on_false: usize,
    inspects: i64
}

impl Monkey {
    fn new(info: &str) -> Self {
        let mut lines = info.lines();
        let id = lines.next().unwrap().chars().filter_map(|c| c.to_digit(10)).next().expect("first line should contain a number id") as usize;
        let items = lines.next().unwrap()[18..].split(", ").filter_map(|s| s.parse::<i64>().ok()).collect::<VecDeque<i64>>();
        let operation = Operation::from_str(&lines.next().unwrap()[19..]);
        let divisor = lines.next().unwrap().split_whitespace().filter_map(|s| s.parse::<i64>().ok()).next().expect("fourth line should contain a number to divide by");
        let on_true = lines.next().unwrap().split_whitespace().filter_map(|s| s.parse::<usize>().ok()).next().expect("fifth line should contain a number id");
        let on_false = lines.next().unwrap().split_whitespace().filter_map(|s| s.parse::<usize>().ok()).next().expect("sixth line should contain a number id");

        Monkey {id, items, operation, divisor, on_true, on_false, inspects: 0}
    }

    /// First item is worry level, second item is monkey to throw to
    fn inspect<F>(&mut self, relief_method: F)  -> (i64, usize)
        where F: Fn(i64) -> i64 {
        let inspecting = self.items.pop_front().expect("monkey with no items should not be inspecting");
        self.inspects += 1;
        let changed = relief_method(match self.operation {
            Operation::Multiply(n) => inspecting * n,
            Operation::Add(n) => inspecting + n,
            Operation::Square => inspecting * inspecting
        });
        (changed, if changed % self.divisor == 0 { self.on_true } else { self.on_false })

    }

    // Needed this because borrowing two monkeys at once is annoying
    fn inspect_all<F>(&mut self, relief_method: F) -> Vec<(i64, usize)>
        where F: Fn(i64) -> i64 {
        let mut result = Vec::new();
        while !self.items.is_empty() {
            result.push(self.inspect(&relief_method));
        }

        result
    }
}

fn split_lines_group(s: &str, n: usize) -> Vec<String> {
    let lines = s.lines().collect::<Vec<&str>>();
    let mut result = Vec::new();
    for chunk in lines.chunks(n) {
        let to_add = chunk.iter().fold(String::new(), |a, e| a + e + "\n");
        result.push(to_add);
    }

    result
}

#[test]
fn day_11_new_monkey_test() {
    let monkey_info = "Monkey 0:
  Starting items: 85, 79, 63, 72
  Operation: new = old * 17
  Test: divisible by 2
    If true: throw to monkey 2
    If false: throw to monkey 6";
    let monkey = Monkey::new(monkey_info);
    println!("{:?}", monkey);
}

#[test]
fn day_11_split_group_test() {
    let input = std::fs::read_to_string("input/day11test").expect("file should exist");
    let infos = split_lines_group(&input, 7);
    for s in infos {
        println!("New monkey:\n {}", s);
    }
}

#[test]
fn day_11_part_1() {
    const EXPECTED_MONKEY_BUSINESS: i64 = 10605;
    let input = std::fs::read_to_string("input/day11test").expect("file should exist");
    let infos = split_lines_group(&input, 7);
    let mut monkeys = Vec::new();
    for info in infos {
        monkeys.push(Monkey::new(&info));
    }
    let rounds = 20;
    for _ in 0..rounds {
        for i in 0..monkeys.len() {
            let to_throw = monkeys[i].inspect_all(|n| n / 3);
            for (worry, id) in to_throw {
                monkeys[id].items.push_back(worry);
            }
        }
    }

    monkeys.sort_by(|a, b| b.inspects.cmp(&a.inspects));
    let mut sorted = monkeys.iter();
    let (first, second) = (sorted.next().unwrap(), sorted.next().unwrap());
    let monkey_business = first.inspects * second.inspects;
    assert_eq!(EXPECTED_MONKEY_BUSINESS, monkey_business);
}

#[test]
fn day_11_part_2() {
    const EXPECTED_MONKEY_BUSINESS: i64 = 2713310158;
    let input = std::fs::read_to_string("input/day11test").expect("file should exist");
    let infos = split_lines_group(&input, 7);
    let mut monkeys = Vec::new();
    for info in infos {
        monkeys.push(Monkey::new(&info));
    }

    // I legitimately would have never figured out how to do part 2 if I didn't look at the subreddit
    let divisor_product = monkeys.iter().map(|monkey| monkey.divisor).product::<i64>();
    let rounds = 10000;
    for _ in 0..rounds {
        for i in 0..monkeys.len() {
            let to_throw = monkeys[i].inspect_all(|n| n % divisor_product);
            for (worry, id) in to_throw {
                monkeys[id].items.push_back(worry);
            }
        }
    }

    monkeys.sort_by(|a, b| b.inspects.cmp(&a.inspects));
    let mut sorted = monkeys.iter();
    let (first, second) = (sorted.next().unwrap(), sorted.next().unwrap());
    let monkey_business = first.inspects as i64 * second.inspects as i64;
    assert_eq!(EXPECTED_MONKEY_BUSINESS, monkey_business);
}

fn main() {
    let input = std::fs::read_to_string("input/day11").expect("file should exist");
    let infos = split_lines_group(&input, 7);
    let mut monkeys = Vec::new();
    for info in infos.iter() {
        monkeys.push(Monkey::new(&info));
    }
    let rounds = 20;
    for _ in 0..rounds {
        for i in 0..monkeys.len() {
            let to_throw = monkeys[i].inspect_all(|n| n / 3);
            for (worry, id) in to_throw {
                monkeys[id].items.push_back(worry);
            }
        }
    }

    monkeys.sort_by(|a, b| b.inspects.cmp(&a.inspects));
    let mut sorted = monkeys.iter();
    let (first, second) = (sorted.next().unwrap(), sorted.next().unwrap());
    let monkey_business = first.inspects * second.inspects;
    
    println!("The level of monkey business after 20 rounds is {}", monkey_business);

    let mut monkeys = Vec::new();
    for info in infos.iter() {
        monkeys.push(Monkey::new(&info));
    }

    // Again, would have never figured this out on my own
    let divisor_product = monkeys.iter().map(|monkey| monkey.divisor).product::<i64>();
    let rounds = 10000;
    for _ in 0..rounds {
        for i in 0..monkeys.len() {
            let to_throw = monkeys[i].inspect_all(|n| n % divisor_product);
            for (worry, id) in to_throw {
                monkeys[id].items.push_back(worry);
            }
        }
    }

    monkeys.sort_by(|a, b| b.inspects.cmp(&a.inspects));
    let mut sorted = monkeys.iter();
    let (first, second) = (sorted.next().unwrap(), sorted.next().unwrap());
    let monkey_business = first.inspects as i64 * second.inspects as i64;
    
    println!("The level of monkey business after 10000 rounds is {}", monkey_business);
}