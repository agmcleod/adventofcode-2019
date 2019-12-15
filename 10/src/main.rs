use std::collections::HashSet;

use read_input::read_text;

fn get_angle(coord: &(usize, usize), coord2: &(usize, usize)) -> f64 {
    (coord2.1 as f64 - coord.1 as f64).atan2(coord2.0 as f64 - coord.0 as f64)
}

fn main() {
    let text = read_text("10/input.txt").unwrap();

    let mut coords = Vec::new();

    for (row, line) in text.lines().enumerate() {
        for (col, ch) in line.chars().enumerate() {
            if ch == '#' {
                coords.push((col, row));
            }
        }
    }

    let mut most_count = 0;
    let mut best_coord = (0, 0);

    for coord in &coords {
        let mut angles = HashSet::new();
        for coord2 in &coords {
            let angle = get_angle(&coord, &coord2);
            angles.insert(format!("{}", angle));
        }

        if angles.len() > most_count {
            most_count = angles.len();
            best_coord = *coord;
        }
    }

    println!("{}, {:?}", most_count, best_coord);
}
