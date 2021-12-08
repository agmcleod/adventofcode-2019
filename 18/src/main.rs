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
    blocked_by: Vec<String>,
    distance: usize,
}

impl PathResult {
    fn new(from: Pos, to: Pos, blocked_by: Vec<String>, distance: usize) -> Self {
        PathResult {
            from,
            to,
            blocked_by,
            distance,
        }
    }
}

struct IterationState {
    collected_keys: HashSet<String>,
    steps: usize,
    pos: Pos,
    key_path: Vec<String>,
}

impl Eq for IterationState {}

impl PartialEq for IterationState {
    fn eq(&self, other: &Self) -> bool {
        self.steps == other.steps
    }
}

impl PartialOrd for IterationState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for IterationState {
    fn cmp(&self, other: &Self) -> Ordering {
        other.steps.cmp(&self.steps)
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

fn get_key<'a, 'b>(room: &'a Room, pos: &'b Pos) -> Option<&'a String> {
    let tile = room.get(pos).unwrap();
    match tile {
        &Tile::Key(ref v) => Some(v),
        _ => None,
    }
}

fn get_paths_key(path_keys: &[String]) -> String {
    path_keys.join("->")
}

fn get_path<'a>(
    room: &Room,
    key_positions: &HashMap<String, Pos>,
    paths: &'a mut HashMap<String, PathResult>,
    player_pos: &Pos,
    path_keys: &Vec<String>,
) -> &'a PathResult {
    let path_key = get_paths_key(path_keys);
    let mut prev_step_distance: usize = 0;
    if paths.contains_key(&path_key) {
        return paths.get(&path_key).unwrap();
    } else if path_keys.len() > 2 {
        // we try one level back, to see if we have that path already, and use it as a starting point
        let prev_step_path_key = get_paths_key(&path_keys[0..path_keys.len() - 1]);
        if paths.contains_key(&prev_step_path_key) {
            prev_step_distance = paths.get(&prev_step_path_key).unwrap().distance;
        }
    }

    let from_pos = if path_keys.len() == 2 {
        player_pos
    } else {
        let from_key = path_keys.get(path_keys.len() - 2).unwrap();
        key_positions.get(from_key).unwrap()
    };

    let to_pos = key_positions.get(path_keys.last().unwrap()).unwrap();

    let mut closed: HashMap<Pos, Pos> = HashMap::new();
    let mut costs: HashMap<Pos, usize> = HashMap::new();
    costs.insert(from_pos.clone(), 0);

    let mut heap = BinaryHeap::new();
    heap.push(PathLocation {
        pos: from_pos.to_owned(),
        cost: 0,
    });

    let mut tracked_positions: Vec<Pos> = Vec::new();

    let mut blocked_by = Vec::new();

    while let Some(path_location) = heap.pop() {
        if path_location.pos == *to_pos {
            let mut pos = closed.get(&path_location.pos).unwrap();
            tracked_positions.push(path_location.pos);

            loop {
                if let Some(p) = closed.get(&pos) {
                    let door_tile = get_door(&room, p);
                    if door_tile.is_some() {
                        blocked_by.push(door_tile.unwrap().to_owned());
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
                    cost: new_cost + distance_to_target(option, to_pos),
                    pos: option.to_owned(),
                });
                closed.insert(option.to_owned(), path_location.pos.clone());
                costs.insert(option.to_owned(), new_cost);
            }
        }
    }

    // println!(
    //     "path from {} {:?} to {} {:?}, result {} total {}",
    //     path_keys.get(path_keys.len() - 2).unwrap(),
    //     from_pos,
    //     path_keys.last().unwrap(),
    //     to_pos,
    //     tracked_positions.len(),
    //     tracked_positions.len() + prev_step_distance
    // );

    paths.insert(
        path_key.clone(),
        PathResult::new(
            from_pos.clone(),
            to_pos.to_owned(),
            blocked_by,
            tracked_positions.len() + prev_step_distance,
        ),
    );

    paths.get(&path_key).unwrap()
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

    let player_start_key = "@".to_string();

    let keys_array = key_positions
        .iter()
        .map(|(key, _pos)| key.to_owned())
        .collect::<Vec<String>>();

    for key in &keys_array {
        get_path(
            &room,
            &key_positions,
            &mut paths,
            &player_pos,
            &vec![player_start_key.clone(), key.to_owned()],
        );
    }

    let mut heap = BinaryHeap::new();

    heap.push(IterationState {
        collected_keys: HashSet::new(),
        pos: player_pos.clone(),
        steps: 0,
        key_path: vec!["@".to_string()],
    });

    while let Some(mut state) = heap.pop() {
        let player_key = "@".to_string();
        let from_key = get_key(&room, &state.pos).unwrap_or(&player_key);

        if *from_key != player_key {
            state.collected_keys.insert(from_key.to_owned());
        }

        let other_keys = keys_array
            .iter()
            .filter(|k| {
                // only path to the key if we have yet to collect it, and it's not the key we're currently pathing from
                !state.collected_keys.contains(*k) && from_key != *k
            })
            .map(|k| {
                let mut key_path = state.key_path.clone();
                key_path.push(k.to_owned());
                key_path
            })
            .filter(|key_path| {
                let path_result =
                    get_path(&room, &key_positions, &mut paths, &player_pos, &key_path);

                // if the path to that key has doors, keep doors where we dont have the key
                // if the length is 0, we've unlocked the doors
                path_result
                    .blocked_by
                    .iter()
                    .filter(|k| !state.collected_keys.contains(&k.to_lowercase()))
                    .collect::<Vec<&String>>()
                    .len()
                    == 0
            })
            .collect::<Vec<Vec<String>>>();

        // println!(
        //     "key: {}, steps: {}, pos: {:?}, collected: {:?}, key path: {:?}, options: {:?}",
        //     from_key,
        //     state.steps,
        //     state.pos,
        //     state.collected_keys,
        //     state.key_path,
        //     other_keys
        //         .iter()
        //         .map(|key_path| { key_path.last().unwrap() })
        //         .collect::<Vec<&String>>()
        // );
        if state.collected_keys.len() == keys_array.len() {
            println!("{}", state.steps);
            break;
        }

        for key_path in &other_keys {
            let path_result = get_path(&room, &key_positions, &mut paths, &player_pos, &key_path);
            heap.push(IterationState {
                collected_keys: state.collected_keys.clone(),
                steps: path_result.distance,
                pos: key_positions
                    .get(key_path.last().unwrap())
                    .unwrap()
                    .to_owned(),
                key_path: key_path.to_owned(),
            })
        }
    }

    Ok(())
}
