use std::cmp;
use std::collections::{HashMap, HashSet};

use read_input::read_text;

mod intcode;
use intcode::ProgramState;

enum StepType {
    Color,
    Turn,
}

#[derive(Copy, Clone)]
enum Direction {
    Down,
    Left,
    Right,
    Up,
}

impl Direction {
    fn from(&mut self, turn: &Turn) {
        *self = match *self {
            Direction::Down => match turn {
                Turn::Left => Direction::Right,
                Turn::Right => Direction::Left,
            },
            Direction::Left => match turn {
                Turn::Left => Direction::Down,
                Turn::Right => Direction::Up,
            },
            Direction::Right => match turn {
                Turn::Left => Direction::Up,
                Turn::Right => Direction::Down,
            },
            Direction::Up => match turn {
                Turn::Left => Direction::Left,
                Turn::Right => Direction::Right,
            },
        };
    }
}

enum Turn {
    Left,
    Right,
}

fn add_direction_to_position(pos: &mut (i32, i32), direction: Direction) {
    match direction {
        Direction::Down => {
            pos.1 += 1;
        }
        Direction::Left => {
            pos.0 -= 1;
        }
        Direction::Up => {
            pos.1 -= 1;
        }
        Direction::Right => {
            pos.0 += 1;
        }
    }
}

fn paint(
    base_program: &Vec<i64>,
    start_color: i64,
    painted_squares: &mut HashSet<(i32, i32)>,
) -> HashMap<(i32, i32), i64> {
    let mut state = ProgramState::new(&base_program, vec![start_color]);
    let mut grid = HashMap::new();
    let mut coord: (i32, i32) = (0, 0);
    grid.insert(coord, 0);

    let mut step_type = StepType::Color;
    let mut direction = Direction::Up;

    intcode::run_program(&mut state, false, |state, output| match step_type {
        StepType::Color => {
            step_type = StepType::Turn;
            grid.insert(coord, output);
            painted_squares.insert(coord);
        }
        StepType::Turn => {
            step_type = StepType::Color;
            if output == 0 {
                direction.from(&Turn::Left);
            } else {
                direction.from(&Turn::Right);
            }

            add_direction_to_position(&mut coord, direction);
            state.inputs.push(*grid.get(&coord).unwrap_or(&0));
        }
    });

    grid
}

fn main() {
    let text = read_text("11/input.txt").unwrap();

    let base_program: Vec<i64> = text.split(",").map(|n| n.parse().expect("nope")).collect();

    let mut painted_squares = HashSet::new();
    paint(&base_program, 0, &mut painted_squares);
    println!("{}", painted_squares.len());

    let grid = paint(&base_program, 1, &mut painted_squares);
    let mut min_x: Option<i32> = None;
    let mut min_y: Option<i32> = None;
    let mut max_x: Option<i32> = None;
    let mut max_y: Option<i32> = None;

    for key in grid.keys() {
        if min_x.is_none() {
            min_x = Some(key.0);
        } else {
            min_x = Some(cmp::min(min_x.unwrap(), key.0));
        }

        if min_y.is_none() {
            min_y = Some(key.1);
        } else {
            min_y = Some(cmp::min(min_y.unwrap(), key.1));
        }

        if max_x.is_none() {
            max_x = Some(key.0);
        } else {
            max_x = Some(cmp::max(max_x.unwrap(), key.0));
        }

        if max_y.is_none() {
            max_y = Some(key.1);
        } else {
            max_y = Some(cmp::max(max_y.unwrap(), key.1));
        }
    }

    let min_x = min_x.unwrap();
    let min_y = min_y.unwrap();
    let max_x = max_x.unwrap();
    let max_y = max_y.unwrap();

    for y in min_y..(max_y + 1) {
        for x in min_x..(max_x + 1) {
            let color = grid.get(&(x, y)).unwrap_or(&0);

            print!(
                "{}",
                match *color {
                    0 => " ",
                    1 => "1",
                    _ => panic!("unrecognized color {}", color),
                },
            );
        }
        print!("\n");
    }
}
