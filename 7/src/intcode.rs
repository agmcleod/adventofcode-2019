#[derive(Debug)]
pub struct ProgramState {
    pub program: Vec<i32>,
    pub index: usize,
    pub inputs: Vec<i32>,
    pub finished: bool,
}

impl ProgramState {
    pub fn new(base_program: &Vec<i32>, phase_setting: i32) -> Self {
        ProgramState {
            program: base_program.clone(),
            index: 0,
            inputs: vec![phase_setting],
            finished: false,
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

fn get_value(numbers: &Vec<i32>, index: usize, offset: usize, instructions: &Vec<char>) -> i32 {
    // we add one here, to skip the 2nd digit of the op code
    let mode = get_mode(instructions, offset + 1);
    match mode.as_ref() {
        "0" => {
            return numbers[numbers[index + offset] as usize];
        }
        "1" => return numbers[index + offset],
        _ => panic!(
            "unrecognized mode in instructions {:?} for index {}",
            instructions,
            offset + 1
        ),
    }
}

pub fn run_program(state: &mut ProgramState, limit_input_use: bool) -> Option<i32> {
    let mut number_of_accepted_inputs = 0;

    let mut result = None;

    loop {
        let instructions_string = format!("{}", state.program[state.index]);
        let mut instructions: Vec<char> = instructions_string.chars().collect();
        instructions.reverse();

        let op_code = instructions[0].to_digit(10).unwrap();

        match op_code {
            1 => {
                let sum = get_value(&state.program, state.index, 1, &instructions)
                    + get_value(&state.program, state.index, 2, &instructions);

                let sum_position = state.program[state.index + 3];
                state.program[sum_position as usize] = sum;
                state.index += 4;
            }
            2 => {
                let product = get_value(&state.program, state.index, 1, &instructions)
                    * get_value(&state.program, state.index, 2, &instructions);
                let product_position = state.program[state.index + 3];
                state.program[product_position as usize] = product;
                state.index += 4;
            }
            3 => {
                let value_pos = state.program[state.index + 1] as usize;
                if number_of_accepted_inputs >= state.inputs.len() && limit_input_use {
                    break;
                }
                let input = state
                    .inputs
                    .get(number_of_accepted_inputs)
                    .unwrap_or(state.inputs.last().unwrap());
                state.program[value_pos] = *input;

                number_of_accepted_inputs += 1;
                state.index += 2;
            }
            4 => {
                result = Some(state.program[state.program[state.index + 1] as usize]);
                state.index += 2;
            }
            5 => {
                if get_value(&state.program, state.index, 1, &instructions) != 0 {
                    state.index = get_value(&state.program, state.index, 2, &instructions) as usize;
                } else {
                    state.index += 3;
                }
            }
            6 => {
                if get_value(&state.program, state.index, 1, &instructions) == 0 {
                    state.index = get_value(&state.program, state.index, 2, &instructions) as usize;
                } else {
                    state.index += 3;
                }
            }
            7 => {
                let pos = state.program[state.index + 3] as usize;
                if get_value(&state.program, state.index, 1, &instructions)
                    < get_value(&state.program, state.index, 2, &instructions)
                {
                    state.program[pos] = 1;
                } else {
                    state.program[pos] = 0;
                }
                state.index += 4;
            }
            8 => {
                let pos = state.program[state.index + 3] as usize;
                if get_value(&state.program, state.index, 1, &instructions)
                    == get_value(&state.program, state.index, 2, &instructions)
                {
                    state.program[pos] = 1;
                } else {
                    state.program[pos] = 0;
                }
                state.index += 4;
            }
            9 => {
                if instructions[1] == '9' {
                    state.finished = true;
                    break;
                } else {
                    panic!("Invalid opcode {}{}", instructions[1], instructions[0]);
                }
            }
            _ => panic!("Invalid opcode {} at {}", op_code, state.index),
        }
    }

    result
}
