use std::io;

use intcode::{run_program, ProgramState};
use read_input::read_text;

fn main() -> io::Result<()> {
    let text = read_text("19/input.txt")?;
    let base_program: Vec<i64> = text.split(",").map(|n| n.parse().expect("nope")).collect();

    let mut program_state = ProgramState::new(&base_program, vec![0, 0]);

    let mut covered_spaces = 0;
    let mut coordinate_stepper = 1;
    run_program(&mut program_state, false, |state, value| {
        state.inputs[0] = coordinate_stepper % 50;
        state.inputs[1] = coordinate_stepper / 50;
        state.inputs_index = 0;
        println!("{} {:?}", value, state.inputs);
        if value == 1 {
            covered_spaces += 1;
        }

        coordinate_stepper += 1;

        coordinate_stepper >= 2499
    });

    println!("{}", covered_spaces);

    Ok(())
}
