use std::{io, time::{Duration, Instant}};
use advent_of_code2022::sand_stuff::*;
use crossterm::{self, terminal, execute, event::{Event, KeyEvent, KeyCode, KeyEventKind, self, KeyModifiers}, style, cursor, queue};

const DEFAULT_HEIGHT: i32 = 25;
const DEFAULT_WIDTH: i32 = 50;
const DEFAULT_SIMULATION_SPEED: f32 = 30f32;
const MAX_SIMULATION_SPEED: f32 = 120f32;

fn run<W>(w: &mut W, height: i32, width: i32, simulation_speed: f32) -> io::Result<()> where W: io::Write {
    let mut canvas = Canvas::new(vec![DrawInstruction::new(&format!("0,{} -> {},{}", height, width, height))], (width as usize / 2, 0));
    canvas.to_string();
    let mut simulation_speed = simulation_speed;
    
    let mut cursor_position: (i32, i32) = (width / 2, 0);
    let mut modified = true;
    let mut brush_mode = false;
    let mut last_updated = Instant::now();

    execute!(w, terminal::EnterAlternateScreen)?;
    terminal::enable_raw_mode()?;

    loop {
        // Draw phase
        if modified {
            queue!(
                w,
                terminal::Clear(terminal::ClearType::All),
                cursor::MoveTo(0, 0),
            )?;
            for line in canvas.to_string().lines() {
                queue!(w, style::Print(line), cursor::MoveToNextLine(1))?;
            }
            queue!(
                w,
                style::Print("run program with arg --help to view additional options"), cursor::MoveToNextLine(1),
                style::Print("wasd to move cursor, r to toggle rock presence at cursor"), cursor::MoveToNextLine(1),
                style::Print(format!("press b to toggle brush mode, placing rocks when the cursor moves: ({})", if brush_mode {"on"} else {"off"})), cursor::MoveToNextLine(1),
                style::Print("t to clear all sand, y to clear sand and reset rocks to initial state"), cursor::MoveToNextLine(1),
                style::Print("enter/space to spawn sand at cursor"), cursor::MoveToNextLine(1),
                style::Print(format!("arrow keys up/down to change simulation speed: {}", simulation_speed)), cursor::MoveToNextLine(1),
                style::Print(format!("esc/q to quit; sand particles: {}", canvas.count_material(Material::Sand))), cursor::MoveToNextLine(1),
            )?;
            queue!(w, cursor::MoveTo(cursor_position.0 as u16, cursor_position.1 as u16))?;
            
            modified = false;
        }

        w.flush()?;

        // Process input
        if event::poll(Duration::ZERO)? {
            modified = true;

            let input = event::read()?;
            match input {
                // Quitting with ctrl + c
                Event::Key(KeyEvent {
                    code: KeyCode::Char('c'),
                    kind: KeyEventKind::Press,
                    modifiers: KeyModifiers::CONTROL,
                    state: _,
                }) => break,
                Event::Key(KeyEvent {
                    code: char_code,
                    kind: KeyEventKind::Press,
                    modifiers: _,
                    state: _
                }) => match char_code {
                    // Quit
                    KeyCode::Esc | KeyCode::Char('q') => break,
        
                    // Movement
                    KeyCode::Char('w') => cursor_position.1 -= 1,
                    KeyCode::Char('a') => cursor_position.0 -= 1,
                    KeyCode::Char('s') => cursor_position.1 += 1,
                    KeyCode::Char('d') => cursor_position.0 += 1,
        
                    // Interact
                    KeyCode::Char('r') => {
                        if let Some(row) = canvas.map.get((cursor_position.1) as usize) {
                            match row.get(cursor_position.0 as usize) {
                                Some(Material::Rock) => {
                                    canvas.map[(cursor_position.1) as usize][cursor_position.0 as usize] = Material::Air;
                                },
                                Some(Material::Air) | Some(Material::Sand) => {
                                    canvas.map[(cursor_position.1) as usize][cursor_position.0 as usize] = Material::Rock;
                                },
                                _ => ()
                            }
                        }
                    }
                    KeyCode::Char('t') => {
                        for row in canvas.map.iter_mut() {
                            for index in 0..row.len() {
                                if row[index] == Material::Sand {
                                    row[index] = Material::Air;
                                }
                            }
                        }
                    }
                    KeyCode::Char('y') => {
                        for row in canvas.map.iter_mut() {
                            for index in 0..row.len() {
                                row[index] = Material::Air;
                            }
                        }
                        canvas.draw_rocks(DrawInstruction::new(&format!("0,{} -> {},{}", height, width, height)));
                    }
                    KeyCode::Char('b') => {
                        brush_mode = !brush_mode;
                        if brush_mode {
                            queue!(w, cursor::DisableBlinking)?;
                        } else {
                            queue!(w, cursor::EnableBlinking)?;
                        }
                    }
                    KeyCode::Char(' ') | KeyCode::Enter if canvas.active_sand.is_none() => {
                        match canvas.map[(cursor_position.1) as usize][cursor_position.0 as usize] {
                            Material::Air | Material::SandSource => {
                                canvas.source_coord = (cursor_position.0 as usize, (cursor_position.1) as usize);
                                canvas.add_sand()
                            },
                            _ => ()
                        }
                    },
                    KeyCode::Up => simulation_speed = MAX_SIMULATION_SPEED.min(simulation_speed + 1f32),
                    KeyCode::Down => simulation_speed = 1f32.max(simulation_speed - 1f32),
        
                    _ => { modified = false }
                },
                _ => { modified = false }
            }
        }
        
        // Update phase
        if (canvas.active_sand.is_some() | canvas.reactivate()) && last_updated.elapsed().as_secs_f32() > 1f32 / simulation_speed {
            // Simulate
            canvas.update(false);
            modified = true;
            last_updated = Instant::now();
        }

        cursor_position.0 = 0.max(width.min(cursor_position.0));        
        cursor_position.1 = 0.max((height).min(cursor_position.1));

        if brush_mode {
            canvas.map[cursor_position.1 as usize][cursor_position.0 as usize] = Material::Rock;
        }
    }

    execute!(w, terminal::LeaveAlternateScreen)?;
    terminal::disable_raw_mode()
}

