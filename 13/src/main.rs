use intcode::{run_program, ProgramState};
use read_input;

#[derive(Copy, Clone, PartialEq)]
enum TileType {
    Empty,
    Wall,
    Block,
    HorizontalPaddle,
    Ball,
}

impl TileType {
    fn from_number(num: usize) -> Self {
        match num {
            0 => TileType::Empty,
            1 => TileType::Wall,
            2 => TileType::Block,
            3 => TileType::HorizontalPaddle,
            4 => TileType::Ball,
            _ => panic!("invalid number {}", num),
        }
    }
}

fn part_one(base_program: &Vec<i64>) {
    let mut state = ProgramState::new(base_program, vec![]);

    let mut output_counter = 0;

    let mut tile = (0, 0, TileType::Empty);
    let mut tiles = Vec::new();

    run_program(&mut state, false, |_, output| {
        output_counter += 1;
        if output_counter == 1 {
            tile.0 = output;
        } else if output_counter == 2 {
            tile.1 = output;
        } else {
            tile.2 = TileType::from_number(output as usize);
            output_counter = 0;
            tiles.push(tile);
        }
    });

    println!(
        "{}",
        tiles
            .iter()
            .filter(|tile| tile.2 == TileType::Block)
            .count()
    );
}

fn part_two(base_program: &Vec<i64>) {
    let mut state = ProgramState::new(base_program, vec![]);
    // play for free
    state.program[0] = 2;

    let mut coords = (0, 0);
    let mut output_counter = 0;
    let mut horizontal_paddle = (0, 0);
    let mut ball = (0, 0);
    let mut score = 0;
    run_program(&mut state, false, |state, output| {
        output_counter += 1;
        if output_counter == 1 {
            coords.0 = output;
        } else if output_counter == 2 {
            coords.1 = output;
        } else {
            if coords == (-1, 0) {
                score = output;
            } else if output == 3 {
                horizontal_paddle = coords;
            } else if output == 4 {
                ball = coords;
            }
            output_counter = 0;
        }

        if horizontal_paddle.0 > ball.0 {
            state.inputs = vec![-1];
        } else if horizontal_paddle.0 < ball.0 {
            state.inputs = vec![1];
        } else {
            state.inputs = vec![0];
        }

        false
    });

    println!("{}", score);
}

fn main() {
    let text = read_input::read_text("13/input.txt").unwrap();

    let base_program: Vec<i64> = text.split(",").map(|n| n.parse().expect("nope")).collect();

    part_one(&base_program);

    part_two(&base_program);
}
