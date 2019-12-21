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

fn main() {
    let text = read_input::read_text("13/input.txt").unwrap();

    let base_program: Vec<i64> = text.split(",").map(|n| n.parse().expect("nope")).collect();

    let mut state = ProgramState::new(&base_program, vec![]);

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
