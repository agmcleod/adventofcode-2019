use std::collections::HashMap;
use std::fmt;

use read_input::read_text;

enum Coordinate {
    Asteroid,
    Open,
}

impl fmt::Debug for Coordinate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Coordinate::Asteroid => "#",
                _ => ".",
            }
        )
    }
}

fn get_angle(coord: &(usize, usize), coord2: &(usize, usize)) -> f32 {
    (coord2.1 as f32 - coord.1 as f32).atan2(coord2.0 as f32 - coord.0 as f32)
}

fn main() {
    let text = read_text("10/input.txt").unwrap();

    let mut grid = HashMap::new();
    let mut coords = Vec::new();

    for (row, line) in text.lines().enumerate() {
        for (col, ch) in line.chars().enumerate() {
            coords.push((col, row));
            grid.insert(
                (col, row),
                match ch {
                    '#' => Coordinate::Asteroid,
                    _ => Coordinate::Open,
                },
            );
        }
    }

    let mut angular_data = HashMap::new();

    for (i, coord) in coords.iter().enumerate() {
        for coord2 in coords.iter().skip(i + 1) {
            let angle = get_angle(&coord, &coord2);
            angular_data.insert((coord, coord2), angle);
            angular_data.insert((coord2, coord), angle);
        }
    }

    let mut best_spot: (Option<(usize, usize)>, usize) = (None, 0);

    for coord in &coords {
        for coord2 in &coords
    }
}
