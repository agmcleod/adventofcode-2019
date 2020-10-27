use std::cmp;
use std::collections::{HashMap, HashSet};
use std::fmt::Display;

use intcode::{run_program, ProgramState};
use read_input::read_text;

enum TileType {
    Empty,
    Scaffold,
}

impl Display for TileType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string_value = match *self {
            TileType::Empty => ".",
            TileType::Scaffold => "#",
        };

        write!(f, "{}", string_value)
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

fn main() {
    let text = read_text("17/input.txt").unwrap();
    let base_program: Vec<i64> = text.split(",").map(|n| n.parse().expect("nope")).collect();

    let mut program_state = ProgramState::new(&base_program, Vec::new());

    let mut tiles: Tiles = HashMap::new();
    let mut scaffold_spots = HashSet::new();

    let mut coord = (0, 0);
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

    let mut program_state = ProgramState::new(&base_program, Vec::new());
    program_state.program[0] = 2;
}
