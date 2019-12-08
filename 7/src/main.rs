use permutohedron::Heap;
use std::cmp::max;

use read_input::read_text;

mod intcode;

use intcode::ProgramState;

fn main() {
    let text = read_text("7/input.txt").unwrap();

    let base_program: Vec<i32> = text.split(",").map(|n| n.parse().expect("nope")).collect();

    // part one
    let mut phases = [0, 1, 2, 3, 4];
    let heap = Heap::new(&mut phases);

    let mut highest_thrust = 0;

    let mut state = ProgramState {
        program: base_program.clone(),
        index: 0,
        inputs: vec![0, 0],
        finished: false,
        inputs_index: 0,
    };

    for phase_sequence in heap {
        let mut result = 0;
        for sequence in &phase_sequence {
            for (idx, num) in base_program.iter().enumerate() {
                state.program[idx] = *num;
            }

            state.inputs = vec![*sequence, result];
            state.index = 0;
            state.inputs_index = 0;
            state.finished = false;

            result = intcode::run_program(&mut state, false).unwrap();
        }
        highest_thrust = max(highest_thrust, result);
    }

    println!("{:?}", highest_thrust);

    // part two
    let mut highest_thrust = 0;
    let mut phases = [5, 6, 7, 8, 9];
    let heap = Heap::new(&mut phases);

    for phase_sequence in heap {
        let mut amplifier_states = [
            ProgramState::new(&base_program, phase_sequence[0]),
            ProgramState::new(&base_program, phase_sequence[1]),
            ProgramState::new(&base_program, phase_sequence[2]),
            ProgramState::new(&base_program, phase_sequence[3]),
            ProgramState::new(&base_program, phase_sequence[4]),
        ];
        // add a zero to the first one
        amplifier_states.get_mut(0).unwrap().inputs.push(0);
        let amplifiers_count = amplifier_states.len();
        'main: loop {
            for i in 0..amplifier_states.len() {
                {
                    let state = amplifier_states.get(i).unwrap();
                    if state.finished {
                        continue;
                    }
                }
                let result = {
                    let mut state = amplifier_states.get_mut(i).unwrap();
                    intcode::run_program(&mut state, true)
                };

                if let Some(output) = result {
                    // if result is returned next inputs
                    {
                        let next_state = amplifier_states
                            .get_mut((i + 1) % amplifiers_count)
                            .unwrap();
                        next_state.inputs.push(output);
                    }

                    let state = amplifier_states.get(i).unwrap();
                    if i == amplifiers_count - 1 && state.finished {
                        highest_thrust = max(highest_thrust, output);
                        break 'main;
                    }
                }
            }
        }
    }

    println!("{}", highest_thrust);
}
