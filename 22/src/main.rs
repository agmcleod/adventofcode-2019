use std::io::Result;

use read_input::read_text;

#[derive(Clone, Debug, PartialEq)]
enum Instruction {
    Cut(i64),
    Increment(i64),
}

impl Instruction {
    fn next_position_for_coord(&self, card: i64, deck_size: i64) -> i64 {
        match self {
            &Instruction::Cut(n) => ((card - n) % deck_size).abs(),
            &Instruction::Increment(n) => ((card * n) % deck_size).abs(),
        }
    }

    fn combine(&self, other: &Self, deck_size: i64) -> Vec<Instruction> {
        match self {
            &Instruction::Cut(n) => match other {
                &Instruction::Cut(other) => {
                    vec![Instruction::Cut(((n + other) % deck_size).abs())]
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
    a * b % modulous
}

fn process_shuffle(deck: &mut Vec<usize>, text: &String) {
    for line in text.lines() {
        if line.starts_with("cut") {
            let cut = line.split(" ").nth(1).unwrap();
            let cut: i32 = cut.parse().expect("could not parse cut as number");
            if cut < 0 {
                deck.rotate_right((cut * -1) as usize);
            } else {
                deck.rotate_left(cut as usize);
            }
        } else if line.starts_with("deal into") {
            deck.reverse();
        } else if line.starts_with("deal with") {
            let incr = line.split(" ").last().unwrap();
            let incr: usize = incr.parse().expect("Could not parse increment");

            let mut table = deck.clone();
            let mut idx = 0;
            while idx < deck.len() {
                table[idx * incr % deck.len()] = deck[idx];
                idx += 1;
            }

            *deck = table;
        } else {
            panic!("Didnt understand command: {}", line);
        }
    }
}

fn reduce_shuffle_process(
    deck_size: i64,
    mut initial_process: Vec<Instruction>,
) -> Vec<Instruction> {
    while initial_process.len() > 2 {
        let mut offset = 0;
        while offset < initial_process.len() - 1 {
            if initial_process[offset].can_be_combined(&initial_process[offset + 1]) {
                // combine next technique with this one to make a new one
                let combined =
                    initial_process[offset].combine(&initial_process[offset + 1], deck_size);
                // remove both items
                initial_process.remove(offset);
                initial_process.remove(offset);
                for (i, ins) in combined.iter().enumerate() {
                    initial_process.insert(offset + i, ins.clone());
                }

                offset = (offset - 1).max(0);
            } else {
                offset += 1;
            }
        }
    }

    initial_process
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

    let binary_times = format!("{:b}", times).chars().rev();
    for bit in binary_times {
        if bit == '1' {}
    }
}

fn main() -> Result<()> {
    let text = read_text("22/input.txt")?;

    const P1_ITERATIONS: usize = 10007;
    // let mut deck: Vec<usize> = Vec::with_capacity(P1_ITERATIONS);

    // for n in 0..P1_ITERATIONS {
    //     deck.push(n);
    // }

    // process_shuffle(&mut deck, &text);

    // println!("{:?}", deck.iter().position(|v| *v == 2019));

    // p2

    // referencing https://github.com/nibarius/aoc/blob/master/src/main/aoc2019/Day22.kt
    // pulling comments and explanations from there.
    // Also a nice reference on this problem, but im not sure how to bring that into code: https://codeforces.com/blog/entry/72593

    const deck_size: usize = 119_315_717_514_047;
    const repeats: usize = 101_741_582_076_661;
    let target_position = 2020;

    /* Deck becomes sorted again after every deckSize - 1 repeats of the shuffle
        according to Euler's Theorem as mentioned in:
        https://www.reddit.com/r/adventofcode/comments/ee56wh/2019_day_22_part_2_so_whats_the_purpose_of_this/fbs6s6z/

        We're interested in what cards ends up at position 2020 after 'repeats' number of repeats of the shuffle
        process. Calculate the number of times extra that the shuffle process has to be done to get to the
        original state.
    */
    let shuffles_left = deck_size - 1 - repeats;

    // If we run the shuffle process 'shufflesLeftUntilInitialState' times and see at what position
    // the card that was at position 2020 ends up at we have the answer to the problem.

    // So first create a reduced shuffle process of two steps with the desired amount of shuffles
    let reduced = reduce_shuffle_process(deck_size, &text);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_increment_functionality() {
        let mut deck = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];

        let text = "deal with increment 3".to_string();
        process_shuffle(&mut deck, &text);

        assert_eq!(deck, vec![0, 7, 4, 1, 8, 5, 2, 9, 6, 3]);
    }
}
