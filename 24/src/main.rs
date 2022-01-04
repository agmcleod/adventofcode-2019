use std::collections::HashSet;
use std::io::Result;

use read_input::read_text;

fn get_adjacent_bugs(map: &Vec<i32>, index: usize, include_recursion: bool) -> i32 {
    let mut count = 0;

    if include_recursion {
        // 12 is the middle index
        if index == 12 {
            return 0;
        }

        // above
        if index == 17 {
            count += map[20..25].iter().fold(0, |sum, b| sum + b);
        // first condition checks for rows lower in map, second condition checks recursive up
        } else if (index >= 5 && map[index - 5] == 1) || (index < 5 && map[7] == 1) {
            count += 1;
        }

        // to the right
        if index == 11 {
            count += map[0] + map[5] + map[10] + map[15] + map[20];
        // first condition checks to the right if it's not right edge, second checks recursive right edge
        } else if (index + 1) % 5 != 0 && *map.get(index + 1).unwrap_or(&0) == 1
            || (index + 1 % 5 == 0 && map[14] == 1)
        {
            count += 1;
        }

        // below
        if index == 7 {
            count += map[0..5].iter().fold(0, |sum, b| sum + b);
        // check if not in bottom row, second condition get recursive level if bottom row
        } else if index < 20 && *map.get(index + 5).unwrap_or(&0) == 1
            || (index >= 20 && map[17] == 1)
        {
            count += 1;
        }

        // to the left
        if index == 13 {
            count += map[4] + map[9] + map[14] + map[19] + map[24];
        // first condition checks if it's not left edge, second checks recursive right edge
        } else if (index) % 5 != 0 && *map.get(index - 1).unwrap_or(&0) == 1
            || (index % 5 == 0 && map[11] == 1)
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

fn run_minute(map: &Vec<i32>, include_recursion: bool) -> Vec<i32> {
    let mut next_state = map.clone();

    for (i, digit) in map.iter().enumerate() {
        let adjacent_bugs = get_adjacent_bugs(&map, i, include_recursion);
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
        map = run_minute(&map, false);

        let sum = sum_binary_value(&map);
        if previous_iterations.contains(&sum) {
            println!("{}", sum);
            break;
        }

        previous_iterations.insert(sum_binary_value(&map));
    }
}

fn solve_p2(mut map: Vec<i32>) {}

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
