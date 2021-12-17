use std::io;

use intcode::{run_program, ProgramState};
use read_input::read_text;

fn check_if_location_in_beam(base_program: &Vec<i64>, x: i64, y: i64) -> bool {
    let mut program_state = ProgramState::new(base_program, vec![x, y]);
    let mut in_beam = false;
    run_program(&mut program_state, false, |_state, value| {
        if value == 1 {
            in_beam = true;
        }

        false
    });

    in_beam
}

fn main() -> io::Result<()> {
    let text = read_text("19/input.txt")?;
    let base_program: Vec<i64> = text.split(",").map(|n| n.parse().expect("nope")).collect();

    let mut covered_spaces = 0;
    let mut coordinate_stepper = 0;

    while coordinate_stepper < 2499 {
        if check_if_location_in_beam(
            &base_program,
            coordinate_stepper % 50,
            coordinate_stepper / 50,
        ) {
            covered_spaces += 1;
        }

        coordinate_stepper += 1;
    }

    println!("{}", covered_spaces);

    let mut y = 99;

    loop {
        let mut x = 0;
        while !check_if_location_in_beam(&base_program, x, y) {
            x += 1;
        }
        if check_if_location_in_beam(&base_program, x + 99, y - 99) {
            println!("{}", x * 10000 + y - 99);
            break;
        }
        y += 1;
    }

    Ok(())
}
