use read_input::read_text;

use std::collections::{HashMap, HashSet};

fn main() {
    let text = read_text("6/input.txt").unwrap();

    let mut orbit_hierarchy: HashMap<String, String> = HashMap::new();
    let mut has_parent = HashSet::new();

    for line in text.lines() {
        let stars: Vec<&str> = line.split(")").collect();

        orbit_hierarchy.insert(stars[0].to_string(), stars[1].to_string());
        has_parent.insert(stars[1].to_string());
    }

    // TODO: rethink direction, might need to go other way to do count properly

    let mut total = 0;
    let mut counted: HashMap<String, usize> = HashMap::new();
    for (star, child_star) in &orbit_hierarchy {
        if !has_parent.contains(star) {
            println!("star {}", star);
            let mut count = 1;
            total += count;
            let mut child_star = child_star;
            loop {
                if let Some(child) = orbit_hierarchy.get(child_star) {
                    // if counted.contains_key(child) {
                    //     break;
                    // }
                    count += 1;
                    total += count;
                    child_star = child;
                // counted.insert(child);
                } else {
                    break;
                }
            }
        }
    }

    println!("{}", total);
}
