use std::collections::HashMap;

use intcode::{run_program, ProgramState};
use read_input;

#[derive(Debug)]
enum TileType {
    Wall,
    Oxygen,
}

fn add_direction_to_coord(coord: &mut (i64, i64), direction: i64) {
    if direction == 1 {
        coord.1 -= 1;
    } else if direction == 2 {
        coord.1 += 1;
    } else if direction == 3 {
        coord.0 -= 1;
    } else if direction == 4 {
        coord.0 += 1;
    } else {
        panic!("Invalid direction: {}", direction);
    }
}

fn main() {
    let text = read_input::read_text("15/input.txt").unwrap();

    let mut map = HashMap::<(i64, i64), TileType>::new();

    let base_program: Vec<i64> = text.split(",").map(|n| n.parse().expect("nope")).collect();

    let mut direction = 1;
    let mut state = ProgramState::new(&base_program, vec![direction]);
    let mut coord = (0, 0);

    // this seems to infinite loop, need to apply a BFS approach to only go down a coordinate once
    run_program(&mut state, false, |state, output| {
        if output == 0 {
            map.insert(coord.clone(), TileType::Wall);
            direction += 1;
            if direction > 4 {
                direction = 1;
            }
            state.inputs[0] = direction;
        } else if output == 1 {
            add_direction_to_coord(&mut coord, direction);
        } else if output == 2 {
            add_direction_to_coord(&mut coord, direction);
            map.insert(coord.clone(), TileType::Oxygen);
            return true;
        }

        println!("{} - {:?}", output, coord);

        false
    });

    println!(
        "{:?}", map
    );
}
