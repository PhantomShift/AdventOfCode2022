use std::collections::HashMap;

enum Direction {
    Left(usize),
    Right(usize),
    Up(usize),
    Down(usize)
}

impl Direction {
    fn from_str(s: &str) -> Self {
        let (left, right) = s.split_once(' ').expect("should be valid string");
        match (left, right.parse::<usize>()) {
            ("L", Ok(n)) => Direction::Left(n),
            ("R", Ok(n)) => Direction::Right(n),
            ("U", Ok(n)) => Direction::Up(n),
            ("D", Ok(n)) => Direction::Down(n),
            _ => panic!("invalid string given")
        }
    }
}
struct Rope {
    head: (i32, i32),
    knots: Vec<(i32, i32)>,
    traversed: HashMap<(i32, i32), bool>
}

impl Rope {
    fn new(length: usize) -> Self {
        assert!(length > 1, "Rope must have at least 2 knots");
        let mut rope = Rope {
            head: (0, 0),
            knots: Vec::new(),
            traversed: HashMap::new()
        };
        rope.traversed.insert((0, 0), true);
        for _ in 0..length {
            rope.knots.push((0, 0));
        }
        rope
    }

    fn move_head(&mut self, direction: Direction) {
        match direction {
            Direction::Left(n) => for _ in 0..n { self.move_to(self.head.0 - 1, self.head.1) }
            Direction::Right(n) => for _ in 0..n { self.move_to(self.head.0 + 1, self.head.1) }
            Direction::Up(n) => for _ in 0..n { self.move_to(self.head.0, self.head.1 + 1) }
            Direction::Down(n) => for _ in 0..n { self.move_to(self.head.0, self.head.1 - 1) }
        }
    }

    fn move_to(&mut self, x: i32, y: i32) {
        self.head = (x, y);
        self.knots[0] = (x, y);

        self.move_next(0);
    }

    fn move_next(&mut self, current_num: usize) {
        let (x, y) = self.knots[current_num];
        let mut knot = &mut self.knots[current_num + 1];
        if (x - knot.0).abs() == 2 && (y - knot.1).abs() == 1
        || (x - knot.0).abs() == 1 && (y - knot.1).abs() == 2
        || (x - knot.0).abs() > 1 || (y - knot.1).abs() > 1 {
            knot.0 += (x - knot.0).signum();
            knot.1 += (y - knot.1).signum();
        }
        if current_num == self.knots.len() - 2 {
            self.traversed.insert(self.knots[current_num + 1], true);
        } else {
            self.move_next(current_num + 1);
        }
    }

    // For debugging purposes
    #[allow(dead_code)]
    fn print(&self, size: i32) {
        for y in (-size..size + 1).rev() {
            for x in -size..size + 1 {
                if self.knots.contains(&(x, y)) {
                    print!("X");
                } else {
                    print!("#");
                }
            }
            print!("\n");
        }
        println!("{}", "-".repeat(size as usize * 2));
    }
}

#[test]
fn day_9_part_1() {
    let test_input = 
"R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";
    let expected_traversed = 13;
    let mut rope = Rope::new(2);
    for line in test_input.lines() {
        rope.move_head(Direction::from_str(line));
    }
    assert_eq!(expected_traversed, rope.traversed.len());
}

#[test]
fn day_9_part_2() {
    let test_input_1 = 
"R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";
    let test_input_2 = 
"R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";
    let expected_traversed_1 = 1;
    let mut rope = Rope::new(10);
    for line in test_input_1.lines() {
        rope.move_head(Direction::from_str(line));
        rope.print(5);
    }
    assert_eq!(expected_traversed_1, rope.traversed.len());

    let expected_traversed_2 = 36;
    let mut rope = Rope::new(10);
    for line in test_input_2.lines() {
        rope.move_head(Direction::from_str(line));
    }
    assert_eq!(expected_traversed_2, rope.traversed.len());
}

fn main() {
    println!("The number of positions visited by the tail at least once is {}", {
        let mut rope = Rope::new(2);
        for line in std::fs::read_to_string("input/day9").unwrap().lines() {
            rope.move_head(Direction::from_str(line));
        }
        rope.traversed.len()
    });

    println!("The number of positions visited by the 10th knot at least once is {}", {
        let mut rope = Rope::new(10);
        for line in std::fs::read_to_string("input/day9").unwrap().lines() {
            rope.move_head(Direction::from_str(line));
        }
        rope.traversed.len()
    });
}