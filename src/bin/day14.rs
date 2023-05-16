/// Run with argument "display" to show the full output of part 1 and "display2" to show the full output of part 2
use advent_of_code2022::sand_stuff::*;

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