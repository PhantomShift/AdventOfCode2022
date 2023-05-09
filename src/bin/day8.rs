use std::slice::Iter;
use std::collections::HashSet;

#[derive(Clone, Copy)]
enum Towards {
    X(i32),
    Y(i32)
}

enum Direction {
    FromTop(usize),
    FromBottom(usize),
    FromLeft(usize),
    FromRight(usize)
}

struct Vec2D<T: Copy> {
    contents: Vec<Vec<T>>
}

impl<T: Copy> Vec2D<T> {
    fn new() -> Vec2D<T> {
        Vec2D { contents: Vec::new() }
    }

    fn push_row(&mut self, row: Vec<T>) {
        self.contents.push(row);
    }
    
    fn iter_row(&self, index: usize) -> Iter<'_, T> {
        self.contents[index].iter()
    }

    // Returns a copy of a column
    fn column(&self, index: usize) -> Vec<T> {
        self.contents.iter().map(|v| v[index]).collect::<Vec<T>>()
    }

    fn get(&self, x: usize, y: usize) -> T {
        self.contents[y][x]
    }
}

impl Vec2D<u32> {
    fn from_str(s: &str) -> Self {
        let mut r = Vec2D::new();
        for line in s.lines() {
            let mut v = Vec::new();
            for char in line.chars() {
                v.push(char.to_digit(10).expect("character should be a number"));
            }
            r.push_row(v);
        }
        r
    }

    fn to_string(&self) -> String {
        let mut s = String::new();
        for index in 0..self.contents.len() {
            for n in self.iter_row(index) {
                s.push_str(n.to_string().as_str());
            }
            s.push_str("\n");
        }
        s
    }

    fn get_visible_from(&self, direction: Direction) -> Vec<(usize, usize)> {
        let v = match direction {
            Direction::FromTop(index) => self.column(index).clone(),
            Direction::FromBottom(index) => self.column(index).into_iter().rev().collect(),
            Direction::FromLeft(index) => self.contents[index].clone(),
            Direction::FromRight(index) => self.contents[index].clone().into_iter().rev().collect(),
        };

        let maximum = *v.iter().max().unwrap();
        let mut visible_coords = Vec::new();
        let mut tallest = 0;
        for (index, &num) in v.iter().enumerate() {
            if index == 0 || num > tallest {
                tallest = num;
                match direction {
                    Direction::FromTop(i) => visible_coords.push((i, index)),
                    Direction::FromBottom(i) => visible_coords.push((i, v.len() - index - 1)),
                    Direction::FromLeft(i) => visible_coords.push((index, i)),
                    Direction::FromRight(i) => visible_coords.push((v.len() - index - 1, i))
                }
                if tallest == maximum { break; }
            }
        }

        visible_coords
    }

    fn get_all_visible(&self) -> HashSet<(usize, usize)> {
        let mut visible_coords = HashSet::new();
        for x in 0..self.contents[0].len() {
            for coord in self.get_visible_from(Direction::FromTop(x)) {
                visible_coords.insert(coord);
            }
            for coord in self.get_visible_from(Direction::FromBottom(x)) {
                visible_coords.insert(coord);
            }
        }
        for y in 0..self.contents.len() {
            for coord in self.get_visible_from(Direction::FromLeft(y)) {
                visible_coords.insert(coord);
            }
            for coord in self.get_visible_from(Direction::FromRight(y)) {
                visible_coords.insert(coord);
            }
        }

        visible_coords
    }

    fn get_viewing_distance(&self, x: usize, y: usize, t: Towards) -> i32 {
        let mut distance = 0;
        let height = self.get(x, y);
        match t {
            Towards::X(step) => {
                let mut n = step;
                while self.contents[0].get((x as i32 + n) as usize).is_some() {
                    distance += 1;
                    if self.get((x as i32 + n) as usize, y) == height { break; }
                    n += step;
                }
            }
            Towards::Y(step) => {
                let mut n = step;
                while self.contents.get((y as i32 + n) as usize).is_some() {
                    distance += 1;
                    if self.get(x, (y as i32 + n) as usize) == height { break; }
                    n += step;
                }
            }
        }

        distance
    }

    fn get_scenic_score(&self, x: usize, y: usize) -> i32 {
        vec![Towards::X(-1), Towards::X(1), Towards::Y(-1), Towards::Y(1)]
            .iter()
            .map(|&t| self.get_viewing_distance(x, y, t))
            .product()
    }
}

#[test]
fn day_8_part_1() {
    let test_input =
"30373
25512
65332
33549
35390";
    let v = Vec2D::from_str(test_input);

    assert_eq!(21, v.get_all_visible().len());

    // Number of trees visible from left and right
    let expect_left_expect_right = [
        (2, 2),
        (2, 2),
        (1, 4),
        (3, 1),
        (3, 2)
    ];
    for (row_num, (expected_left, expected_right)) in expect_left_expect_right.iter().enumerate() {
        assert_eq!(expected_left, &v.get_visible_from(Direction::FromLeft(row_num)).len());
        assert_eq!(expected_right, &v.get_visible_from(Direction::FromRight(row_num)).len());
    }
}

#[test]
fn day_8_part_2() {
    let test_input =
"30373
25512
65332
33549
35390";
    let v = Vec2D::from_str(test_input);
    let expected_max = 8;
    let expected_height = 5;
    let mut calc_max = 0;
    let mut found_height = 0;
    for (x, y) in v.get_all_visible() {
        let score = v.get_scenic_score(x, y);
        if score > calc_max {
            calc_max = score;
            found_height = v.get(x, y);
        }
    }

    assert_eq!(expected_height, found_height);
    assert_eq!(expected_max, calc_max);
}

fn main() {
    let trees = Vec2D::from_str(std::fs::read_to_string("input/day8").expect("input file should exist").as_str());
    let visible_trees = trees.get_all_visible();
    println!("The number of trees visible from outside is {}", visible_trees.len());

    println!("The greatest scenic score possible is {}", {
        visible_trees.iter()
            .map(|(x, y)| trees.get_scenic_score(*x, *y))
            .max().unwrap()
    })
}