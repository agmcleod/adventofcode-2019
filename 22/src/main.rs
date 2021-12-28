use std::{io::Result, ops::Mul};

use num::{self, FromPrimitive, ToPrimitive};
use read_input::read_text;

#[derive(Clone, Debug, PartialEq)]
enum Instruction {
    Cut(i64),
    Increment(i64),
}

impl Instruction {
    fn next_position_for_card(&self, card: i64, deck_size: i64) -> i64 {
        match self {
            &Instruction::Cut(n) => mod_floor(card - n, deck_size),
            &Instruction::Increment(n) => mod_floor(card * n, deck_size),
        }
    }

    fn combine(&self, other: &Self, deck_size: i64) -> Vec<Instruction> {
        match self {
            &Instruction::Cut(n) => match other {
                &Instruction::Cut(other) => {
                    vec![Instruction::Cut(mod_floor(n + other, deck_size))]
                }
                &Instruction::Increment(other) => {
                    vec![
                        Instruction::Increment(other),
                        Instruction::Cut(mul_mod(n, other, deck_size)),
                    ]
                }
            },
            &Instruction::Increment(n) => match other {
                &Instruction::Increment(other) => {
                    vec![Instruction::Increment(mul_mod(n, other, deck_size))]
                }
                _ => panic!("cannot combine {:?} {:?}", self, other),
            },
        }
    }

    fn can_be_combined(&self, other: &Self) -> bool {
        !(matches!(self, Self::Increment(_)) && matches!(other, Self::Cut(_)))
    }
}

fn mod_floor(n: i64, m: i64) -> i64 {
    let r = n % m;
    if r < 0 {
        r + m
    } else {
        r
    }
}

fn get_instructions_from_input(text: &String, deck_size: i64) -> Vec<Instruction> {
    let mut instructions = Vec::new();
    for line in text.lines() {
        if line.starts_with("cut") {
            let cut = line.split(" ").nth(1).unwrap();
            let cut: i64 = cut.parse().expect("could not parse cut as number");
            instructions.push(Instruction::Cut(cut));
        } else if line.starts_with("deal into") {
            instructions.push(Instruction::Increment(deck_size - 1));
            instructions.push(Instruction::Cut(1));
        } else if line.starts_with("deal with") {
            let incr = line.split(" ").last().unwrap();
            let incr: i64 = incr.parse().expect("Could not parse increment");
            instructions.push(Instruction::Increment(incr));
        } else {
            panic!("Didnt understand command: {}", line);
        }
    }

    instructions
}

fn mul_mod(a: i64, b: i64, modulous: i64) -> i64 {
    let n = num::BigInt::from_i64(a)
        .unwrap()
        .mul(num::BigInt::from_i64(b).unwrap())
        % num::BigInt::from_i64(modulous).unwrap();

    n.to_i64().unwrap()
    // (a * b) % modulous
}

fn reduce_shuffle_process(deck_size: i64, mut instructions: Vec<Instruction>) -> Vec<Instruction> {
    while instructions.len() > 2 {
        let mut offset = 0;
        while offset < instructions.len() - 1 {
            if instructions[offset].can_be_combined(&instructions[offset + 1]) {
                // combine next technique with this one to make a new one
                let combined = instructions[offset].combine(&instructions[offset + 1], deck_size);
                // remove both items
                instructions.remove(offset);
                instructions.remove(offset);
                for (i, ins) in combined.iter().enumerate() {
                    instructions.insert(offset + i, ins.clone());
                }

                offset = (offset - 1).max(0);
            } else {
                offset += 1;
            }
        }
    }

    instructions
}

// Create a reduced shuffle process repeated the given number of times by doubling the amount
// of iterations until reaching the final count
fn repeat_shuffle_process(
    process: &Vec<Instruction>,
    times: i64,
    deck_size: i64,
) -> Vec<Instruction> {
    // iterate trough the bits in the binary representation of the number of times to repeat
    // from least significant to most significant

    let mut result = Vec::new();
    let mut current = process.clone();

    let mut iterations_left = times;
    while iterations_left > 0 {
        if iterations_left % 2 == 1 {
            // A number is the sum of the value of all the ones in the binary representation
            // Store the process for all bits that are set
            result.append(&mut current.clone());
        }
        current.append(&mut current.clone());
        current = reduce_shuffle_process(deck_size, current);
        iterations_left /= 2;
    }

    reduce_shuffle_process(deck_size, result)
}

fn final_position_for_card(card: i64, deck_size: i64, instructions: &Vec<Instruction>) -> i64 {
    instructions.iter().fold(card, |card, instr| {
        instr.next_position_for_card(card, deck_size)
    })
}

fn main() -> Result<()> {
    let text = read_text("22/input.txt")?;

    let p1_size = 10007;
    let instructions = get_instructions_from_input(&text, p1_size);
    let instructions = reduce_shuffle_process(p1_size, instructions);
    println!("{:?}", instructions);
    println!("{}", final_position_for_card(2019, p1_size, &instructions));

    // p2
    // referencing https://github.com/nibarius/aoc/blob/master/src/main/aoc2019/Day22.kt
    // pulling comments and explanations from there.
    // Also a nice reference on this problem, but im not sure how to bring that into code:
    // https://codeforces.com/blog/entry/72593

    let p2_size: i64 = 119_315_717_514_047;
    let repeats: i64 = 101_741_582_076_661;

    /* Deck becomes sorted again after every deckSize - 1 repeats of the shuffle
        according to Euler's Theorem as mentioned in:
        https://www.reddit.com/r/adventofcode/comments/ee56wh/2019_day_22_part_2_so_whats_the_purpose_of_this/fbs6s6z/

        We're interested in what cards ends up at position 2020 after 'repeats' number of repeats of the shuffle
        process. Calculate the number of times extra that the shuffle process has to be done to get to the
        original state.
    */
    let shuffles_left = p2_size - 1 - repeats;

    // If we run the shuffle process 'shufflesLeftUntilInitialState' times and see at what position
    // the card that was at position 2020 ends up at we have the answer to the problem.

    // So first create a reduced shuffle process of two steps with the desired amount of shuffles
    let instructions = get_instructions_from_input(&text, p2_size);
    let reduced = reduce_shuffle_process(p2_size, instructions);
    let repeated = repeat_shuffle_process(&reduced, shuffles_left, p2_size);

    println!("{}", final_position_for_card(2020, p2_size, &repeated));

    Ok(())
}
