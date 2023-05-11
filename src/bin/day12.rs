// Hello A* my old friend,
// I knew I'd probably see you again
use std::collections::{VecDeque, HashMap};

#[derive(Debug)]
struct Node<T> {
    // in a real implementation you'd probably use a UUID or something instead but eh
    id: usize,
    data: T,
}

impl<T> PartialEq for Node<T> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

struct Map<T> {
    nodes: Vec<Node<T>>,
    node_neighbors: Vec<Vec<usize>>
}

impl<T> Map<T> {
    fn set_neighbors(&mut self, a: &Node<T>, b: &Node<T>) {
        let a = self.get_index_of(a).unwrap();
        let b = self.get_index_of(b).unwrap();
        self.set_neighbors_by_index(a, b);
    }
    
    fn set_neighbors_by_index(&mut self, a: usize, b: usize) {
        self.node_neighbors[a].push(b);
        self.node_neighbors[b].push(a);
    }
    
    fn get_neighbor_indexes(&self, node: &Node<T>) -> &Vec<usize> {
        &self.node_neighbors[self.get_index_of(&node).unwrap()]
    }

    fn get_index_of(&self, node: &Node<T>) -> Option<usize> {
        self.nodes.iter().position(|n| n == node)
    }
}

struct AStarInfo {
    id: usize,
    local_score: i32,
    global_score: i32,
    visited: bool,
    parent: Option<usize>
}

impl AStarInfo {
    fn new(id: usize) -> Self {
        AStarInfo {
            id,
            local_score: i32::MAX,
            global_score: i32::MAX,
            visited: false,
            parent: None
        }
    }
}

impl PartialEq for AStarInfo {
    fn eq(&self, other: &Self) -> bool {
        self.global_score == other.global_score
    }
}

impl PartialOrd for AStarInfo {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.global_score.cmp(&other.global_score))
    }
}

fn a_star_solve<'a, T, D, H>(map: &'a Map<T>, start: &'a Node<T>, target: &'a Node<T>, distance: D, heuristic: H) -> Vec<&'a Node<T>>
    where
        D: Fn(&'a Node<T>, &'a Node<T>) -> i32,
        H: Fn(&'a Node<T>, &'a Node<T>) -> i32 {
    let mut tracker: HashMap<usize, AStarInfo> = HashMap::new();
    tracker.insert(map.get_index_of(start).unwrap(), AStarInfo {
        id: start.id,
        local_score: 0,
        global_score: heuristic(start, target),
        visited: false,
        parent: None
    });

    let mut unchecked = VecDeque::new();
    unchecked.push_back(start);
    let mut current = None;
    while !unchecked.is_empty() {
        current = unchecked.pop_front();
        let node = current.expect("non-empty queue should still have items");
        tracker.get_mut(&map.get_index_of(node).unwrap()).expect("tracking info for node should exist").visited = true;
        if node == target { break; }
        for index in map.get_neighbor_indexes(node) {
            let neighbor = &map.nodes[*index];
            let potential_score = tracker.get(&node.id).unwrap().local_score.checked_add(distance(node, neighbor)).unwrap_or(i32::MAX);
            let mut neighbor_info = tracker.entry(map.get_index_of(neighbor).unwrap()).or_insert(AStarInfo::new(neighbor.id));
            
            if !neighbor_info.visited && potential_score < neighbor_info.local_score {
                neighbor_info.parent = map.get_index_of(node);
                neighbor_info.local_score = potential_score;
                neighbor_info.global_score = potential_score + heuristic(neighbor, target);

                unchecked.push_back(neighbor);
            }
        }
        unchecked.make_contiguous().sort_by(|a, b| {
            let score_a = tracker.get(&a.id).unwrap().global_score;
            let score_b = tracker.get(&b.id).unwrap().global_score;
            score_a.cmp(&score_b)
        })
    }

    let mut checking_node = current.unwrap();
    let mut path = Vec::new();
    path.push(checking_node);
    while let Some(node_index) = tracker.get(&checking_node.id).unwrap().parent {
        let node = &map.nodes[node_index];
        path.push(node);
        checking_node = node;
    }

    path
}

#[derive(Debug)]
struct Hill {
    height: char,
    position: (i32, i32)
}

impl Hill {
    fn calc_height(c: char) -> i32 {
        match c {
            'a'..='z' => c as i32,
            'S' => 'a' as i32,
            'E' => 'z' as i32,
            _ => panic!("Invald character given for height")
        }
    }
}

