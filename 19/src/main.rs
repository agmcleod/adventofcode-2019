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

    // width of the ray at 50 y
    let x_diff = max_x - min_x;

    // defines the amount of spaces inset from the left so the ship can fit vertically in the ray
    let x_inset_for_ship = (100.0 / slope_1).round() as i64;
    println!("inset {}", x_inset_for_ship);

    // Get first y coordinate of the ray that is at least as wide as the offset + space ship
    // Given slope is consistent, we can take desired width of 100, see how many steps are required based on current width
    // and then multiply it by the current height (50)
    let desired_width = x_inset_for_ship + 100;
    let mut possible_y = (desired_width as f32 / x_diff as f32 * 50.0).round();

    loop {
        possible_y -= 1.0;
        // Slope equation, m = (y2 - y1) / (x2 - x1)
        // with slope value, setting (x1, y1) = (0, 0) we can simplify to get x2 using:
        // x2 = y2 / m
        let prev_x_min = (possible_y / slope_1).round() as i64;
        let prev_x_max = (possible_y / slope_2).round() as i64;
        if prev_x_max - prev_x_min < desired_width {
            possible_y += 1.0;
            break;
        }
    }

    let possible_x_min = (possible_y / slope_1).round() as i64;

    println!(
        "{} at min x {}, y {}",
        (possible_x_min + x_inset_for_ship) * 10_000 + possible_y as i64,
        possible_x_min + x_inset_for_ship,
        possible_y
    );

    Ok(())
}
