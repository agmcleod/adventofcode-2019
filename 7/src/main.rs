use permutohedron::Heap;
use std::cmp::max;

use read_input::read_text;

mod intcode;

fn main() {
    let text = read_text("7/input.txt").unwrap();

    let base_program: Vec<i32> = text.split(",").map(|n| n.parse().expect("nope")).collect();

    // part one
    let mut phases = [0, 1, 2, 3, 4];
    let heap = Heap::new(&mut phases);

    let mut highest_thrust = 0;
    let mut working_numbers = base_program.clone();
    for phase_sequence in heap {
        let mut result = 0;
        for sequence in &phase_sequence {
            for (idx, num) in base_program.iter().enumerate() {
                working_numbers[idx] = *num;
            }
            result = intcode::run_program(&mut working_numbers, *sequence, result).unwrap();
        }
        highest_thrust = max(highest_thrust, result);
    }

    println!("{:?}", highest_thrust);

    // part two
    let mut highest_thrust = 0;
    let mut phases = [5, 6, 7, 8, 9];
    // let heap = Heap::new(&mut phases);

    // for phase_sequence in heap {
    let mut amplifier_states = [
        base_program.clone(),
        base_program.clone(),
        base_program.clone(),
        base_program.clone(),
        base_program.clone(),
    ];
    let mut index = 0;
    let mut result = 0;
    let phase_sequence = [9, 8, 7, 6, 5];
    loop {
        for (i, sequence) in phase_sequence.iter().enumerate() {
            let input = if index == 0 {
                // if first iteration, start with the sequence number
                *sequence
            } else {
                result
            };
            println!(
                "sequence {} iteration {} result {}",
                sequence, index, result
            );
            // if result is returned update local var
            if let Some(new_result) =
                intcode::run_program(amplifier_states.get_mut(i).unwrap(), input, result)
            {
                result = new_result;
            } else {
                // otherwise assume program E ended
                println!("set output {}", result);
                highest_thrust = max(highest_thrust, result);
                break;
            }
        }
        index += 1;
    }
    // }

    println!("{}", highest_thrust);
}
