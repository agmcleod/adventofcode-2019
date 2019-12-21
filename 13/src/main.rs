use intcode::{run_program, ProgramState};
use read_input;

fn main() {
    let text = read_input::read_text("13/input.txt").unwrap();

    let base_program: Vec<i64> = text.split(",").map(|n| n.parse().expect("nope")).collect();

    let mut state = ProgramState::new(&base_program, vec![]);
}