fn construct_map(s: &str) -> Map<Hill> {
    let mut chars = Vec::new();
    for line in s.lines() {
        let mut row = Vec::new();
        for char in line.chars() {
            row.push(char);
        }
        chars.push(row);
    }
    let mut id = 0;
    let mut hills = Vec::new();
    for (y, row) in chars.iter().enumerate() {
        for (x, char) in row.iter().enumerate() {
            let hill = Node {
                id,
                data: Hill {
                    height: *char,
                    position: (x as i32, y as i32)
                }
            };
            hills.push(hill);
            id += 1;
        }
    }

    let mut hill_neighbors = Vec::new();
    for hill in hills.iter() {
        let mut neighbors = Vec::new();
        let pos = hill.data.position;
        for neighbor in hills.iter().filter(|&other| {
            let other_pos = other.data.position;
            (pos.0 - other_pos.0).abs() + (pos.1 - other_pos.1).abs() < 2
            && compare_hill_height(hill, other) == 1
        }) {
            neighbors.push(neighbor.id);
        }
        hill_neighbors.push(neighbors);
    }
    
    Map { nodes: hills, node_neighbors: hill_neighbors }
}

fn compare_hill_height(a: &Node<Hill>, b: &Node<Hill>) -> i32 {
    if Hill::calc_height(b.data.height) - Hill::calc_height(a.data.height) < 2 {
        1
    } else {
        i32::MAX
    }
}

// Height also needed to be taken into account to give it a bias towards moving upwards
fn hill_heuristic(a: &Node<Hill>, target: &Node<Hill>) -> i32 {
    (a.data.position.0 - target.data.position.0).abs()
    + (a.data.position.1 - target.data.position.1).abs()
    + Hill::calc_height(target.data.height) - Hill::calc_height(a.data.height)
}

impl<'a> Map<Hill> {
    fn get_start(&'a self) -> &'a Node<Hill> {
        self.nodes.iter().find(|n| n.data.height == 'S').unwrap()
    }

    fn get_end(&'a self) -> &'a Node<Hill> {
        self.nodes.iter().find(|n| n.data.height == 'E').unwrap()
    }
}

#[test]
fn day_12_part_1() {
    let map_str = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";
    let expected_steps = 31;
    let map = construct_map(map_str);
    let start = map.get_start();
    let end = map.get_end();

    let path = a_star_solve(&map, start, end, compare_hill_height, hill_heuristic);
    // Subtract one as path includes the start node
    assert_eq!(expected_steps, path.len() - 1);

    for y in 0..5 {
        for x in 0..8 {
            if let Some(c) = path.iter().find(|n| n.data.position == (x, y)) {
                print!("{}", c.data.height);
            } else {
                print!(".");
            }
        }
        print!("\n");
    }
}

// Part 2 is not very well optimized; on release build with actual puzzle input,
// it takes my machine a little over 2 seconds (and about a minute in debug)
// Attempting to reverse with the code I have present actually makes it run slower
// for whatever reason, may relate to how generic I made the code
// A speed optimization that could be made is caching the calculated distance
// between a given node and the target to be reused between runs,
// but I don't feel like figuring that out at 3am
#[test]
fn day_12_part_2() {
    let map_str = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";
    let expected_steps = 29;
    let map = construct_map(map_str);
    let end = map.get_end();

    let mut shortest = usize::MAX;
    for node in map.nodes.iter() {
        if node.data.height == 'a' {
            let path = a_star_solve(&map, node, end, compare_hill_height, hill_heuristic);
            if path[0] == end && path.len() - 1 < shortest {
                shortest = path.len() - 1;   
            }
        }
    }

    assert_eq!(expected_steps, shortest);
}

fn main() {
    let map = construct_map(&std::fs::read_to_string("input/day12").expect("file should exist"));
    let start = map.get_start();
    let end = map.get_end();
    let path = a_star_solve(&map, start, end, compare_hill_height, hill_heuristic);

    println!("The number of steps needed to get to the desired spot is {}", path.len() - 1);

    let mut shortest = usize::MAX;
    for node in map.nodes.iter() {
        if node.data.height == 'a' {
            let path = a_star_solve(&map, node, end, compare_hill_height, hill_heuristic);
            if path[0] == end && path.len() - 1 < shortest {
                shortest = path.len() - 1;   
            }
        }
    }

    println!("The shortest path that starts from a spot with height a is {}", shortest);
}