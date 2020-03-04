use std::collections::HashMap;

use read_input::read_text;

#[derive(Clone, Debug)]
struct Reaction {
    requirements: Vec<(String, i64)>,
    output_type: String,
    output_amount: i64,
}

impl Reaction {
    fn new(requirements: Vec<(String, i64)>, output_type: String, output_amount: i64) -> Self {
        Reaction {
            requirements,
            output_type,
            output_amount,
        }
    }
}

fn get_count_with_chemical(text: &str) -> (String, i64) {
    let mut iter = text.split(" ");
    let number = iter.next().unwrap().parse().unwrap();
    let chemical = iter.next().unwrap().to_string();

    (chemical, number)
}

fn add_to_factory(factory: &mut HashMap<String, i64>, reaction: &String, amount: i64) {
    if factory.contains_key(reaction) {
        *factory.get_mut(reaction).unwrap() += amount;
    } else {
        factory.insert(reaction.clone(), amount);
    }
}

fn sum_amounts_for_chemical(
    reactions: &HashMap<String, Reaction>,
    factory: &mut HashMap<String, i64>,
    current_reaction: &Reaction,
    multiplier: i64,
) -> i64 {
    current_reaction
        .requirements
        .iter()
        .fold(0, |sum, requirement| {
            let mut default = 0;
            let amount = factory.get_mut(&requirement.0).unwrap_or(&mut default);

            if *amount >= requirement.1 * multiplier {
                // already have enough, consume it from the factory
                *amount -= requirement.1 * multiplier;
                // dont pass additional, as we didnt manufacturer the resources
                sum + 0
            } else {
                // base material, like ORE
                if !reactions.contains_key(&requirement.0) {
                    // return the ore produced
                    sum + requirement.1 * multiplier
                } else {
                    let reaction = reactions.get(&requirement.0).unwrap();
                    let mut sub_total = 0;
                    let target = requirement.1 * multiplier;

                    loop {
                        sub_total +=
                            sum_amounts_for_chemical(reactions, factory, reaction, multiplier);
                        add_to_factory(
                            factory,
                            &reaction.output_type,
                            reaction.output_amount * multiplier,
                        );
                        if *factory.get(&reaction.output_type).unwrap() >= target {
                            break;
                        }
                    }

                    *factory.get_mut(&reaction.output_type).unwrap() -= target;

                    sum + sub_total
                }
            }
        })
}

fn main() {
    let text = read_text("14/input.txt").unwrap();

    let mut requirements = HashMap::new();
    let mut factory = HashMap::new();
    factory.insert("ORE".to_string(), 0);

    for line in text.lines() {
        let mut iter = line.split(" => ");
        let inputs = iter.next().unwrap();
        let output = iter.next().unwrap();
        let output_details = get_count_with_chemical(output);
        requirements.insert(
            output_details.0.clone(),
            Reaction::new(
                inputs
                    .split(", ")
                    .map(|input| get_count_with_chemical(input))
                    .collect(),
                output_details.0,
                output_details.1,
            ),
        );
    }

    let ore_per_fuel = sum_amounts_for_chemical(
        &requirements,
        &mut factory.clone(),
        requirements.get("FUEL").unwrap(),
        1,
    );
    println!("{}", ore_per_fuel);

    let mut multiplier = 10;
    let tril = 1_000_000_000_000;

    // find high bound multiplier
    loop {
        let ore_per_fuel = sum_amounts_for_chemical(
            &requirements,
            &mut factory.clone(),
            requirements.get("FUEL").unwrap(),
            multiplier,
        );
        if ore_per_fuel >= tril {
            break;
        }

        multiplier *= 10;
    }

    let mut min = multiplier / 10;
    let mut max = tril;

    // binary search
    loop {
        let current = (max - min) / 2 + min;

        let mut attempt_factory = factory.clone();
        let ore_per_fuel = sum_amounts_for_chemical(
            &requirements,
            &mut attempt_factory,
            requirements.get("FUEL").unwrap(),
            current,
        );

        // println!("=====\n{} < {}, {}", min, max, current);
        // println!("{}", ore_per_fuel);
        // println!("{}\n=====", tril);
        if ore_per_fuel == tril {
            println!("{}", current);
            break;
        } else if max - min <= 1 {
            println!("{}, {}", min, ore_per_fuel);
            break;
        } else if ore_per_fuel > tril {
            max = current;
        } else {
            min = current;
        }
    }
}
