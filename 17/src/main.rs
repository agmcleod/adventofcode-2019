use std::cmp;
use std::collections::{HashMap, HashSet};
use std::fmt::Display;

use intcode::{run_program, ProgramState};
use read_input::read_text;

#[derive(PartialEq)]
enum TileType {
    Empty,
    Scaffold,
    Robot,
}

impl Display for TileType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string_value = match *self {
            TileType::Empty => ".",
            TileType::Scaffold => "#",
            TileType::Robot => "^",
        };

        write!(f, "{}", string_value)
    }
}

#[derive(Clone, Debug)]
enum FacingDirection {
    Up,
    Left,
    Right,
    Down,
}

impl FacingDirection {
    fn rotate(&mut self, turn: TurnDirection) {
        match *self {
            FacingDirection::Up => {
                if turn == TurnDirection::Left {
                    *self = FacingDirection::Left;
                } else {
                    *self = FacingDirection::Right;
                }
            },
            FacingDirection::Left => {
                if turn == TurnDirection::Left {
                    *self = FacingDirection::Down;
                } else {
                    *self = FacingDirection::Up;
                }
            },
            FacingDirection::Right => {
                if turn == TurnDirection::Left {
                    *self = FacingDirection::Up;
                } else {
                    *self = FacingDirection::Down;
                }
            },
            FacingDirection::Down => {
                if turn == TurnDirection::Left {
                    *self = FacingDirection::Right;
                } else {
                    *self = FacingDirection::Left;
                }
            },
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
enum TurnDirection {
    Left,
    Right,
}

impl TurnDirection {
    fn as_string(&self) -> String {
        match *self {
            TurnDirection::Left => String::from("L"),
            TurnDirection::Right => String::from("R"),
        }
    }
}

type Coord = (i64, i64);

type Tiles = HashMap<Coord, TileType>;

fn print_map(max_x: i64, max_y: i64, tiles: &Tiles) {
    for y in 0..=max_y {
        for x in 0..=max_x {
            let coord = (x, y);
            if tiles.contains_key(&coord) {
                print!("{}", tiles.get(&coord).unwrap());
            }
        }
        print!("\n");
    }
}

fn move_by_direction(tiles: &Tiles, coord: &mut Coord, direction: &FacingDirection) -> bool {
    match *direction {
        FacingDirection::Down => {
            if let Some(tile) = tiles.get(&(coord.0, coord.1 + 1)) {
                if *tile == TileType::Scaffold {
                    coord.1 += 1;
                    return true;
                }
            }
        },
        FacingDirection::Up => {
            if let Some(tile) = tiles.get(&(coord.0, coord.1 - 1)) {
                if *tile == TileType::Scaffold {
                    coord.1 -= 1;
                    return true;
                }
            }
        },
        FacingDirection::Left => {
            if let Some(tile) = tiles.get(&(coord.0 - 1, coord.1)) {
                if *tile == TileType::Scaffold {
                    coord.0 -= 1;
                    return true;
                }
            }
        },
        FacingDirection::Right => {
            if let Some(tile) = tiles.get(&(coord.0 + 1, coord.1)) {
                if *tile == TileType::Scaffold {
                    coord.0 += 1;
                    return true;
                }
            }
        },
    }

    false
}

fn build_commands(tiles: &Tiles, robot_coord: &Coord) -> Vec<String> {
    let mut facing_direction = FacingDirection::Up;
    let mut commands = Vec::new();
    let mut coord = robot_coord.clone();
    let mut move_count = 0;

    // build out commands
    loop {
        let moved = move_by_direction(&tiles, &mut coord, &facing_direction);
        if !moved {
            if move_count > 0 {
                commands.push(move_count.to_string());
            }
            let mut dir = facing_direction.clone();
            let mut turn = TurnDirection::Left;
            dir.rotate(turn.clone());
            let moved = move_by_direction(&tiles, &mut coord, &dir);
            if !moved {
                dir = facing_direction.clone();
                turn = TurnDirection::Right;
                dir.rotate(turn.clone());
                let moved = move_by_direction(&tiles, &mut coord, &dir);
                // no place left to go
                if !moved {
                    break
                }
            }

            move_count = 1;

            facing_direction = dir;
            commands.push(turn.as_string());
        } else {
            move_count += 1;
        }
    }

    commands
}

fn get_next_index(used_command_index: &Vec<usize>, start: &usize, base_size: &usize, total_commands: usize) -> Option<usize> {
    for i in *start+1..total_commands {
        let mut chunk_used = false;
        for chunk_index in i..i+*base_size {
            if used_command_index.contains(&chunk_index) {
                chunk_used = true;
                break
            }
        }

        if !chunk_used {
            return Some(i);
        }
    }

    None
}

fn main() {
    let text = read_text("17/input.txt").unwrap();
    let base_program: Vec<i64> = text.split(",").map(|n| n.parse().expect("nope")).collect();

    let mut program_state = ProgramState::new(&base_program, Vec::new());

    let mut tiles: Tiles = HashMap::new();
    let mut scaffold_spots = HashSet::new();

    let mut coord = (0, 0);
    let mut robot_coord = (0, 0);
    let mut max_x = 0;
    run_program(&mut program_state, false, |_program_state, value| {
        if value == 35 {
            tiles.insert(coord.clone(), TileType::Scaffold);
            scaffold_spots.insert(coord.clone());
            coord.0 += 1;
        } else if value == 46 {
            tiles.insert(coord.clone(), TileType::Empty);
            coord.0 += 1;
        } else if value == 10 {
            coord.1 += 1;
            coord.0 = 0;
        } else if value == 94 {
            tiles.insert(coord.clone(), TileType::Robot);
            robot_coord.0 = coord.0;
            robot_coord.1 = coord.1;
            coord.0 += 1;
        }

        max_x = cmp::max(max_x, coord.0);

        false
    });

    print_map(max_x, coord.1, &tiles);

    let mut sum = 0;

    for coord in &scaffold_spots {
        if scaffold_spots.contains(&(coord.0 - 1, coord.1))
            && scaffold_spots.contains(&(coord.0 + 1, coord.1))
            && scaffold_spots.contains(&(coord.0, coord.1 - 1))
            && scaffold_spots.contains(&(coord.0, coord.1 + 1))
        {
            sum += coord.0 * coord.1;
        }
    }

    println!("p1: {}", sum);

    let commands = build_commands(&tiles, &robot_coord);

    let mut functions: [Vec::<String>; 3] = [Vec::new(), Vec::new(), Vec::new()];
    let mut function_mapping = HashMap::new();
    let mut used_command_index = Vec::new();

    let mut set_functions_count = 0;
    let mut base_size = 2;
    let mut index = 0;
    loop {
        let steps = &commands[index..index + base_size];

        functions[set_functions_count] = steps.iter().cloned().collect::<Vec<String>>();

        function_mapping.insert(set_functions_count, vec![index]);
        used_command_index.append(&mut (index..index + base_size).collect());

        loop {
            let next_index = get_next_index(&used_command_index, &index, &base_size, commands.len());
            if next_index.is_none() {
                break
            }
            index = next_index.unwrap();
            let steps = &commands[index..index + base_size];
            if functions[set_functions_count].as_slice().eq(steps) {
                used_command_index.append(&mut (index..index + base_size).collect());
                function_mapping.get_mut(&set_functions_count).unwrap().push(index);
            }
        }

        set_functions_count += 1;

        if set_functions_count == 3 {
            if  used_command_index.len() == commands.len() {
                break
            }
            set_functions_count = 0;
            function_mapping.clear();
            used_command_index.clear();
            base_size += 1;
        }
    }

    let mut program_state = ProgramState::new(&base_program, Vec::new());
    program_state.program[0] = 2;
}
