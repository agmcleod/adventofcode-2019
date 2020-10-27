use std::cmp;
use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter, Result};

use crate::Direction::Down;
use intcode::{run_program, ProgramState};
use read_input;
use std::hash::Hash;
use std::time::Duration;

#[derive(Debug, PartialEq)]
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
            TileType::Oxygen => ".",
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

impl Direction {
    fn as_coord(&self, from: &(i64, i64)) -> (i64, i64) {
        match *self {
            Direction::Up => (from.0, from.1 - 1),
            Direction::Down => (from.0, from.1 + 1),
            Direction::Left => (from.0 - 1, from.1),
            Direction::Right => (from.0 + 1, from.1),
        }
    }
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

fn get_adjacents(
    coord: &(i64, i64),
    map: &HashMap<(i64, i64), TileType>,
) -> Vec<((i64, i64), Direction)> {
    let mut adjacents = Vec::new();

    let up = Direction::Up.as_coord(coord);
    if !map.contains_key(&up) {
        adjacents.push((up, Direction::Up));
    }
    let down = Direction::Down.as_coord(coord);
    if !map.contains_key(&down) {
        adjacents.push((down, Direction::Down));
    }
    let left = Direction::Left.as_coord(coord);
    if !map.contains_key(&left) {
        adjacents.push((left, Direction::Left));
    }
    let right = Direction::Right.as_coord(coord);
    if !map.contains_key(&right) {
        adjacents.push((right, Direction::Right));
    }

    adjacents
}

fn get_adjacent_empties(
    coord: &(i64, i64),
    map: &HashMap<(i64, i64), TileType>,
) -> Vec<(i64, i64)> {
    let mut adjacents = Vec::new();

    let up = Direction::Up.as_coord(coord);
    if map.get(&up).unwrap_or(&TileType::Unknown) == &TileType::Empty {
        adjacents.push(up);
    }
    let down = Direction::Down.as_coord(coord);
    if map.get(&down).unwrap_or(&TileType::Unknown) == &TileType::Empty {
        adjacents.push(down);
    }
    let left = Direction::Left.as_coord(coord);
    if map.get(&left).unwrap_or(&TileType::Unknown) == &TileType::Empty {
        adjacents.push(left);
    }
    let right = Direction::Right.as_coord(coord);
    if map.get(&right).unwrap_or(&TileType::Unknown) == &TileType::Empty {
        adjacents.push(right);
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

fn _print_map(
    min_x: i64,
    min_y: i64,
    max_x: i64,
    max_y: i64,
    render_path: bool,
    path_home: &HashSet<(i64, i64)>,
    map: &HashMap<(i64, i64), TileType>,
) {
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            if render_path && path_home.contains(&(x, y)) {
                print!("-");
            } else {
                print!("{}", map.get(&(x, y)).unwrap_or(&TileType::Unknown));
            }
        }
        print!("\n");
    }
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

    let mut oxygen_coord = None;
    let mut min_x: i64 = 0;
    let mut min_y: i64 = 0;
    let mut max_x: i64 = 0;
    let mut max_y: i64 = 0;
    while work_to_do.len() > 0 {
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

    let oxygen_coord = oxygen_coord.unwrap();

    let mut count = 0;
    let mut current_coord = oxygen_coord.clone();

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

        current_coord = *next;
    }

    println!("{}", count);

    // print_map(min_x, min_y, max_x, max_y, false, &path_home, &map);

    let mut work = vec![(oxygen_coord, 0)];
    let mut max_depth = 0;

    loop {
        let (coord, level) = work.remove(0);

        max_depth = cmp::max(level, max_depth);

        let adjacents = get_adjacent_empties(&coord, &map);

        for adjacent in &adjacents {
            map.insert(*adjacent, TileType::Oxygen);
            work.push((*adjacent, level + 1));
        }

        if work.is_empty() {
            break;
        }
    }

    // print_map(min_x, min_y, max_x, max_y, false, &path_home, &map);

    println!("{}", max_depth);
}
