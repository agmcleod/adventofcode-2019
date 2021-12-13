use std::io;

use intcode::{run_program, ProgramState};
use read_input::read_text;

fn main() -> io::Result<()> {
    let text = read_text("19/input.txt")?;
    let base_program: Vec<i64> = text.split(",").map(|n| n.parse().expect("nope")).collect();

    let mut covered_spaces = 0;
    let mut coordinate_stepper = 0;
    let mut min_x = std::i64::MAX;
    let mut max_x = 0i64;

    while coordinate_stepper < 2499 {
        let mut program_state = ProgramState::new(
            &base_program,
            vec![coordinate_stepper % 50, coordinate_stepper / 50],
        );
        run_program(&mut program_state, false, |state, value| {
            if value == 1 {
                covered_spaces += 1;

                if state.inputs[1] == 49 {
                    min_x = min_x.min(state.inputs[0]);
                    max_x = max_x.max(state.inputs[0]);
                }
            }

            false
        });

        coordinate_stepper += 1;
    }

    println!("{}. min {},49 max {},49", covered_spaces, min_x, max_x);
    let slope_1: f32 = 49.0 / min_x as f32;
    let slope_2: f32 = 49.0 / max_x as f32;

    let x_diff = max_x - min_x;
    let possible_y = 100 / x_diff * 50;
    // Slope equation, m = (y2 - y1) / (x2 - x1)
    // with slope value, we can simplify to get x2 using:
    // x2 = y2 / m
    let possible_x_min = (possible_y as f32 / slope_1).round() as i64;
    let possible_x_max = (possible_y as f32 / slope_2).round() as i64;

    println!(
        "{} & {} = {}",
        possible_x_min,
        possible_x_max,
        possible_x_max - possible_x_min
    );

    Ok(())
}
