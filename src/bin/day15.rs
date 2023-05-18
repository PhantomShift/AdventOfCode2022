use advent_of_code2022::point::Point;
use std::collections::{HashMap, HashSet};

// Will probably start using regex to process inputs from now on if they're not very simple;
// lazy_static is used in conjunction so I'm not constantly recompiling the string
use regex;
use lazy_static::lazy_static;

fn get_points_on_distance_within_bounds(p: Point<i32>, distance: i32, min: i32, max: i32) -> HashSet<Point<i32>> {
    let r = min..=max;
    let mut points = HashSet::new();
    for y in p.y - distance..=p.y + distance {
        if !r.contains(&y) { continue; }
        let d = distance - (p.y - y).abs();
        if r.contains(&(p.x - d)) {
            points.insert(Point::new(p.x - d, y));
        }
        if r.contains(&(p.x + d)) {
            points.insert(Point::new(p.x + d, y));
        }
    }

    points
}

fn get_hash_set_intersections(sets: Vec<HashSet<Point<i32>>>) -> HashSet<Point<i32>> {
    let mut intersections: HashSet<Point<i32>> = HashSet::new();
    for (index, set) in sets.iter().enumerate() {
        for other in sets.iter().skip(index + 1) {
            intersections = &intersections | &(set & other);
        }
        println!("done calculating intersection for a set")
    }

    intersections
}

// WHY ARE YOU SO BIG
fn calc_tuning_freq(p: &Point<i32>) -> i64 {
    static MUL: i64 = 4000000;
    p.x as i64 * MUL + p.y as i64
}

// Used for day_15_part_2_output_downscaled
#[allow(unused)]
fn div_rounded(a: i32, b: i32) -> i32 {
    let d = a / b;
    let q = (a % b) as f32;
    if q / b as f32 > 0.5 {
        d + 1
    } else {
        d
    }
}

/// The `Point` value of `Sensor` represents the position of its closest `Beacon`
#[allow(unused)]
#[derive(Debug, Clone, Copy, PartialEq)]
enum Tile {
    Sensor(Point<i32>),
    Beacon,
    Empty,
    Unknown
}

impl From<Tile> for char {
    fn from(value: Tile) -> Self {
        match value {
            Tile::Sensor(_) => 'S',
            Tile::Beacon => 'B',
            Tile::Empty => '#',
            Tile::Unknown => '.',
        }
    }
}

struct CaveMap {
    min: Point<i32>,
    max: Point<i32>,
    points: HashMap<Point<i32>, Tile>,
    /// Value is the manhattan distance from its beacon
    signals: HashMap<Point<i32>, i32>
}

impl CaveMap {
    fn new(inputs: &str) -> Self {
        let mut points = HashMap::new();
        let mut signals = HashMap::new();
        let mut min_x = i32::MAX;
        let mut max_x = i32::MIN;
        let mut min_y = i32::MAX;
        let mut max_y = i32::MIN;
    
        for line in inputs.lines() {
            let capture = PUZZLE_CAPTURE.captures(line).unwrap();
            let sensor = Point::new(
                capture["sensor_x"].parse::<i32>().unwrap(),
                capture["sensor_y"].parse::<i32>().unwrap()
            );
            let beacon = Point::new(
                capture["beacon_x"].parse::<i32>().unwrap(),
                capture["beacon_y"].parse::<i32>().unwrap()
            );
            let distance = sensor.manhattan_distance(&beacon);
    
            min_x = min_x.min(sensor.x - distance);
            max_x = max_x.max(sensor.x + distance);
            min_y = min_y.min(sensor.y - distance);
            max_y = max_y.max(sensor.y + distance);
            
            points.insert(sensor, Tile::Sensor(beacon));
            points.insert(beacon, Tile::Beacon);
            signals.insert(sensor, distance);
        }
    
        CaveMap { min: Point::new(min_x, min_y), max: Point::new(max_x, max_y), points, signals }
    }

    // Exists to show how I originally attempted to solve part two
    #[allow(unused)]
    fn get_unknown_in_range(&self, min: Point<i32>, max: Point<i32>) -> Option<Point<i32>> {
        for y in min.y..=max.y {
            for x in min.x..=max.x {
                let point = Point::new(x, y);
                if self.is_point_unknown(point) {
                    return Some(point);
                }
            }
        }
        None
    }

    /// As optimized as I can get it with my knowledge anyways.
    /// On my machine, part two takes my machine about 40~50 seconds
    /// to process on **RELEASE** build.
    /// 
    /// Additionally, I have not ensured that this always works,
    /// I'm too dumb to figure that out.
    fn get_unknown_in_range_optimized(&self, min: i32, max: i32) -> Option<Point<i32>> {
        let mut borders = Vec::new();
        for (&signal, &distance) in self.signals.iter() {
            borders.push(get_points_on_distance_within_bounds(signal, distance, min, max));
            println!("made a set of borders");
        }
        println!("done making borders");
        let intersections = get_hash_set_intersections(borders);
        println!("done making intersections");

        for point in intersections.iter() {
            let p = Point::new(point.x - 1, point.y);
            if self.is_point_unknown(p) { return Some(p); }
            let p = Point::new(point.x + 1, point.y);
            if self.is_point_unknown(p) { return Some(p); }
        }

        None
    }

