use std::io::Result;

use intcode::{get_base_program, run_program, ProgramState};
use read_input::read_text;

fn get_instructions_as_input(instructions: &Vec<&str>) -> Vec<i64> {
    let mut input: Vec<i64> = instructions
        .join("\n")
        .as_bytes()
        .iter()
        .map(|u| *u as i64)
        .collect();

    // add a trailing new line
    input.push(10);
    input
}

fn try_program(instructions: &Vec<&str>, base_program: &Vec<i64>) {
    let input = get_instructions_as_input(instructions);

    let mut program_state = ProgramState::new(base_program, input);

    run_program(&mut program_state, false, |_state, value| {
        if value > 127 {
            print!("{}", value);
        } else {
            print!("{}", String::from_utf8(vec![value as u8]).unwrap());
        }

        false
    });
}

fn main() -> Result<()> {
    let text = read_text("21/input.txt")?;
    let base_program = get_base_program(&text);

    // p1
    let mut instructions = Vec::with_capacity(15);
    instructions.push("NOT B J"); // set jump true if B is a hole
    instructions.push("NOT C T"); // set register to true if C is a hole
    instructions.push("OR T J"); // keep jumping if it's a hole
    instructions.push("AND D J"); // keep jumping if D is safe
    instructions.push("NOT A T"); // if next space is a hole, set register
    instructions.push("OR T J"); // jump if jumping or A is a hole
    instructions.push("WALK");

    try_program(&instructions, &base_program);

    // p2
    let mut instructions = Vec::with_capacity(15);
    instructions.push("NOT B J"); // set jump true if B is a hole
    instructions.push("NOT C T"); // set register to true if C is a hole
    instructions.push("OR T J"); // keep jumping if it's a hole
    instructions.push("AND D J"); // keep jumping if D is safe
    instructions.push("NOT A T"); // if next space is a hole, set register
    instructions.push("OR T J"); // jump if jumping or A is a hole
    instructions.push("RUN");

    try_program(&instructions, &base_program);

    Ok(())
}
