use std::cmp::{Eq, Ord, Ordering};
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::env;
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
    blocked_by: HashSet<String>,
    distance: usize,
}

impl PathResult {
    fn new(from: Pos, to: Pos, blocked_by: HashSet<String>, distance: usize) -> Self {
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

fn get_paths_key(path_keys: &[String]) -> String {
    if path_keys[0] == "@" {
        path_keys.join("->")
    } else {
        let mut path_keys = path_keys.to_owned();
        path_keys.sort();
        path_keys.join("->")
    }
}

fn write_path<'a>(
    room: &Room,
    key_positions: &HashMap<String, Pos>,
    paths: &mut HashMap<String, PathResult>,
    player_pos: &Pos,
    path_keys: &Vec<String>,
) {
    let path_key = get_paths_key(path_keys);
    let from_key = path_keys.first().unwrap();
    let from_pos = if from_key == "@" {
        player_pos
    } else {
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

    let mut blocked_by = HashSet::new();

    while let Some(path_location) = heap.pop() {
        if path_location.pos == *to_pos {
            let mut pos = &path_location.pos;

            loop {
                if let Some(p) = closed.get(&pos) {
                    let door_tile = get_door(&room, p);
                    if door_tile.is_some() {
                        blocked_by.insert(door_tile.unwrap().to_owned());
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

    if tracked_positions.len() > 0 {
        paths.insert(
            path_key.clone(),
            PathResult::new(
                from_pos.clone(),
                to_pos.to_owned(),
                blocked_by,
                tracked_positions.len(),
            ),
        );
    }
}

fn get_path<'a>(
    paths: &'a HashMap<String, PathResult>,
    current_key: &String,
    next_key: &String,
) -> Option<&'a PathResult> {
    let key = get_paths_key(&vec![current_key.to_owned(), next_key.to_owned()]);
    paths.get(&key)
}

fn get_distance_to_collect_keys(
    room: &Room,
    key_positions: &HashMap<String, Pos>,
    paths: &HashMap<String, PathResult>,
    distance_cache: &mut HashMap<(String, Vec<String>), usize>,
    current_key: &String,
    keys_to_collect: Vec<String>,
) -> usize {
    if keys_to_collect.len() == 0 {
        return 0;
    }

    if distance_cache.contains_key(&(current_key.to_owned(), keys_to_collect.clone())) {
        return *distance_cache
            .get(&(current_key.to_owned(), keys_to_collect))
            .unwrap();
    }

    let reachable_keys: Vec<&String> = keys_to_collect
        .iter()
        .filter(|next_key| {
            let path_to_next_key = get_path(paths, current_key, *next_key);
            if path_to_next_key.is_none() {
                return false;
            }
            // check that the path to the key has no doors that we have yet to collect keys for
            keys_to_collect
                .iter()
                .filter(|k| {
                    path_to_next_key
                        .unwrap()
                        .blocked_by
                        .contains(&k.to_uppercase())
                })
                .count()
                == 0
        })
        .collect();

    if reachable_keys.len() == 0 {
        return 0;
    }

    let mut count = std::usize::MAX;
    for key in &reachable_keys {
        let distance = get_path(paths, current_key, key).unwrap().distance
            + get_distance_to_collect_keys(
                room,
                key_positions,
                paths,
                distance_cache,
                key,
                keys_to_collect
                    .iter()
                    .filter(|k| k != key)
                    .cloned()
                    .collect(),
            );
        count = count.min(distance);
    }

    distance_cache.insert((current_key.to_owned(), keys_to_collect.clone()), count);

    count
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let mut input_path = "18/input.txt";
    let mut is_p2 = false;
    if args.len() >= 2 && args[1] == "p2" {
        input_path = "18/inputp2.txt";
        is_p2 = true;
    }
    let input = read_input::read_text(input_path)?;

    let mut room: Room = HashMap::new();

    let mut player_positions = Vec::new();

    let mut key_positions = HashMap::new();

    for (row, line) in input.lines().enumerate() {
        for (col, ch) in line.chars().enumerate() {
            if ch == '.' {
                room.insert((col, row), Tile::Open);
            } else if ch == '#' {
                room.insert((col, row), Tile::Wall);
            } else if ch == '@' {
                room.insert((col, row), Tile::Open);
                player_positions.push((col, row));
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

    let keys_to_collect = key_positions
        .iter()
        .map(|(key, _pos)| key.to_owned())
        .collect::<Vec<String>>();

    for player_pos in &player_positions {
        for (i, key) in keys_to_collect.iter().enumerate() {
            write_path(
                &room,
                &key_positions,
                &mut paths,
                player_pos,
                &vec![player_start_key.clone(), key.to_owned()],
            );

            for j in (i + 1)..keys_to_collect.len() {
                write_path(
                    &room,
                    &key_positions,
                    &mut paths,
                    player_pos,
                    &vec![key.to_owned(), keys_to_collect[j].clone()],
                );
            }
        }
    }

    if is_p2 {
        let mut groups: HashMap<Pos, Vec<String>> = HashMap::new();
        for (key, path_result) in &paths {
            if !key.contains("@") {
                continue;
            }
            let key_for_group = key.split("->").last().unwrap().to_owned();
            if groups.contains_key(&path_result.from) {
                groups
                    .get_mut(&path_result.from)
                    .unwrap()
                    .push(key_for_group);
            } else {
                groups.insert(path_result.from.clone(), vec![key_for_group]);
            }
        }

        println!("{:?}", groups);

        let mut sum = 0;
        for (_pos, keys_to_collect) in &groups {
            let mut distance_cache: HashMap<(String, Vec<String>), usize> = HashMap::new();
            sum += get_distance_to_collect_keys(
                &room,
                &key_positions,
                &paths,
                &mut distance_cache,
                &"@".to_string(),
                keys_to_collect.to_owned(),
            );
        }
        println!("{}", sum);
    } else {
        let mut distance_cache: HashMap<(String, Vec<String>), usize> = HashMap::new();

        let count = get_distance_to_collect_keys(
            &room,
            &key_positions,
            &paths,
            &mut distance_cache,
            &"@".to_string(),
            keys_to_collect,
        );

        println!("{}", count);
    }

    Ok(())
}