    fn get_empty_in_row(&self, row_number: i32) -> Vec<Point<i32>> {
        let mut empty = Vec::new();
        'range: for x in self.min.x..=self.max.x {
            let point = Point::from((x, row_number));
            if self.points.get(&point).is_some() { continue; }
            for (&signal_pos, &distance) in self.signals.iter() {
                if point.manhattan_distance(&signal_pos) <= distance {
                    empty.push(point);
                    continue 'range;
                }
            }
        }

        empty
    }

    fn is_point_unknown(&self, point: Point<i32>) -> bool {
        if self.points.get(&point).is_some() { return false; }
        for (&signal, &distance) in self.signals.iter() {
            if point.manhattan_distance(&signal) <= distance {
                return false;
            }
        }

        true
    }
}

lazy_static! {
    static ref PUZZLE_CAPTURE: regex::Regex = regex::Regex::new(r"Sensor at x=(?P<sensor_x>-?\d+), y=(?P<sensor_y>-?\d+): closest beacon is at x=(?P<beacon_x>-?\d+), y=(?P<beacon_y>-?\d+)").unwrap();
}

#[test]
fn day_15_regex_test() {
    let example = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15";
    let capture = PUZZLE_CAPTURE.captures(example);
    if let Some(cap) = capture {
        
        for m in cap.iter() {
            if let Some(m) = m {
                println!("{}", m.as_str());
            }
        }
    }
}

#[test]
fn day_15_part_1() {
    let inputs = std::fs::read_to_string("input/day15test").unwrap();
    let row_number = 10;
    let map = CaveMap::new(&inputs);
    let empty = map.get_empty_in_row(row_number);
    for y in map.min.y..=map.max.y {
        for x in map.min.x..=map.max.x {
            let t = map.points.get(&(x, y).into()).or(Some(&Tile::Unknown)).unwrap();
            if empty.contains(&(x, y).into()) {
                print!("{}", char::from(Tile::Empty));
            } else {
                print!("{}", char::from(*t));
            }
        }
        println!()
    }
    assert_eq!(26, map.get_empty_in_row(row_number).iter().count());
}

#[test]
fn day_15_part_2() {
    let inputs = std::fs::read_to_string("input/day15test").unwrap();
    let map = CaveMap::new(&inputs);

    for y in 0..=20 {
        for x in 0..=20 {
            if map.is_point_unknown((x, y).into()) {
                print!("{}", char::from(Tile::Unknown));
            } else {
                let t = map.points.get(&(x, y).into()).or(Some(&Tile::Empty)).unwrap();
                print!("{}", char::from(*t));
            }
        }
        println!()
    }

    // let unknown = map.get_unknown_in_range(Point::new(0, 0), Point::new(20, 20)).unwrap();
    let unknown = map.get_unknown_in_range_optimized(0, 20).unwrap();

    assert_eq!(Point::new(14, 11), unknown);
    assert_eq!(56000011, calc_tuning_freq(&unknown));
}

// I was REALLY struggling with part two man.
#[test]
fn day_15_part_2_output_downscaled() {
    let scaling_factor = 10000;
    let inputs = std::fs::read_to_string("input/day15").unwrap();
    let modified = PUZZLE_CAPTURE.replace_all(&inputs, |cap: &regex::Captures| {
        format!(
            "Sensor at x={}, y={}: closest beacon is at x={}, y={}",
            cap["sensor_x"].parse::<i32>().unwrap() / scaling_factor,
            cap["sensor_y"].parse::<i32>().unwrap() / scaling_factor,
            cap["beacon_x"].parse::<i32>().unwrap() / scaling_factor,
            cap["beacon_y"].parse::<i32>().unwrap() / scaling_factor,
        )
    });

    let map = CaveMap::new(&modified);
    for y in map.min.y..=map.max.y {
        for x in map.min.x..=map.max.x {
            if map.is_point_unknown((x, y).into()) {
                print!("{}", char::from(Tile::Unknown));
            } else {
                let t = map.points.get(&(x, y).into()).or(Some(&Tile::Empty)).unwrap();
                print!("{}", char::from(*t));
            }
        }
        println!()
    }
}

fn main() {
    let inputs = std::fs::read_to_string("input/day15").unwrap();
    let row_number = 2000000;
    let map = CaveMap::new(&inputs);
    let empty = map.get_empty_in_row(row_number).iter().count();

    println!("The number of tiles that cannot contain a beacon on row {row_number} is {empty}");
    
    let maximum = 4000000;
    // let unknown = map.get_unknown_in_range(Point::new(0, 0), Point::new(maximum, maximum)).unwrap();
    let unknown = map.get_unknown_in_range_optimized(0, maximum).unwrap();
    
    println!("The tuning frequency of the one tile out of range is {}", calc_tuning_freq(&unknown));
}