use std::cmp;
use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter, Result};

use crate::Direction::Down;
use intcode::{run_program, ProgramState};
use read_input;
use std::hash::Hash;
use std::time::Duration;

#[derive(Debug)]
enum TileType {
    Empty,
    Wall,
    Oxygen,
    Unknown,
}

impl Display for TileType {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let string_value = match *self {
            TileType::Empty => " ",
            TileType::Wall => "#",
            TileType::Oxygen => "O",
            TileType::Unknown => "?",
        };

        write!(f, "{}", string_value)
    }
}

#[derive(Copy, Clone)]
enum Direction {
    Up = 1,
    Down = 2,
    Left = 3,
    Right = 4,
}

struct Work {
    prev_coord: Option<(i64, i64)>,
    coord: (i64, i64),
    state: ProgramState,
    direction: Direction,
}

impl Work {
    fn new(
        prev_coord: Option<(i64, i64)>,
        coord: (i64, i64),
        state: ProgramState,
        direction: Direction,
    ) -> Self {
        Work {
            prev_coord,
            coord,
            state,
            direction,
        }
    }
}

fn add_direction_to_coord(coord: &mut (i64, i64), direction: Direction) {
    match direction {
        Direction::Up => {
            coord.1 -= 1;
        }
        Direction::Down => {
            coord.1 += 1;
        }
        Direction::Left => {
            coord.0 -= 1;
        }
        Direction::Right => {
            coord.0 += 1;
        }
    }
}

fn get_adjacents(
    coord: &(i64, i64),
    map: &HashMap<(i64, i64), TileType>,
) -> Vec<((i64, i64), Direction)> {
    let mut adjacents = Vec::new();

    if !map.contains_key(&(coord.0, coord.1 - 1)) {
        adjacents.push(((coord.0, coord.1 - 1), Direction::Up));
    }
    if !map.contains_key(&(coord.0, coord.1 + 1)) {
        adjacents.push(((coord.0, coord.1 + 1), Direction::Down));
    }
    if !map.contains_key(&(coord.0 - 1, coord.1)) {
        adjacents.push(((coord.0 - 1, coord.1), Direction::Left));
    }
    if !map.contains_key(&(coord.0 + 1, coord.1)) {
        adjacents.push(((coord.0 + 1, coord.1), Direction::Right));
    }

    adjacents
}

fn populate_coords(coords: &Vec<((i64, i64), Direction)>, map: &mut HashMap<(i64, i64), TileType>) {
    for (coord, _) in coords {
        if !map.contains_key(&coord) {
            map.insert(coord.clone(), TileType::Unknown);
        }
    }
}

fn queue_new_work(
    coord: &(i64, i64),
    adjacents: &Vec<((i64, i64), Direction)>,
    state: &ProgramState,
    work_to_do: &mut Vec<Work>,
) {
    for (next_coord, dir) in adjacents {
        let mut new_state = state.clone();
        new_state.inputs[0] = *dir as i64;
        work_to_do.push(Work::new(
            Some(coord.clone()),
            next_coord.clone(),
            new_state,
            *dir,
        ));
    }
}

fn get_magnitude(coord1: &(i64, i64), coord2: &(i64, i64)) -> (i64, i64) {
    (coord1.0 - coord2.0, coord1.1 - coord2.1)
}

fn main() {
    let text = read_input::read_text("15/input.txt").unwrap();

    let mut map = HashMap::<(i64, i64), TileType>::new();
    let mut paths = HashMap::<(i64, i64), (i64, i64)>::new();

    let base_program: Vec<i64> = text.split(",").map(|n| n.parse().expect("nope")).collect();

    let mut work_to_do: Vec<Work> = vec![
        Work::new(
            None,
            (0, 0),
            ProgramState::new(&base_program, vec![Direction::Up as i64]),
            Direction::Up,
        ),
        Work::new(
            None,
            (0, 0),
            ProgramState::new(&base_program, vec![Direction::Down as i64]),
            Direction::Down,
        ),
        Work::new(
            None,
            (0, 0),
            ProgramState::new(&base_program, vec![Direction::Left as i64]),
            Direction::Left,
        ),
        Work::new(
            None,
            (0, 0),
            ProgramState::new(&base_program, vec![Direction::Right as i64]),
            Direction::Right,
        ),
    ];

    map.insert((0, 0), TileType::Unknown);

    let mut quit = false;
    let mut oxygen_coord = None;
    let mut min_x: i64 = 0;
    let mut min_y: i64 = 0;
    let mut max_x: i64 = 0;
    let mut max_y: i64 = 0;
    while work_to_do.len() > 0 && !quit {
        let mut work = work_to_do.remove(0);
        let coord = work.coord.clone();
        let prev_coord = work.prev_coord.clone();

        run_program(&mut work.state, false, |state, output| {
            if output == 0 {
                map.insert(coord, TileType::Wall);
            } else if output == 1 {
                map.insert(coord, TileType::Empty);
                let adjacents = get_adjacents(&coord, &map);
                populate_coords(&adjacents, &mut map);
                queue_new_work(&coord, &adjacents, &state, &mut work_to_do);
            } else if output == 2 {
                map.insert(coord.clone(), TileType::Oxygen);
                oxygen_coord = Some(coord);
                quit = true;
            }

            if let Some(prev_coord) = prev_coord {
                if paths.contains_key(&coord) {
                    panic!("path already contained child {:?}", coord);
                }
                paths.insert(coord.clone(), prev_coord);
            }

            min_x = cmp::min(min_x, coord.0);
            min_y = cmp::min(min_y, coord.1);
            max_x = cmp::max(max_x, coord.0);
            max_y = cmp::max(max_y, coord.1);

            true
        });
    }

    let mut count = 0;
    let mut current_coord = oxygen_coord.unwrap().clone();
    let mut vector = None;

    let mut path_home = HashSet::new();
    path_home.insert(current_coord.clone());

    loop {
        let next = paths.get(&current_coord).unwrap();
        count += 1;

        path_home.insert(next.clone());
        if next == &(0, 0) {
            count += 1;
            break;
        }

        if vector.is_none() {
            vector = Some(get_magnitude(&current_coord, next))
        } else {
            let next_magnitude = get_magnitude(&current_coord, next);
            if vector.unwrap() != next_magnitude {
                // count += 1;
                vector = Some(next_magnitude);
            }
        }

        current_coord = *next;
    }

    // for y in (min_y..=max_y) {
    //     for x in (min_x..=max_x) {
    //         if path_home.contains(&(x, y)) {
    //             print!("-");
    //         } else {
    //             print!("{}", map.get(&(x, y)).unwrap_or(&TileType::Unknown));
    //         }
    //     }
    //     print!("\n");
    // }

    println!("{}", count);
}
