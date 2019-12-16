#[derive(Debug)]
pub struct ProgramState {
    pub program: Vec<i64>,
    pub index: usize,
    pub inputs: Vec<i64>,
    pub finished: bool,
    pub inputs_index: usize,
    pub relative_base: i64,
}

impl ProgramState {
    pub fn new(base_program: &Vec<i64>, inputs: Vec<i64>) -> Self {
        ProgramState {
            program: base_program.clone(),
            index: 0,
            inputs,
            finished: false,
            inputs_index: 0,
            relative_base: 0,
        }
    }
}

fn get_mode(instructions: &Vec<char>, index: usize) -> String {
    if let Some(val) = instructions.get(index) {
        val.to_string()
    } else {
        String::from("0")
    }
}

fn get_from_pos(numbers: &Vec<i64>, index: usize) -> i64 {
    *numbers.get(index).unwrap_or(&0)
}

fn get_value(
    numbers: &Vec<i64>,
    state: &ProgramState,
    offset: usize,
    instructions: &Vec<char>,
) -> i64 {
    // we add one here, to skip the 2nd digit of the op code
    let mode = get_mode(instructions, offset + 1);
    match mode.as_ref() {
        "0" => {
            return get_from_pos(
                numbers,
                get_from_pos(numbers, state.index + offset) as usize,
            );
        }
        "1" => return get_from_pos(numbers, state.index + offset),
        "2" => {
            return get_from_pos(
                numbers,
                (get_from_pos(numbers, state.index + offset) + state.relative_base) as usize,
            )
        }
        _ => panic!(
            "unrecognized mode in instructions {:?} for index {}",
            instructions,
            offset + 1
        ),
    }
}

// used for opcodes where 0 and 1 always use immediate mode
fn get_insert_value(
    numbers: &Vec<i64>,
    state: &ProgramState,
    offset: usize,
    instructions: &Vec<char>,
) -> i64 {
    // we add one here, to skip the 2nd digit of the op code
    let mode = get_mode(instructions, offset + 1);
    match mode.as_ref() {
        "0" | "1" => return get_from_pos(numbers, state.index + offset),
        "2" => return get_from_pos(numbers, state.index + offset) + state.relative_base,
        _ => panic!(
            "unrecognized mode in instructions {:?} for index {}",
            instructions,
            offset + 1
        ),
    }
}

fn insert_into_program(program: &mut Vec<i64>, position: usize, value: i64) {
    if position >= program.len() {
        let len = position - program.len() + 1;
        let mut append_vec = vec![0; len];
        program.append(&mut append_vec);
    }

    program[position] = value;
}

pub fn run_program<F>(state: &mut ProgramState, limit_input_use: bool, mut handle_output: F)
where
    F: FnMut(&mut ProgramState, i64),
{
    loop {
        let instructions_string = format!("{}", state.program[state.index]);
        let mut instructions: Vec<char> = instructions_string.chars().collect();
        instructions.reverse();

        let op_code = instructions[0].to_digit(10).unwrap();

        match op_code {
            1 => {
                let sum = get_value(&state.program, &state, 1, &instructions)
                    + get_value(&state.program, &state, 2, &instructions);

                let sum_position = get_insert_value(&state.program, &state, 3, &instructions);
                insert_into_program(&mut state.program, sum_position as usize, sum);
                state.index += 4;
            }
            2 => {
                let product = get_value(&state.program, &state, 1, &instructions)
                    * get_value(&state.program, &state, 2, &instructions);
                let product_position = get_insert_value(&state.program, &state, 3, &instructions);
                insert_into_program(&mut state.program, product_position as usize, product);
                state.index += 4;
            }
            3 => {
                let value_pos = get_insert_value(&state.program, &state, 1, &instructions) as usize;
                if state.inputs_index >= state.inputs.len() && limit_input_use {
                    break;
                }
                let input = state
                    .inputs
                    .get(state.inputs_index)
                    .unwrap_or(state.inputs.last().unwrap());

                insert_into_program(&mut state.program, value_pos, *input);

                if state.inputs_index < state.inputs.len() {
                    state.inputs_index += 1;
                }
                state.index += 2;
            }
            4 => {
                let output = get_value(&state.program, &state, 1, &instructions);
                handle_output(state, output);
                state.index += 2;
            }
            5 => {
                if get_value(&state.program, &state, 1, &instructions) != 0 {
                    state.index = get_value(&state.program, &state, 2, &instructions) as usize;
                } else {
                    state.index += 3;
                }
            }
            6 => {
                if get_value(&state.program, &state, 1, &instructions) == 0 {
                    state.index = get_value(&state.program, &state, 2, &instructions) as usize;
                } else {
                    state.index += 3;
                }
            }
            7 => {
                let pos = get_insert_value(&state.program, &state, 3, &instructions) as usize;
                if get_value(&state.program, &state, 1, &instructions)
                    < get_value(&state.program, &state, 2, &instructions)
                {
                    insert_into_program(&mut state.program, pos, 1);
                } else {
                    insert_into_program(&mut state.program, pos, 0);
                }
                state.index += 4;
            }
            8 => {
                let pos = get_insert_value(&state.program, &state, 3, &instructions) as usize;
                if get_value(&state.program, &state, 1, &instructions)
                    == get_value(&state.program, &state, 2, &instructions)
                {
                    insert_into_program(&mut state.program, pos, 1);
                } else {
                    insert_into_program(&mut state.program, pos, 0);
                }
                state.index += 4;
            }
            9 => {
                let second_digit = instructions.get(1).unwrap_or(&'0');
                if *second_digit == '9' {
                    state.finished = true;
                    break;
                } else {
                    state.relative_base += get_value(&state.program, &state, 1, &instructions);
                }
                state.index += 2;
            }
            _ => panic!("Invalid opcode {} at {}", op_code, state.index),
        }
    }
}
