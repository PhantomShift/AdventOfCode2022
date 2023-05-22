pub mod utils {
    pub fn split_lines_group(s: &str, n: usize) -> Vec<String> {
        let lines = s.lines().collect::<Vec<&str>>();
        let mut result = Vec::new();
        for chunk in lines.chunks(n) {
            let to_add = chunk.iter().fold(String::new(), |a, e| a + e + "\n");
            result.push(to_add);
        }
    
        result
    }

    /// Very very rudimentary implementation of getting all
    /// the permutations of a given list of data (currently only implemented for Vectors)
    /// 
    /// # Panics
    /// Panics if factorial of length of permutable is greater than `usize`
    pub trait Permutable<'a, T> {
        type Item;
        fn permutations(&'a self) -> Permutations<Self::Item>;
    }

    pub struct Permutations<'a, C> {
        permutation_number: Option<usize>,
        collection: &'a C
    }

    /// Returns none if `n!` > `usize::MAX`;
    pub fn factorial(n: usize) -> Option<usize> {
        let mut r: usize = 1;
        for i in 1..=n {
            match r.checked_mul(i) {
                Some(result) => r = result,
                None => return None
            }
        }
        Some(r)
    }

    /// Transforms an integer `n` into a [factoradic](https://en.wikipedia.org/wiki/Factorial_number_system)
    /// with given `radix`
    /// 
    /// # Examples
    /// ```
    /// pub use advent_of_code2022::utils::to_factoradic;
    /// assert_eq!(to_factoradic(463, 6), vec![0, 1, 0, 1, 4, 3]);
    /// ```
    pub fn to_factoradic(n: usize, radix: usize) -> Vec<usize> {
        let mut result = Vec::new();
        let mut i = n;
        for divisor in 1..=radix {
            result.push(i % divisor);
            i = i / divisor;
        }
        result.reverse();

        result
    }

    /// Continuing on using the [wikipedia page on factoradics](https://en.wikipedia.org/wiki/Factorial_number_system),
    /// maps a factoradic to a permutation of a list `0..=radix`. 
    pub fn factoradic_as_permutation(factoradic: &Vec<usize>) -> Vec<usize> {
        let mut set = Vec::from_iter(0..=factoradic.len());
        let mut result = Vec::new();
        for &n in factoradic {
            result.push(set.remove(n))
        }

        result
    }
    #[test]
    fn factoradic_as_permutation_test() {
        let decimal = 2982;
        let radix = 7;
        let factoradic = to_factoradic(decimal, radix);
        assert_eq!(factoradic, vec![4, 0, 4, 1, 0, 0, 0]);
        let permutation = factoradic_as_permutation(&factoradic);
        assert_eq!(permutation, vec![4, 0, 6, 2, 1, 3, 5]);
    }
    /// Wrapper around `to_factoradic` and `factoradic_as_permutation`
    fn permutation_indices(n: usize, radix: usize) -> Vec<usize> {
        factoradic_as_permutation(&to_factoradic(n, radix))
    }

    impl<'a, C> Permutations<'a, C> {
        fn new(collection: &'a C) -> Self {
            Permutations { permutation_number: Some(0), collection }
        }
    }
    
    impl<'a, T> Iterator for Permutations<'a, Vec<T>> {
        type Item = Vec<&'a T>;

        fn size_hint(&self) -> (usize, Option<usize>) {
            (0, factorial(self.collection.len()))
        }
        
        fn next(&mut self) -> Option<Self::Item> {
            match self.permutation_number {
                None => None,
                Some(n) if n >= factorial(self.collection.len()).expect("collection should be sufficiently small") => None,
                Some(n) => {
                    let radix = self.collection.len();
                    let indices = permutation_indices(n, radix);
                    let mut references = Vec::new();
                    for index in indices {
                        let v = &self.collection[index];
                        references.push(v);
                    }
                    self.permutation_number = n.checked_add(1);

                    Some(references)
                }
            }
        }
    }

    impl<'a, T> Permutable<'a ,T> for Vec<T> {
        type Item = Vec<T>;
        fn permutations(&'a self) -> Permutations<Self::Item> {
            if factorial(self.len()) == None {
                panic!("Vector too large to get permutations of, self.len() == {}", self.len())
            }
            Permutations::new(self)
        }
    }

    #[test]
    fn permutation_simple_test() {
        let v = vec!['a', 'b', 'c'];
        v.permutations().for_each(|p| println!("{:?}", p));
        let v = vec![34, 42, -9];
        v.permutations().for_each(|p| println!("{:?}", p));
        let v = vec!["AA", "BB", "CC", "DD"];
        v.permutations().for_each(|p| println!("{:?}", p));
    }
}

pub mod sand_stuff {   
    #[derive(PartialEq)]
    pub enum Material {
        Air,
        Rock,
        Sand,
        SandSource
    }

    impl Material {
        pub fn as_char(&self) -> char {
            match self {
                Material::Air => '.',
                Material::Rock => '#',
                Material::Sand => 'o',
                Material::SandSource => '+'
            }
        }
    }

