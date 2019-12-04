use std::cmp;

use read_input::read_text;

mod math;

fn main() {
    let text = read_text("3/input.txt").unwrap();

    let coords: Vec<Vec<(i32, i32)>> = text
        .lines()
        .map(|line| {
            let mut coord = (0, 0);
            let mut coords = vec![coord];
            for instruction in line.split(",") {
                let direction = instruction.get(0..1).unwrap();
                let amount: i32 = instruction
                    .get(1..)
                    .unwrap()
                    .parse()
                    .expect("could not parse to number");

                match direction {
                    "D" => {
                        coord.1 += amount;
                    }
                    "L" => {
                        coord.0 -= amount;
                    }
                    "R" => {
                        coord.0 += amount;
                    }
                    "U" => {
                        coord.1 -= amount;
                    }
                    _ => panic!("unknown direction {}", direction),
                }
                coords.push(coord);
            }
            coords
        })
        .collect();

    let mut distance: Option<i32> = None;
    let mut found_distance = false;
    let mut distance_for_one = 0;
    for set_one in coords[0].windows(2) {
        let mut distance_for_two = 0;
        for set_two in coords[1].windows(2) {
            if set_one[0] != (0, 0) && set_two[0] != (0, 0) {
                if math::do_intersect(&set_one[0], &set_one[1], &set_two[0], &set_two[1]) {
                    if let Some(intersection) = math::get_intersect_point(
                        &set_one[0],
                        &set_one[1],
                        &set_two[0],
                        &set_two[1],
                    ) {
                        if !found_distance {
                            found_distance = true;

                            let one = distance_for_one
                                + (set_one[0].0 - intersection.0 + set_one[0].1 - intersection.1)
                                    .abs();
                            let two = distance_for_two
                                + (set_two[0].0 - intersection.0 + set_two[0].1 - intersection.1)
                                    .abs();
                            println!("total steps {}", one + two);
                        }

                        let intersection_distance = intersection.0.abs() + intersection.1.abs();

                        if distance.is_none() {
                            distance = Some(intersection_distance);
                        } else {
                            distance = Some(cmp::min(distance.unwrap(), intersection_distance));
                        }
                    }
                }
            }
            distance_for_two += (set_two[0].0 - set_two[1].0 + set_two[0].1 - set_two[1].1).abs();
        }
        distance_for_one += (set_one[0].0 - set_one[1].0 + set_one[0].1 - set_one[1].1).abs();
    }

    println!("{:?}", distance);
}
