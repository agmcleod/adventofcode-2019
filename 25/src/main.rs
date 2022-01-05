use std::io::Result;

use intcode;
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

fn main() -> Result<()> {
    let text = read_text("25/input.txt")?;

    let base_program = intcode::get_base_program(&text);

    // let original_instructions = vec![
    //     "north",
    //     "take tambourine",
    //     "east",
    //     "take astrolabe",
    //     "south",
    //     "take shell",
    //     "north",
    //     "east",
    //     "north",
    //     "take klein bottle",
    //     "north",
    //     "take easter egg",
    //     "south",
    //     "south",
    //     "west",
    //     "west",
    //     "north",
    //     "east",
    //     // "take photons", commenting out as this causes death
    //     "west",
    //     "south",
    //     "south",
    //     "west",
    //     "take dark matter",
    //     "west",
    //     // "take giant electromagnet", causes zero movement
    //     "north",
    //     // "take molten lava", death, obviously
    //     "east",
    //     "east",
    //     // "take infinite loop" leads to an infinite loop
    //     "west",
    //     "west",
    //     "west",
    //     "take coin",
    //     "south",
    //     "south",
    //     "north", // heading back to hull breach
    //     "east",
    //     "south",
    //     "east",
    //     "east",
    //     "south", // from hb
    //     "south",
    //     "take hypercube",
    //     "north",
    //     "west",
    // ];

    let instructions = vec![
        "south",
        "south",
        "take hypercube",
        "north",
        "north",
        "north",
        "take tambourine",
        "east",
        // "take astrolabe",
        "south",
        // "take shell",
        "north",
        "east",
        "north",
        // "take klein bottle",
        "north",
        // "take easter egg",
        "south",
        "south",
        "west",
        "west",
        "south",
        "west",
        "take dark matter",
        "west",
        // "take giant electromagnet",
        "north",
        // "take molten lava",
        "west",
        "take coin",
        "south",
        "south",
    ];

    let mut program =
        intcode::ProgramState::new(&base_program, get_instructions_as_input(&instructions));

    intcode::run_program(&mut program, true, |state, value| {
        print!("{}", String::from_utf8(vec![value as u8]).unwrap());
        false
    });

    Ok(())
}
