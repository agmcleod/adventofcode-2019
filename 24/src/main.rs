use std::collections::{HashMap, HashSet};
use std::io::Result;

use read_input::read_text;

fn get_adjacent_bugs(
    map: &Vec<i32>,
    index: usize,
    nested_map: Option<&Vec<i32>>,
    surrounding_map: Option<&Vec<i32>>,
) -> i32 {
    let mut count = 0;
    let include_recursion = nested_map.is_some() && surrounding_map.is_some();

    if include_recursion {
        // 12 is the middle index
        if index == 12 {
            return 0;
        }

        let nested_map = nested_map.unwrap();
        let surrounding_map = surrounding_map.unwrap();

        // above
        if index == 17 {
            count += nested_map[20..25].iter().fold(0, |sum, b| sum + b);
        // first condition checks for rows lower in map, second condition checks recursive up
        } else if (index >= 5 && map[index - 5] == 1) || (index < 5 && surrounding_map[7] == 1) {
            count += 1;
        }

        // to the right
        if index == 11 {
            count +=
                nested_map[0] + nested_map[5] + nested_map[10] + nested_map[15] + nested_map[20];
        // first condition checks to the right if it's not the right edge, second checks recursive right edge
        } else if (index + 1) % 5 != 0 && *map.get(index + 1).unwrap_or(&0) == 1
            || (index + 1 % 5 == 0 && surrounding_map[13] == 1)
        {
            count += 1;
        }

        // below
        if index == 7 {
            count += nested_map[0..5].iter().fold(0, |sum, b| sum + b);
        // check if not in bottom row, second condition get recursive level if bottom row
        } else if index < 20 && *map.get(index + 5).unwrap_or(&0) == 1
            || (index >= 20 && surrounding_map[17] == 1)
        {
            count += 1;
        }

        // to the left
        if index == 13 {
            count +=
                nested_map[4] + nested_map[9] + nested_map[14] + nested_map[19] + nested_map[24];
        // first condition checks if it's not left edge, second checks recursive left edge
        } else if (index) % 5 != 0 && *map.get(index - 1).unwrap_or(&0) == 1
            || (index % 5 == 0 && surrounding_map[11] == 1)
        {
            count += 1;
        }
    } else {
        // above
        if index >= 5 && map[index - 5] == 1 {
            count += 1;
        }

        // to the right
        if (index + 1) % 5 != 0 && *map.get(index + 1).unwrap_or(&0) == 1 {
            count += 1;
        }

        // below
        if *map.get(index + 5).unwrap_or(&0) == 1 {
            count += 1;
        }

        // left
        if index % 5 != 0 && map[index - 1] == 1 {
            count += 1;
        }
    }

    count
}

fn sum_binary_value(map: &Vec<i32>) -> i32 {
    map.iter()
        .enumerate()
        .fold(0, |sum, (i, b)| b * 2i32.pow(i as u32) + sum)
}

fn run_minute(
    map: &Vec<i32>,
    nested_map: Option<&Vec<i32>>,
    surrounding_map: Option<&Vec<i32>>,
) -> Vec<i32> {
    let mut next_state = map.clone();

    for (i, digit) in map.iter().enumerate() {
        let adjacent_bugs = get_adjacent_bugs(&map, i, nested_map, surrounding_map);
        if *digit == 1 && adjacent_bugs != 1 {
            next_state[i] = 0;
        } else if *digit == 0 && (adjacent_bugs == 1 || adjacent_bugs == 2) {
            next_state[i] = 1;
        }
    }

    next_state
}

fn solve_p1(mut map: Vec<i32>) {
    let mut previous_iterations = HashSet::new();
    previous_iterations.insert(sum_binary_value(&map));

    loop {
        map = run_minute(&map, None, None);

        let sum = sum_binary_value(&map);
        if previous_iterations.contains(&sum) {
            println!("{}", sum);
            break;
        }

        previous_iterations.insert(sum_binary_value(&map));
    }
}

fn create_if_does_not_exist(levels: &mut HashMap<i32, Vec<i32>>, next_idx: i32) {
    if !levels.contains_key(&next_idx) {
        levels.insert(next_idx, vec![0; 25]);
    }
}

fn solve_p2(map: Vec<i32>) {
    let mut levels = HashMap::new();
    levels.insert(0, map);

    let mut min_surrounding_level = 0;
    let mut max_nested_level = 0;

    for _ in 0..200 {
        let mut next_state = levels.clone();

        let mut level = 0;
        // created first nested
        create_if_does_not_exist(&mut levels, level + 1);

        // go for surrounding maps
        loop {
            create_if_does_not_exist(&mut levels, level - 1);

            let map = levels.get(&level).unwrap();
            let nested_map = levels.get(&(level + 1)).unwrap();
            let surrounding_map = levels.get(&(level - 1)).unwrap();
            let next_map = run_minute(map, Some(nested_map), Some(surrounding_map));

            if *map == next_map && level <= min_surrounding_level {
                break;
            }

            min_surrounding_level = level.min(min_surrounding_level);

            next_state.insert(level, next_map);

            level -= 1;
        }

        level = 1;
        // go for nested maps
        loop {
            create_if_does_not_exist(&mut levels, level + 1);

            let map = levels.get(&level).unwrap();
            let nested_map = levels.get(&(level + 1)).unwrap();
            let surrounding_map = levels.get(&(level - 1)).unwrap();
            let next_map = run_minute(map, Some(nested_map), Some(surrounding_map));

            if *map == next_map && level >= max_nested_level {
                break;
            }

            max_nested_level = level.max(max_nested_level);

            next_state.insert(level, next_map);

            level += 1;
        }

        levels = next_state;
    }

    let bugs = levels.iter().fold(0, |sum, (_, map)| {
        sum + map.iter().fold(0, |sum, b| sum + *b)
    });

    // for (depth, level) in &levels {
    //     println!("Depth {}", depth);
    //     for (i, b) in level.iter().enumerate() {
    //         let value = if *b == 0 { "." } else { "#" };
    //         print!("{}", value);
    //         if (i + 1) % 5 == 0 {
    //             print!("\n");
    //         }
    //     }
    // }

    println!("{}", bugs);
}

fn main() -> Result<()> {
    let text = read_text("24/input.txt")?;

    let mut map = Vec::new();

    for line in text.lines() {
        for ch in line.chars() {
            if ch == '.' {
                map.push(0);
            } else if ch == '#' {
                map.push(1);
            } else {
                panic!("Unknown character {}", ch);
            }
        }
    }

    solve_p1(map.clone());
    solve_p2(map);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sum_binary_value() {
        assert_eq!(sum_binary_value(&vec![1, 0, 1]), 5);
        assert_eq!(sum_binary_value(&vec![1, 0, 1, 1]), 13);
        assert_eq!(sum_binary_value(&vec![1, 1, 1, 1]), 15);
    }
}
