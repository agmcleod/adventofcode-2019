use read_input::read_text;

use std::collections::{HashMap, HashSet};

fn traverse_stars(
    orbit_hierarchy: &HashMap<String, Vec<String>>,
    star: &String,
    count: usize,
) -> usize {
    if !orbit_hierarchy.contains_key(star) {
        return count;
    }

    orbit_hierarchy
        .get(star)
        .unwrap()
        .iter()
        .fold(count, |sum, star| {
            sum + traverse_stars(orbit_hierarchy, star, count + 1)
        })
}

fn next_star(
    reverse_direction: &HashMap<String, String>,
    count: usize,
    you_stars: &mut HashMap<String, usize>,
    san_stars: &mut HashMap<String, usize>,
    next_you_star: &String,
    next_san_star: &String,
) -> usize {
    let mut next_san_star = next_san_star;
    let mut next_you_star = next_you_star;
    if let Some(you) = reverse_direction.get(next_you_star) {
        you_stars.insert(you.clone(), count);
        next_you_star = you;
        if san_stars.contains_key(you) {
            return count + san_stars.get(you).unwrap();
        }
    }

    if let Some(san) = reverse_direction.get(next_san_star) {
        san_stars.insert(san.clone(), count);
        next_san_star = san;
        if you_stars.contains_key(san) {
            return count + you_stars.get(san).unwrap();
        }
    }

    next_star(
        reverse_direction,
        count + 1,
        you_stars,
        san_stars,
        next_you_star,
        next_san_star,
    )
}

fn main() {
    let text = read_text("6/input.txt").unwrap();

    let mut orbit_hierarchy: HashMap<String, Vec<String>> = HashMap::new();
    let mut reverse_direction: HashMap<String, String> = HashMap::new();
    let mut has_parent = HashSet::new();

    for line in text.lines() {
        let stars: Vec<&str> = line.split(")").collect();

        reverse_direction.insert(stars[1].to_string(), stars[0].to_string());

        if orbit_hierarchy.contains_key(stars[0]) {
            orbit_hierarchy
                .get_mut(stars[0])
                .unwrap()
                .push(stars[1].to_string());
        } else {
            orbit_hierarchy.insert(stars[0].to_string(), vec![stars[1].to_string()]);
        }

        has_parent.insert(stars[1].to_string());
    }

    let mut total = 0;
    for (star, _) in &orbit_hierarchy {
        if !has_parent.contains(star) {
            total = traverse_stars(&orbit_hierarchy, star, 0);
        }
    }

    println!("{}", total);

    let mut you_stars = HashMap::new();
    let mut san_stars = HashMap::new();

    let you = reverse_direction.get("YOU").unwrap();
    you_stars.insert(you.clone(), 0);

    let san = reverse_direction.get("SAN").unwrap();
    san_stars.insert(san.clone(), 0);

    let orbit_diff = next_star(
        &reverse_direction,
        1,
        &mut you_stars,
        &mut san_stars,
        you,
        san,
    );

    println!("{}", orbit_diff);
}
