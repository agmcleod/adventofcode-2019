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

#[derive(Eq, PartialEq)]
struct PathLocation {
    pos: Pos,
    cost: usize,
}

impl Ord for PathLocation {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for PathLocation {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

struct PathResult {
    from: Pos,
    to: Pos,
    blocked_by: Option<String>,
    distance: usize,
}

impl PathResult {
    fn new(from: Pos, to: Pos, blocked_by: Option<String>, distance: usize) -> Self {
        PathResult {
            from,
            to,
            blocked_by,
            distance,
        }
    }
}

impl std::fmt::Debug for PathResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // f.debug_struct("PathResult").field("from", &self.from).field("to", &self.to).field("blocked_by", &self.blocked_by).field("distance", &self.distance).finish()
        f.write_fmt(format_args!(
            "from: {:?} to: {:?}, blocked?: {:?}, distance: {}",
            self.from, self.to, self.blocked_by, self.distance
        ))
    }
}

fn add_move_option(options: &mut Vec<Pos>, tile: &Tile, next_pos: (usize, usize)) {
    match tile {
        &Tile::Wall => {}
        _ => {
            options.push(next_pos);
        }
    }
}

fn get_next_steps(pos: &Pos, room: &Room) -> Vec<Pos> {
    let mut options = Vec::new();
    let left = (pos.0 - 1, pos.1);
    if room.contains_key(&left) {
        add_move_option(&mut options, room.get(&left).unwrap(), left);
    }
    let right = (pos.0 + 1, pos.1);
    if room.contains_key(&right) {
        add_move_option(&mut options, room.get(&right).unwrap(), right);
    }
    let up = (pos.0, pos.1 - 1);
    if room.contains_key(&up) {
        add_move_option(&mut options, room.get(&up).unwrap(), up);
    }
    let down = (pos.0, pos.1 + 1);
    if room.contains_key(&down) {
        add_move_option(&mut options, room.get(&down).unwrap(), down);
    }

    options
}

fn distance_to_target(location: &Pos, target: &Pos) -> usize {
    let mut x_diff = location.1 as i16 - target.1 as i16;
    let mut y_diff = location.0 as i16 - target.0 as i16;
    if x_diff < 0 {
        x_diff *= -1;
    }
    if y_diff < 0 {
        y_diff *= -1;
    }

    x_diff as usize + y_diff as usize
}

fn get_door<'a, 'b>(room: &'a Room, pos: &'b Pos) -> Option<&'a String> {
    let tile = room.get(pos).unwrap();
    match tile {
        &Tile::Door(ref v) => Some(v),
        _ => None,
    }
}

fn main() -> Result<()> {
    let input = read_input::read_text("18/input.txt")?;

    let mut room: Room = HashMap::new();

    let mut player_pos = (0, 0);

    let mut key_positions = HashMap::new();

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
                    key_positions.insert(value.clone(), (col, row));
                    room.insert((col, row), Tile::Key(value));
                } else {
                    room.insert((col, row), Tile::Door(value));
                }
            }
        }
    }

    let mut paths: HashMap<String, PathResult> = HashMap::new();

    for (key, key_pos) in &key_positions {
        let mut closed: HashMap<Pos, Pos> = HashMap::new();
        let mut costs: HashMap<Pos, usize> = HashMap::new();
        costs.insert(player_pos.clone(), 0);

        let mut heap = BinaryHeap::new();
        heap.push(PathLocation {
            pos: player_pos,
            cost: 0,
        });

        let mut tracked_positions: Vec<Pos> = Vec::new();

        let mut blocked_by = None;

        while let Some(path_location) = heap.pop() {
            if path_location.pos == *key_pos {
                let mut pos = closed.get(&path_location.pos).unwrap();
                tracked_positions.push(path_location.pos);

                loop {
                    if let Some(p) = closed.get(&pos) {
                        let door_tile = get_door(&room, p);
                        if door_tile.is_some() {
                            blocked_by = door_tile;
                        }
                        tracked_positions.push(*p);
                        pos = p;
                    } else {
                        break;
                    }
                }
                break;
            }

            let options = get_next_steps(&path_location.pos, &room);

            for option in &options {
                let new_cost = costs.get(&path_location.pos).unwrap() + 1;
                if !costs.contains_key(option) || new_cost < *costs.get(option).unwrap() {
                    heap.push(PathLocation {
                        cost: new_cost + distance_to_target(option, key_pos),
                        pos: option.to_owned(),
                    });
                    closed.insert(option.to_owned(), path_location.pos.clone());
                    costs.insert(option.to_owned(), new_cost);
                }
            }
        }

        paths.insert(
            format!("@->{}", key),
            PathResult::new(
                player_pos.clone(),
                key_pos.to_owned(),
                blocked_by.map(|v| v.to_owned()),
                tracked_positions.len(),
            ),
        );
    }

    for (key, res) in &paths {
        println!("{} - {:?}", key, res);
    }

    Ok(())
}
