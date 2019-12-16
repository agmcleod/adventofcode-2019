use std::collections::{HashMap, HashSet};
use std::f64::consts::PI;

use read_input::read_text;

fn get_angle(coord: &(i32, i32), coord2: &(i32, i32)) -> f64 {
    let angle = (coord2.1 as f64 - coord.1 as f64).atan2(coord2.0 as f64 - coord.0 as f64);

    // set origin to top middle
    let angle = angle + (90.0 * PI / 180.0);
    if angle == (360.0 * PI / 180.0) {
        return 0.0;
    }
    angle
}

fn get_degrees(angle: f64) -> i32 {
    if angle == 0.0 {
        return 0;
    }
    let angle = if angle > 0.0 { angle } else { 2.0 * PI + angle };
    (angle * 100000.0 * 180.0 / PI) as i32
}

fn main() {
    let text = read_text("10/input.txt").unwrap();

    let mut coords = Vec::new();

    for (row, line) in text.lines().enumerate() {
        for (col, ch) in line.chars().enumerate() {
            if ch == '#' {
                coords.push((col as i32, row as i32));
            }
        }
    }

    let mut most_count = 0;
    let mut best_coord = (0, 0);

    for coord in &coords {
        let mut angles = HashSet::new();
        for coord2 in &coords {
            let angle = get_angle(&coord, &coord2);

            let angle_key = (angle * 100000.0 * 180.0 / std::f64::consts::PI) as i32;
            angles.insert(angle_key);
        }

        if angles.len() > most_count {
            most_count = angles.len();
            best_coord = *coord;
        }
    }

    println!("{}, {:?}", most_count, best_coord);

    let mut asteroids_by_angle: HashMap<i32, Vec<(i32, i32)>> = HashMap::new();

    for coord in &coords {
        let angle = get_angle(&best_coord, &coord);

        let angle_key = get_degrees(angle);

        if asteroids_by_angle.contains_key(&angle_key) {
            asteroids_by_angle.get_mut(&angle_key).unwrap().push(*coord);
        } else {
            asteroids_by_angle.insert(angle_key, vec![*coord]);
        }
    }

    for (_, coords) in &mut asteroids_by_angle {
        coords.sort_by(|a, b| {
            (best_coord.0 - a.0 + best_coord.1 - a.1)
                .cmp(&(best_coord.0 - b.0 + best_coord.1 - b.1))
        });
    }

    let mut angles: Vec<i32> = asteroids_by_angle.keys().cloned().collect();

    angles.sort();

    let mut i = 0;
    let mut count = 0;
    loop {
        let angle = angles.get(i % angles.len()).unwrap();
        let coords = asteroids_by_angle.get_mut(&angle).unwrap();
        let coord = coords.remove(0);
        if coords.len() == 0 {
            angles.remove(i % angles.len());
            if angles.len() == 0 {
                break;
            }
        } else {
            i += 1;
        }
        count += 1;
        if count == 200 {
            println!("{:?}, {}", coord, coord.0 * 100 + coord.1);
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_angle_returns_0() {
        let angle = get_angle(&(8, 3), &(8, 1));
        assert_eq!(angle, 0.0);
        let angle = get_degrees(angle);

        assert_eq!(angle, 0);
    }
}
