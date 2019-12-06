use read_input::read_text;

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

fn part_one(base_program: &Vec<i32>, working_numbers: &mut Vec<i32>) {
    for (idx, num) in base_program.iter().enumerate() {
        working_numbers[idx] = *num;
    }

    let mut index = 0;

    loop {
        let instructions_string = format!("{}", working_numbers[index]);
        let mut instructions: Vec<char> = instructions_string.chars().collect();
        instructions.reverse();

        let op_code = instructions[0].to_digit(10).unwrap();

        match op_code {
            1 => {
                let sum = get_value(&working_numbers, index, 1, &instructions)
                    + get_value(&working_numbers, index, 2, &instructions);

                let sum_position = working_numbers[index + 3];
                working_numbers[sum_position as usize] = sum;
                index += 4;
            }
            2 => {
                let product = get_value(&working_numbers, index, 1, &instructions)
                    * get_value(&working_numbers, index, 2, &instructions);
                let product_position = working_numbers[index + 3];
                working_numbers[product_position as usize] = product;
                index += 4;
            }
            3 => {
                // set target to value of 1
                let value_pos = working_numbers[index + 1] as usize;
                working_numbers[value_pos] = 1;
                index += 2;
            }
            4 => {
                println!("{}", working_numbers[working_numbers[index + 1] as usize]);
                index += 2;
            }
            99 => break,
            _ => panic!("Invalid opcode {} at {}", op_code, index),
        }
    }
}

fn part_two(base_program: &Vec<i32>, working_numbers: &mut Vec<i32>) {
    for (idx, num) in base_program.iter().enumerate() {
        working_numbers[idx] = *num;
    }

    let mut index = 0;

    loop {
        let instructions_string = format!("{}", working_numbers[index]);
        let mut instructions: Vec<char> = instructions_string.chars().collect();
        instructions.reverse();

        let op_code = instructions[0].to_digit(10).unwrap();

        match op_code {
            1 => {
                let sum = get_value(&working_numbers, index, 1, &instructions)
                    + get_value(&working_numbers, index, 2, &instructions);

                let sum_position = working_numbers[index + 3];
                working_numbers[sum_position as usize] = sum;
                index += 4;
            }
            2 => {
                let product = get_value(&working_numbers, index, 1, &instructions)
                    * get_value(&working_numbers, index, 2, &instructions);
                let product_position = working_numbers[index + 3];
                working_numbers[product_position as usize] = product;
                index += 4;
            }
            3 => {
                // set target to value of 5
                let value_pos = working_numbers[index + 1] as usize;
                working_numbers[value_pos] = 5;
                index += 2;
            }
            4 => {
                println!("{}", working_numbers[working_numbers[index + 1] as usize]);
                index += 2;
            }
            5 => {
                if get_value(&working_numbers, index, 1, &instructions) != 0 {
                    index = get_value(&working_numbers, index, 2, &instructions) as usize;
                } else {
                    index += 3;
                }
            }
            6 => {
                if get_value(&working_numbers, index, 1, &instructions) == 0 {
                    index = get_value(&working_numbers, index, 2, &instructions) as usize;
                } else {
                    index += 3;
                }
            }
            7 => {
                let pos = working_numbers[index + 3] as usize;
                if get_value(&working_numbers, index, 1, &instructions)
                    < get_value(&working_numbers, index, 2, &instructions)
                {
                    working_numbers[pos] = 1;
                } else {
                    working_numbers[pos] = 0;
                }
                index += 4;
            }
            8 => {
                let pos = working_numbers[index + 3] as usize;
                if get_value(&working_numbers, index, 1, &instructions)
                    == get_value(&working_numbers, index, 2, &instructions)
                {
                    working_numbers[pos] = 1;
                } else {
                    working_numbers[pos] = 0;
                }
                index += 4;
            }
            99 => break,
            _ => panic!("Invalid opcode {} at {}", op_code, index),
        }
    }
}

fn main() {
    let text = read_text("5/input.txt").unwrap();

    let base_program: Vec<i32> = text.split(",").map(|n| n.parse().expect("nope")).collect();
    // let mut numbers = base_program.clone();

    // part_one(&base_program, &mut numbers);

    let mut numbers = base_program.clone();
    part_two(&base_program, &mut numbers);
}