    pub struct DrawInstruction {
        vertices: Vec<(usize, usize)>,
        pub min_x: usize,
        pub max_x: usize,
        pub max_y: usize
    }

    impl DrawInstruction {
        pub fn new(s: &str) -> Self {
            let mut vertices = Vec::new();
            let mut min_x = usize::MAX;
            let mut max_x = 0;
            let mut max_y = 0;
            for pair in s.split(" -> ") {
                let (left, right) = pair.split_once(',').expect("pair should be formatted 'x,y'");
                let x = left.parse::<usize>().expect("left side should contain a number");
                let y = right.parse::<usize>().expect("right side should contain a number");
                if x < min_x { min_x = x }
                if x > max_x { max_x = x }
                if y > max_y { max_y = y }
                
                vertices.push((x, y));
            }

            DrawInstruction { vertices, min_x, max_x, max_y }
        }
    }

    pub struct Canvas {
        pub map: Vec<Vec<Material>>,
        pub source_coord: (usize, usize),
        pub active_sand: Option<(usize, usize)>,
        pub min_x: usize,
        pub min_y: usize,
    }

    impl Canvas {
        pub fn new(instructions: Vec<DrawInstruction>, source_coord: (usize, usize)) -> Self {
            let min_x = instructions.iter().min_by(|a, b| a.min_x.cmp(&b.min_x)).unwrap().min_x;
            let min_y = 0;
            let max_x = instructions.iter().max_by(|a, b| a.max_x.cmp(&b.max_x)).unwrap().max_x + 1;
            let max_y = instructions.iter().max_by(|a, b| a.max_y.cmp(&b.max_y)).unwrap().max_y + 1;

            let height = max_y - min_y;
            let width = max_x - min_x;
            let map = (0..height)
                .map(|_y| {
                    (0..width).map(|_x| {
                        Material::Air
                    }).collect::<Vec<Material>>()
                }).collect::<Vec<Vec<Material>>>();

            let mut canvas = Canvas { map, source_coord, active_sand: None, min_x, min_y };

            for instruction in instructions {
                canvas.draw_rocks(instruction);
            }

            let (source_x, source_y) = canvas.map_coord(source_coord.0, source_coord.1);
            canvas.map[source_y][source_x] = Material::SandSource;

            canvas
        }

        /// Normalizes coordinate to minimal value, i.e. if the top-left corner is at
        /// x = 3, a value of 10 is mapped to 7
        fn map_coord(&self, x: usize, y: usize) -> (usize, usize) {
            (x - self.min_x, y - self.min_y)
        }

        pub fn draw_rocks(&mut self, instruction: DrawInstruction) {
            for slice in instruction.vertices.windows(2) {
                let (this, next) = (slice[0], slice[1]);
                for x in this.0.min(next.0)..this.0.max(next.0) + 1 {
                    for y in this.1.min(next.1)..this.1.max(next.1) + 1 {
                        let (x, y) = self.map_coord(x, y);
                        self.map[y][x] = Material::Rock;
                    }
                }
            }
        }

        /// # Panic
        /// Panics when attempting to add sand while there is currently a piece of sand falling
        pub fn add_sand(&mut self) {
            if self.active_sand.is_some() { panic!("Attempt to add sand when there is currently a piece of sand falling") }

            let (x, y) = self.map_coord(self.source_coord.0, self.source_coord.1);

            self.active_sand = Some((x, y))
        }

        /// Checks tile to bottom right, bottom left and directly below.
        fn attempt_move_down(&self, x: usize, y: usize) -> Option<(usize, usize)> {
            match self.map.get(y + 1) {
                None => { None },
                Some(row_below) => {
                    match row_below.get(x) {
                        None => unreachable!(),
                        Some(Material::Air) => Some((x, y + 1)),
                        // Check diagonals
                        Some(_) => {
                            let down_left = x.checked_sub(1).and_then(|x| row_below.get(x));
                            let down_right = row_below.get(x + 1);
                            match (down_left, down_right) {
                                (Some(material), Some(_) | None) if material == &Material::Air => {
                                    Some((x - 1, y + 1))
                                }
                                (Some(_) | None, Some(material)) if material == &Material::Air => {
                                    Some((x + 1, y + 1))
                                },
                                (Some(_), Some(_)) => Some((x, y)),
                                (Some(_), None) | (None, _) => None
                            }
                        }
                    }
                }
            }
        }

