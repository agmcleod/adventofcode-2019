use read_input::read_text;

mod intcode;
use intcode::ProgramState;

fn main() {
    let text = read_text("9/input.txt").unwrap();

    let base_program: Vec<i64> = text.split(",").map(|n| n.parse().expect("nope")).collect();

    let mut state = ProgramState::new(&base_program, vec![1]);
    let result = intcode::run_program(&mut state, false);
    println!("{:?}", result);

    let mut state = ProgramState::new(&base_program, vec![2]);
    let result = intcode::run_program(&mut state, false);
    println!("{:?}", result);
}
