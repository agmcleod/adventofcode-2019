use std::collections::{HashMap, HashSet};
use std::io::Result;

use read_input::read_text;

#[derive(PartialEq)]
enum Tile {
    Path,
    GateLatter(String),
}

type Pos = (i32, i32);

fn add_gate(gates: &mut HashMap<String, Vec<Pos>>, gate: String, pos: Pos) {
    if gates.contains_key(&gate) {
        gates.get_mut(&gate).unwrap().push(pos);
    } else {
        gates.insert(gate, vec![pos]);
    }
}

fn set_path_for_gate(
    map: &HashMap<Pos, Tile>,
    gates: &mut HashMap<String, Vec<Pos>>,
    second_letter_pos: &Pos,
    first_letter: &String,
    before_gate_path: &Pos,
    after_gate_path: &Pos,
) -> bool {
    let tile = map.get(&second_letter_pos);
    if tile.is_some() {
        match tile.unwrap() {
            &Tile::GateLatter(ref second_letter) => {
                let gate = format!("{}{}", first_letter, second_letter);

                // get tile above first letter
                let tile = map.get(&before_gate_path);
                if tile.is_some() && tile.unwrap() == &Tile::Path {
                    add_gate(gates, gate, before_gate_path.to_owned());
                } else {
                    let tile = map.get(&after_gate_path);
                    if tile.is_some() && tile.unwrap() == &Tile::Path {
                        add_gate(gates, gate, after_gate_path.to_owned());
                    } else {
                        panic!(
                            "Could not find path beside gate {} tried {:?} & {:?}",
                            gate, before_gate_path, after_gate_path
                        );
                    }
                }

                return true;
            }
            _ => {}
        }
    }

    false
}

fn main() -> Result<()> {
    let input = read_text("20/input.txt")?;

    let mut map: HashMap<(i32, i32), Tile> = HashMap::new();
    let mut gates = HashMap::new();

    for (row, line) in input.lines().enumerate() {
        for (col, ch) in line.chars().enumerate() {
            if ch != ' ' && ch != '#' {
                let tile = match ch {
                    '.' => Tile::Path,
                    _ => Tile::GateLatter(ch.to_string()),
                };

                map.insert((col as i32, row as i32), tile);
            }
        }
    }

    for (pos, tile) in &map {
        match tile {
            &Tile::GateLatter(ref value) => {
                if !set_path_for_gate(
                    &map,
                    &mut gates,
                    &(pos.0, pos.1 + 1),
                    value,
                    &(pos.0, pos.1 - 1),
                    &(pos.0, pos.1 + 2),
                ) {
                    set_path_for_gate(
                        &map,
                        &mut gates,
                        &(pos.0 + 1, pos.1),
                        value,
                        &(pos.0 - 1, pos.1),
                        &(pos.0 + 2, pos.1),
                    );
                }
            }
            _ => {}
        }
    }

    Ok(())
}
