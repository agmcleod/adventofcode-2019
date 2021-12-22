use std::collections::{HashMap, HashSet};
use std::io::Result;

use read_input::read_text;

#[derive(PartialEq)]
enum Tile {
    Path,
    GateLatter(String),
}

type Pos = (i32, i32);

#[derive(Debug, PartialEq)]
enum GateType {
    Interior,
    Exterior,
}

#[derive(Debug)]
struct Path {
    pos: Pos,
    steps: i32,
    gate_type: GateType,
}

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
    gate_positions: &mut HashMap<Pos, String>,
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
                    add_gate(gates, gate.clone(), before_gate_path.to_owned());
                    gate_positions.insert(before_gate_path.to_owned(), gate);
                } else {
                    let tile = map.get(&after_gate_path);
                    if tile.is_some() && tile.unwrap() == &Tile::Path {
                        add_gate(gates, gate.clone(), after_gate_path.to_owned());
                        gate_positions.insert(after_gate_path.to_owned(), gate);
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

fn get_neighbours(map: &HashMap<Pos, Tile>, origin: &Pos, walked_path: &HashSet<Pos>) -> Vec<Pos> {
    let mut neighbours = Vec::new();

    let left_pos = (origin.0 - 1, origin.1);
    let left_tile = map.get(&left_pos);
    if left_tile.is_some() && left_tile.unwrap() == &Tile::Path && !walked_path.contains(&left_pos)
    {
        neighbours.push(left_pos);
    }

    let up_pos = (origin.0, origin.1 - 1);
    let up_tile = map.get(&up_pos);
    if up_tile.is_some() && up_tile.unwrap() == &Tile::Path && !walked_path.contains(&up_pos) {
        neighbours.push(up_pos);
    }

    let right_pos = (origin.0 + 1, origin.1);
    let right_tile = map.get(&right_pos);
    if right_tile.is_some()
        && right_tile.unwrap() == &Tile::Path
        && !walked_path.contains(&right_pos)
    {
        neighbours.push(right_pos);
    }

    let down_pos = (origin.0, origin.1 + 1);
    let down_tile = map.get(&down_pos);
    if down_tile.is_some() && down_tile.unwrap() == &Tile::Path && !walked_path.contains(&down_pos)
    {
        neighbours.push(down_pos);
    }

    neighbours
}

fn recurse_paths(
    map: &HashMap<Pos, Tile>,
    gate_positions: &HashMap<Pos, String>,
    gate_paths: &mut HashMap<Pos, Vec<Path>>,
    origin: &Pos,
    current_pos: &Pos,
    walked_path: &mut HashSet<Pos>,
    count: i32,
    edges: &(i32, i32, i32, i32),
) {
    let neighbours = get_neighbours(&map, current_pos, &walked_path);

    for neighbour in &neighbours {
        walked_path.insert(neighbour.to_owned());
        if gate_positions.contains_key(neighbour) {
            let mut gate_type = GateType::Interior;
            if neighbour.0 == edges.0
                || neighbour.0 == edges.1
                || neighbour.1 == edges.2
                || neighbour.1 == edges.3
            {
                gate_type = GateType::Exterior;
            }
            gate_paths.get_mut(origin).unwrap().push(Path {
                pos: neighbour.to_owned(),
                steps: count + 1,
                gate_type,
            });
        } else {
            recurse_paths(
                map,
                gate_positions,
                gate_paths,
                origin,
                neighbour,
                walked_path,
                count + 1,
                edges,
            );
        }
    }
}

fn run_through_portals(
    gate_paths: &HashMap<Pos, Vec<Path>>,
    gates: &HashMap<String, Vec<Pos>>,
    from: &Pos,
    aa_pos: &Pos,
    zz_pos: &Pos,
    used_positions: HashSet<Pos>,
    steps: i32,
    options: &mut Vec<i32>,
    gate_positions: &HashMap<Pos, String>,
    recursive_maze: bool,
    layer: usize,
) {
    let paths = gate_paths
        .get(from)
        .unwrap()
        .iter()
        .filter(|p| {
            if used_positions.contains(&p.pos)
                && (!recursive_maze || p.gate_type == GateType::Interior)
                || p.pos == *aa_pos
            {
                return false;
            }

            // any exterior paths not AA or ZZ are not reachable yet
            if recursive_maze
                && layer == 1
                && p.gate_type == GateType::Exterior
                && p.pos != *aa_pos
                && p.pos != *zz_pos
            {
                return false;
            }

            return true;
        })
        .collect::<Vec<&Path>>();

    println!(
        "from {:?} {} options: {:?} layer: {}",
        from,
        gate_positions.get(from).unwrap(),
        paths,
        layer
    );

    for path in paths {
        if path.pos == *zz_pos {
            // if we're recursing for p2, and we're deeper than layer 1, skip ZZ
            if recursive_maze && layer > 1 {
                continue;
            }
            options.push(steps + path.steps);
        } else {
            // remove the current portal entrance from the available options
            let mut used_positions = used_positions.clone();
            used_positions.insert(path.pos.clone());
            let gate_string = gate_positions.get(&path.pos).unwrap();

            let next_pos = gates
                .get(gate_string)
                .unwrap()
                .iter()
                .find(|v| **v != path.pos);

            if next_pos.is_none() {
                panic!(
                    "Could not find a connecting gate for {} {:?}",
                    gate_string, path.pos
                );
            }

            let layer = if recursive_maze {
                if path.gate_type == GateType::Exterior {
                    layer - 1
                } else {
                    layer + 1
                }
            } else {
                layer
            };

            run_through_portals(
                gate_paths,
                gates,
                next_pos.unwrap(),
                aa_pos,
                zz_pos,
                used_positions,
                steps + path.steps + 1,
                options,
                gate_positions,
                recursive_maze,
                layer,
            )
        }
    }
}

fn main() -> Result<()> {
    let input = read_text("20/input.txt")?;

    let mut map: HashMap<Pos, Tile> = HashMap::new();
    let mut gates = HashMap::new();
    let mut gate_positions = HashMap::new();

    let mut min_x = std::i32::MAX;
    let mut max_x = 0;
    let mut min_y = std::i32::MAX;
    let mut max_y = 0;

    for (row, line) in input.lines().enumerate() {
        for (col, ch) in line.chars().enumerate() {
            let row = row as i32;
            let col = col as i32;
            if ch != ' ' && ch != '#' {
                let tile = match ch {
                    '.' => Tile::Path,
                    _ => Tile::GateLatter(ch.to_string()),
                };

                map.insert((col, row), tile);
            }
            if ch == '#' {
                min_x = min_x.min(col);
                max_x = max_x.max(col);

                min_y = min_y.min(row);
                max_y = max_y.max(row);
            }
        }
    }

    for (pos, tile) in &map {
        match tile {
            &Tile::GateLatter(ref value) => {
                if !set_path_for_gate(
                    &map,
                    &mut gates,
                    &mut gate_positions,
                    &(pos.0, pos.1 + 1),
                    value,
                    &(pos.0, pos.1 - 1),
                    &(pos.0, pos.1 + 2),
                ) {
                    set_path_for_gate(
                        &map,
                        &mut gates,
                        &mut gate_positions,
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

    let mut gate_paths = HashMap::new();

    for (coord, _) in &gate_positions {
        gate_paths.insert(coord.to_owned(), Vec::new());

        let mut walked_path = HashSet::new();
        walked_path.insert(coord.to_owned());

        recurse_paths(
            &map,
            &gate_positions,
            &mut gate_paths,
            coord,
            coord,
            &mut walked_path,
            0,
            &(min_x, max_x, min_y, max_y),
        );
    }

    // for (coord, path) in &gate_paths {
    //     println!("{:?} {:?}", gate_positions.get(coord).unwrap(), path);
    // }

    let aa = gates.get("AA").unwrap()[0];
    let zz = gates.get("ZZ").unwrap()[0];

    let mut options = Vec::new();
    let mut used_positions = HashSet::new();
    used_positions.insert(aa.clone());
    run_through_portals(
        &gate_paths,
        &gates,
        &aa,
        &aa,
        &zz,
        used_positions,
        0,
        &mut options,
        &gate_positions,
        false,
        1,
    );

    options.sort();
    println!("{:?}", options[0]);

    // p2
    let mut options = Vec::new();
    let mut used_positions = HashSet::new();
    used_positions.insert(aa.clone());
    run_through_portals(
        &gate_paths,
        &gates,
        &aa,
        &aa,
        &zz,
        used_positions,
        0,
        &mut options,
        &gate_positions,
        true,
        1,
    );

    options.sort();
    println!("{:?}", options[0]);

    Ok(())
}