        /// If sand falls into the abyss, returns `None`.
        /// If sand is active or comes to rest, returns position of sand.
        /// If sand comes to rest or falls into the abyss, sets `self.active_sand` to `None`.
        /// If draw is set to true, displays canvas to output after updating
        /// # Panic
        /// Panics if attempting to update when `self.active_sand` is `None`.
        pub fn update(&mut self, draw: bool) -> Option<(usize, usize)> {
            if self.active_sand.is_none() { panic!("Attempt to update canvas when no active sand is present") }

            let (x, y) = self.active_sand.unwrap();
            if self.map[y][x] == Material::Sand { self.map[y][x] = Material::Air; }
            let new_position: Option<(usize, usize)> = self.attempt_move_down(x, y);
            if new_position.is_some() {
                let (new_x, new_y) = new_position.unwrap();
                self.map[new_y][new_x] = Material::Sand;
                if x == new_x && y == new_y {
                    self.active_sand = None;
                } else {
                    self.active_sand = Some((new_x, new_y));
                }
            } else {
                self.active_sand = None;
            }

            if draw {
                self.display();
            }

            new_position
        }

        /// Specifically for use in interactive settings; scans from bottom for any sand particles
        /// that can fall. If there is, sets `self.active_sand` to its position and returns `true`.
        /// Otherwise, returns `false`.
        pub fn reactivate(&mut self) -> bool {
            for (y, row) in self.map.iter().enumerate().rev() {
                for x in 0..row.len() {
                    if row[x] == Material::Sand {
                        let new_pos = self.attempt_move_down(x, y);
                        if new_pos.is_some() && new_pos.unwrap() == (x, y) {
                            continue;
                        }
                        self.active_sand = Some((x, y));
                        return true;
                    }
                }
            }

            false
        }

        pub fn count_material(&self, material: Material) -> usize {
            self.map.iter()
            .map(|row| row.iter().filter(|m| m == &&material).count())
            .sum()
        }

        /// Unfortunately trait Display has a limit for the amount of data it can output,
        /// necessitating that you just print it out manually for the full puzzle output
        /// (especially true for part 2)
        pub fn display(&self) {
            for row in self.map.iter() {
                println!("{}", row.iter().map(Material::as_char).collect::<String>())
            }
        }
    }

    impl std::fmt::Display for Canvas {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let s = self.map.iter()
                .map(|row| {
                    row.iter().map(Material::as_char).collect::<String>() + "\n"
                })
                .collect::<String>();
            write!(f, "{}", s)
        }
    }
}

pub mod point {
    use std::hash::Hash;
    use std::ops::RangeBounds;
    use num::{Integer, Signed, ToPrimitive, range_inclusive};
    use num::{abs, range};

    /// 2-dimensional points object with signed integers
    #[derive(Debug, Hash, Clone, Copy, Eq)]
    pub struct Point<N> where N: Integer + Signed + Copy + Hash + ToPrimitive {
        pub x: N,
        pub y: N
    }
    
    impl<N> Point<N> where N: Integer + Signed + Copy + Hash + ToPrimitive {
        pub fn new(x: N, y: N) -> Self {
            Point { x: x, y: y }
        }

        pub fn zero() -> Self {
            Point { x: N::zero(), y: N::zero() }
        }

        pub fn in_range(&self, corner_a: Self, corner_b: Self) -> bool {
            range(corner_a.x.min(corner_b.x), corner_a.x.max(corner_b.x)).contains(&self.x)
            && range(corner_a.y.min(corner_b.y), corner_a.y.max(corner_b.y)).contains(&self.y)
        }

        pub fn manhattan_distance(&self, other: &Self) -> N {
            abs(self.x - other.x) + abs(self.y - other.y)
        }

        /// # Panic
        /// Panics if `distance` is negative
        pub fn points_within_manhattan_distance(point: Self, distance: N) -> Vec<Self> {
            assert!(distance.is_positive());
            let mut v = Vec::new();
            for y in range_inclusive(point.y - distance, point.y + distance) {
                let d = distance - abs(point.y - y);
                for x in range_inclusive(point.x - d, point.x + d) {
                    v.push(Point { x, y })
                }
            }

            v
        }

        pub fn add_tuple(&self, t: (N, N)) -> Self {
            *self + t.into()
        }
    }

    impl<N> From<(N, N)> for Point<N> where N: Integer + Signed + Copy + Hash + ToPrimitive {
        fn from(value: (N, N)) -> Self {
            Point { x: value.0, y: value.1 }
        }
    }

    impl<N> PartialEq for Point<N> where N: Integer + Signed + Copy + Hash + ToPrimitive {
        fn eq(&self, other: &Self) -> bool {
            self.x == other.x && self.y == other.y
        }
    }

    impl<N> PartialEq<(N, N)> for Point<N> where N: Integer + Signed + Copy + Hash + ToPrimitive {
        fn eq(&self, other: &(N, N)) -> bool {
            self.x == other.0 && self.y == other.1
        }
    }

    impl<N> std::ops::Add for Point<N> where N: Integer + Signed + Copy + Hash + ToPrimitive {
        type Output = Self;

        fn add(self, rhs: Self) -> Self::Output {
            Self { x: self.x + rhs.x, y: self.y + rhs.y }
        }
    }

    #[allow(unused_imports)]
    mod tests {
        use super::*;
        #[test]
        fn test() {
            let point = Point {x: 1, y: 2};
            let other = Point {x: 5, y: 5};
            
            println!("{}", point.manhattan_distance(&other));
        }
    }
}