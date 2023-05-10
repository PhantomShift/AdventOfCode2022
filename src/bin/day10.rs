use std::collections::VecDeque;

enum Instruction {
    Addx(i32),
    Noop
}

impl Instruction {
    fn from_str(s: &str) -> Self {
        if s == "noop" { return Instruction::Noop };
        Instruction::Addx(
            s.split_once(' ')
            .expect("should be formatted as 'addx n'").1
            .parse::<i32>()
            .expect("n should be a valid i32 value")
        )
    }
}

struct Machine {
    command_queue: VecDeque<Instruction>,
    register: i32,
    cycle_counter: usize,
    current_instruction: Option<Instruction>
}

impl Machine {
    fn new(commands: &str) -> Self {
        let mut machine = Machine { command_queue: VecDeque::new(), register: 1, cycle_counter: 0, current_instruction: None };
        for line in commands.lines() {
            let instruction = Instruction::from_str(line);
            if let Instruction::Addx(_) = instruction {
                machine.command_queue.push_back(Instruction::Noop);
            }
            machine.command_queue.push_back(instruction);
        }

        machine
    }

    // Returns strength if cycle = 20, 60, 100 .. 20 + 40n
    fn cycle(&mut self) -> Option<i32> {
        let mut to_return = None;
        if (self.cycle_counter + 20) % 40 == 0 {
            to_return = Some(self.cycle_counter as i32 * self.register);
        }

        if let Some(Instruction::Addx(value)) = self.current_instruction {
            self.register += value;
        }
        self.current_instruction = self.command_queue.pop_front();
        
        self.cycle_counter += 1;
        to_return
    }
}

#[test]
fn day_10_part_1() {
    let test_commands_small = "noop
addx 3
addx -5";
    let expected_states = [1, 1, 1, 4, 4, -1];
    let mut machine = Machine::new(test_commands_small);
    for value in expected_states {
        machine.cycle();
        assert_eq!(value, machine.register);
    }

    let test_commands = std::fs::read_to_string("input/day10test").unwrap();
    let mut expected_values = VecDeque::from([420, 1140, 1800, 2940, 2880, 3960]);
    let expected_sum: i32 = expected_values.iter().sum();
    let mut machine = Machine::new(&test_commands);
    let mut calculated_values = Vec::new();
    while !machine.command_queue.is_empty() {
        if let Some(value) = machine.cycle() {
            assert_eq!(expected_values.pop_front().unwrap(), value);
            calculated_values.push(value);
        }
    }

    assert_eq!(expected_sum, calculated_values.iter().sum());
}

#[test]
fn day_10_part_2() {
    let test_commands = std::fs::read_to_string("input/day10test").unwrap();
    let test_image = std::fs::read_to_string("input/day10testimage").unwrap();

    let mut machine = Machine::new(&test_commands);
    let mut image = String::new();
    for _ in 0..6 {
        for pixel_num in 0..40 {
            machine.cycle();
            if (machine.register - pixel_num).abs() <= 1 {
                image += "#";
            } else {
                image += ".";
            }
        }
        image += "\n";
    }

    assert_eq!(test_image, image);
}

fn main() {
    println!("The sum of the first six signal strengths is {}", {
        let commands = std::fs::read_to_string("input/day10").expect("file should exist");
        let mut machine = Machine::new(&commands);
        let mut strengths = 0;
        while machine.cycle_counter <= 220 {
            if let Some(strength) = machine.cycle() {
                strengths += strength
            }
        }

        strengths
    });

    println!("The CRT displays...");
    print!("{}", {
        let commands = std::fs::read_to_string("input/day10").expect("file should exist");
        let mut machine = Machine::new(&commands);
        let mut image = String::new();
        for _ in 0..6 {
            for pixel_num in 0..40 {
                machine.cycle();
                if (machine.register - pixel_num).abs() <= 1 {
                    image += "#";
                } else {
                    image += ".";
                }
            }
            image += "\n";
        }

        image
    });
}