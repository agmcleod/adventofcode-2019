use read_input::read_text;

fn try_intcode(
    base_program: &Vec<usize>,
    working_numbers: &mut Vec<usize>,
    noun: usize,
    verb: usize,
) -> usize {
    for (idx, num) in base_program.iter().enumerate() {
        working_numbers[idx] = *num;
    }

    let mut index = 0;

    working_numbers[1] = noun;
    working_numbers[2] = verb;

    loop {
        let op_code = working_numbers[index];
        match op_code {
            1 => {
                let sum = working_numbers[working_numbers[index + 1]]
                    + working_numbers[working_numbers[index + 2]];
                let sum_position = working_numbers[index + 3];
                working_numbers[sum_position] = sum;
            }
            2 => {
                let product = working_numbers[working_numbers[index + 1]]
                    * working_numbers[working_numbers[index + 2]];
                let product_position = working_numbers[index + 3];
                working_numbers[product_position] = product;
            }
            99 => break,
            _ => panic!("Invalid opcode {} at {}", op_code, index),
        }
        index += 4;
    }

    return working_numbers[0];
}

fn main() {
    let text = read_text("2/input.txt").unwrap();

    let base_program: Vec<usize> = text.split(",").map(|n| n.parse().expect("nope")).collect();
    let mut numbers = base_program.clone();

    // part 1
    let pos_zero_value = try_intcode(&base_program, &mut numbers, 12, 2);
    println!("{}", pos_zero_value);

    let mut noun = 0;
    let mut verb = 0;
    loop {
        let pos_zero_value = try_intcode(&base_program, &mut numbers, noun, verb);
        if pos_zero_value == 19690720 {
            println!("{}", 100 * noun + verb);
            break;
        }
        noun += 1;
        if noun > 99 {
            verb += 1;
            noun = 0;
            if verb > 99 {
                panic!("Could not find solution");
            }
        }
    }
}
