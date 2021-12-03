use std::cmp::{Eq, Ord, Ordering};
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::io::Result;

use read_input;

#[derive(Clone, PartialEq)]
enum Tile {
    Wall,
    Open,
    Key(String),
    Door(String),
}

type Pos = (usize, usize);
type Room = HashMap<Pos, Tile>;

#[derive(Clone)]
struct Work {
    move_count: usize,
    used_tiles: HashSet<Pos>,
    pos: Pos,
    room: Room,
    keys: Vec<String>,
}

impl Eq for Work {}

impl PartialEq for Work {
    fn eq(&self, other: &Self) -> bool {
        self.move_count == other.move_count
    }
}

impl PartialOrd for Work {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

impl Ord for Work {
    fn cmp(&self, other: &Self) -> Ordering {
        // reverse sort so the heap returns the lowest count
        other.move_count.cmp(&self.move_count)
    }
}

fn add_move_option(
    options: &mut Vec<Pos>,
    tile: &Tile,
    next_pos: (usize, usize),
    keys: &Vec<String>,
) {
    match tile {
        &Tile::Open => {
            options.push(next_pos);
        }
        Tile::Door(value) => {
            if keys.contains(&value.to_lowercase()) {
                options.push(next_pos);
            }
        }
        Tile::Key(_value) => {
            options.push(next_pos);
        }
        _ => {}
    }
}

fn get_next_steps(
    used_tiles: &HashSet<Pos>,
    pos: &Pos,
    room: &Room,
    keys: &Vec<String>,
) -> Vec<Pos> {
    let mut options = Vec::new();
    let left = (pos.0 - 1, pos.1);
    if room.contains_key(&left) && !used_tiles.contains(&left) {
        add_move_option(&mut options, room.get(&left).unwrap(), left, keys);
    }
    let right = (pos.0 + 1, pos.1);
    if room.contains_key(&right) && !used_tiles.contains(&right) {
        add_move_option(&mut options, room.get(&right).unwrap(), right, keys);
    }
    let up = (pos.0, pos.1 - 1);
    if room.contains_key(&up) && !used_tiles.contains(&up) {
        add_move_option(&mut options, room.get(&up).unwrap(), up, keys);
    }
    let down = (pos.0, pos.1 + 1);
    if room.contains_key(&down) && !used_tiles.contains(&down) {
        add_move_option(&mut options, room.get(&down).unwrap(), down, keys);
    }

    options
}

fn main() -> Result<()> {
    let input = read_input::read_text("18/input.txt")?;

    let mut room: Room = HashMap::new();

    let mut player_pos = (0, 0);

    for (row, line) in input.lines().enumerate() {
        for (col, ch) in line.chars().enumerate() {
            if ch == '.' {
                room.insert((col, row), Tile::Open);
            } else if ch == '#' {
                room.insert((col, row), Tile::Wall);
            } else if ch == '@' {
                room.insert((col, row), Tile::Open);
                player_pos = (col, row);
            } else {
                let value = ch.to_string();
                if value == value.to_lowercase() {
                    room.insert((col, row), Tile::Key(value));
                } else {
                    room.insert((col, row), Tile::Door(value));
                }
            }
        }
    }

    let mut work_items = BinaryHeap::new();
    work_items.push(Work {
        used_tiles: HashSet::new(),
        move_count: 0,
        pos: player_pos,
        room,
        keys: Vec::new(),
    });

    loop {
        let work = work_items.pop();
        if work.is_none() {
            break;
        }

        let mut work = work.unwrap();

        let tile = work.room.get(&work.pos).unwrap();
        match tile {
            Tile::Door(value) => {
                if !work.keys.contains(&value.to_lowercase()) {
                    panic!(
                        "Landed on door {} but does not have a key on ring: {:?}",
                        value, work.keys
                    );
                }

                work.room.insert(work.pos, Tile::Open);
                work.used_tiles.clear();
            }
            Tile::Key(value) => {
                work.keys.push(value.clone());
                work.room.insert(work.pos, Tile::Open);
                work.used_tiles.clear();

                let mut has_keys = false;
                for (_key, tile) in &work.room {
                    match tile {
                        Tile::Key(_v) => {
                            has_keys = true;
                            break;
                        }
                        _ => {}
                    }
                }

                if !has_keys {
                    println!("{}, keys: {:?}", work.move_count, work.keys);
                    break;
                }
            }
            _ => {}
        }

        work.used_tiles.insert(work.pos.clone());

        let options = get_next_steps(&work.used_tiles, &work.pos, &work.room, &work.keys);
        for opt in &options {
            work_items.push(Work {
                used_tiles: work.used_tiles.clone(),
                move_count: work.move_count + 1,
                pos: opt.to_owned(),
                room: work.room.clone(),
                keys: work.keys.clone(),
            })
        }

        // println!(
        //     "Added work: {:?} from {:?} for new count: {}",
        //     options,
        //     work.pos,
        //     work.move_count + 1
        // );
    }

    Ok(())
}