fn main() -> io::Result<()> {
    let mut width = DEFAULT_WIDTH;
    let mut height = DEFAULT_HEIGHT;
    let mut simulation_speed = DEFAULT_SIMULATION_SPEED;

    let args = std::env::args().collect::<Vec<String>>();

    if args.iter().find(|s| s.to_lowercase() == "--help").is_some() {
println!("usage: {} [-o|--option [value]]
Interactive toy for simulating sand as defined by day 14 of Advent of Code 2022.
Do note that it will not display/behave properly if the terminal is not large
enough to accompany the canvas at its given size.

Options
 --help                     view this message

Canvas Size
 -w | --width               1 <= n <= 100;\tdefault: {}
 -h | --height              1 <= n <= 100;\tdefault: {}

Simulation Speed - How quickly to update in frames per second
 -s | --simulation-speed    1 <= n <= {};\tdefault: {}
", std::module_path!(), DEFAULT_WIDTH, DEFAULT_HEIGHT, MAX_SIMULATION_SPEED, DEFAULT_SIMULATION_SPEED);
        return Ok(());
    }
    for pair in args.windows(2) {
        match pair[0].to_lowercase().as_str() {
            "-w" | "--width" => {
                match pair[1].parse::<i32>() {
                    Ok(n) if n.clamp(1, 100) == n => {
                        width = n;
                    },
                    _ => {
                        println!("width should be a number from 1 to 100; run with --help for info");
                        return Ok(());
                    }
                }
            },
            "-h" | "--height" => {
                match pair[1].parse::<i32>() {
                    Ok(n) if n.clamp(1, 100) == n => {
                        height = n;
                    },
                    _ => {
                        println!("height should be a number from 1 to 100; run with --help for info");
                        return Ok(());
                    }
                }
            },
            "-s" | "--simulation-speed" => {
                match pair[1].parse::<f32>() {
                    Ok(n) if n.clamp(1f32, MAX_SIMULATION_SPEED) == n => {
                        simulation_speed = n;
                    },
                    _ => {
                        println!("simulation speed should be a number from 1 to {}; run with --help for info", MAX_SIMULATION_SPEED);
                        return Ok(());
                    }
                }
            },
            _ => ()
        }
    }
    let mut stdout = io::stdout();
    run(&mut stdout, height, width, simulation_speed)
}