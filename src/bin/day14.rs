/// Run with argument "display" to show the full output of part 1 and "display2" to show the full output of part 2

#[derive(PartialEq)]
enum Material {
    Air,
    Rock,
    Sand,
    SandSource
}

impl Material {
    fn as_char(&self) -> char {
        match self {
            Material::Air => '.',
            Material::Rock => '#',
            Material::Sand => 'o',
            Material::SandSource => '+'
        }
    }
}

struct DrawInstruction {
    vertices: Vec<(usize, usize)>,
    min_x: usize,
    max_x: usize,
    max_y: usize
}

impl DrawInstruction {
    fn new(s: &str) -> Self {
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

struct Canvas {
    map: Vec<Vec<Material>>,
    source_coord: (usize, usize),
    active_sand: Option<(usize, usize)>,
    min_x: usize,
    min_y: usize,
}

impl Canvas {
    fn new(instructions: Vec<DrawInstruction>, source_coord: (usize, usize)) -> Self {
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

    fn draw_rocks(&mut self, instruction: DrawInstruction) {
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
    fn add_sand(&mut self) {
        if self.active_sand.is_some() { panic!("Attempt to add sand when there is currently a piece of sand falling") }

        let (x, y) = self.map_coord(self.source_coord.0, self.source_coord.1);

        self.active_sand = Some((x, y))
    }

    /// If sand falls into the abyss, returns `None`.
    /// If sand is active or comes to rest, returns position of sand.
    /// If sand comes to rest or falls into the abyss, sets `self.active_sand` to `None`.
    /// If draw is set to true, displays canvas to output after updating
    /// # Panic
    /// Panics if attempting to update when `self.active_sand` is `None`.
    fn update(&mut self, draw: bool) -> Option<(usize, usize)> {
        if self.active_sand.is_none() { panic!("Attempt to update canvas when no active sand is present") }

        let (x, y) = self.active_sand.unwrap();
        if self.map[y][x] == Material::Sand { self.map[y][x] = Material::Air; }
        let new_position: Option<(usize, usize)> = match self.map.get(y + 1) {
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
        };
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

    fn count_material(&self, material: Material) -> usize {
        self.map.iter()
        .map(|row| row.iter().filter(|m| m == &&material).count())
        .sum()
    }

    /// Unfortunately trait Display has a limit for the amount of data it can output,
    /// necessitating that you just print it out manually for the full puzzle output
    /// (especially true for part 2)
    fn display(&self) {
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

#[test]
fn day_14_part_1() {
    let input = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";
    let instructions = input.lines().map(|s| DrawInstruction::new(s)).collect::<Vec<DrawInstruction>>();
    let mut canvas = Canvas::new(instructions, (500, 0));
    println!("{}", canvas);

    let mut should_stop = false;
    while !should_stop {
        canvas.add_sand();
        while canvas.active_sand.is_some() {
            // std::thread::sleep(std::time::Duration::from_secs_f32(0.25));
            should_stop = canvas.update(false).is_none();
        }
    }
    println!("{}", canvas);
    assert_eq!(24, canvas.count_material(Material::Sand));
}

// Though not quite as bad as day 12, part 2 of day 14 still takes my machine
// about 23-25 seconds on debug build using the real input,
// which is likely a product of the fact that I am actually
// simulating every individual step instead of using some
// big-brain solution that smart people would come up with 
#[test]
fn day_14_part_2() {
    let input = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";
    let mut instructions = input.lines().map(|s| DrawInstruction::new(s)).collect::<Vec<DrawInstruction>>();
    let floor_height = instructions.iter().max_by(|a, b| a.max_y.cmp(&b.max_y)).unwrap().max_y + 2;
    let (floor_left, floor_right) = (500 - floor_height, 500 + floor_height);
    instructions.push(DrawInstruction::new(&format!("{},{} -> {},{}", floor_left, floor_height, floor_right, floor_height)));
    let mut canvas = Canvas::new(instructions, (500, 0));
    canvas.display();

    while canvas.count_material(Material::SandSource) > 0 {
        canvas.add_sand();
        while canvas.active_sand.is_some() {
            canvas.update(false);

        }
    }

    assert_eq!(93, canvas.count_material(Material::Sand));
}

fn main() {
    let display = std::env::args().any(|s| s == "display");
    let input = std::fs::read_to_string("input/day14").expect("file should exist");
    let instructions = input.lines().map(|s| DrawInstruction::new(s)).collect::<Vec<DrawInstruction>>();
    let mut canvas = Canvas::new(instructions, (500, 0));
    'simulation: loop {
        canvas.add_sand();
        while canvas.active_sand.is_some() {
            if canvas.update(false).is_none() {
                break 'simulation;
            }
        }
    }
    if display { canvas.display(); }

    println!("The number of sand particles at rest before they start falling into the abyss is {}", canvas.count_material(Material::Sand));
    
    let display = std::env::args().any(|s| s == "display2");
    let mut instructions = input.lines().map(|s| DrawInstruction::new(s)).collect::<Vec<DrawInstruction>>();
    let floor_height = instructions.iter().max_by(|a, b| a.max_y.cmp(&b.max_y)).unwrap().max_y + 2;
    let (floor_left, floor_right) = (500 - floor_height, 500 + floor_height);
    instructions.push(DrawInstruction::new(&format!("{},{} -> {},{}", floor_left, floor_height, floor_right, floor_height)));
    let mut canvas = Canvas::new(instructions, (500, 0));
    while canvas.count_material(Material::SandSource) > 0 {
        canvas.add_sand();
        while canvas.active_sand.is_some() {
            // Uncomment these two lines and comment third to see each step individually in all their simulated glory
            // std::thread::sleep(Duration::from_secs_f32(1f32 / 170f32));
            // canvas.update(true);
            canvas.update(false);
        }
    }
    if display { canvas.display(); }

    println!("The number of sand particles at rest when the source is plugged is {}", canvas.count_material(Material::Sand));
}