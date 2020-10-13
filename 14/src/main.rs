use std::collections::HashMap;

use read_input::read_text;

#[derive(Clone, Debug)]
struct Requirement {
    name: String,
    amount: i64,
}

impl Requirement {
    fn new(name: String, amount: i64) -> Self {
        Requirement { name, amount }
    }
}

#[derive(Clone, Debug)]
struct Reaction {
    requirements: Vec<Requirement>,
    output_type: String,
    output_amount: i64,
}

impl Reaction {
    fn new(requirements: Vec<Requirement>, output_type: String, output_amount: i64) -> Self {
        Reaction {
            requirements,
            output_type,
            output_amount,
        }
    }
}

fn get_requirement_from_text(text: &str) -> Requirement {
    let mut iter = text.split(" ");
    let number = iter.next().unwrap().parse().unwrap();
    let chemical = iter.next().unwrap().to_string();

    Requirement::new(chemical, number)
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
            let amount_in_factory = factory.get_mut(&requirement.name).unwrap_or(&mut default);

            let target = requirement.amount * multiplier;

            if *amount_in_factory >= target {
                // already have enough, consume it from the factory
                *amount_in_factory -= target;
                // dont pass additional, as we didnt manufacturer the resources,
                // also it probably wasn't ore
                sum + 0
            } else {
                // base material, like ORE
                if !reactions.contains_key(&requirement.name) {
                    // return the ore produced
                    sum + target
                } else {
                    let requirement_reaction = reactions.get(&requirement.name).unwrap();
                    let mut ore_total = 0;

                    loop {
                        ore_total += sum_amounts_for_chemical(
                            reactions,
                            factory,
                            requirement_reaction,
                            multiplier,
                        );
                        add_to_factory(
                            factory,
                            &requirement_reaction.output_type,
                            requirement_reaction.output_amount * multiplier,
                        );
                        if *factory.get(&requirement_reaction.output_type).unwrap() >= target {
                            break;
                        }
                    }

                    *factory.get_mut(&requirement_reaction.output_type).unwrap() -= target;

                    sum + ore_total
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
        let output_details = get_requirement_from_text(output);
        requirements.insert(
            output_details.name.clone(),
            Reaction::new(
                inputs
                    .split(", ")
                    .map(|input| get_requirement_from_text(input))
                    .collect(),
                output_details.name,
                output_details.amount,
            ),
        );
    }

    let ore_per_fuel = sum_amounts_for_chemical(
        &requirements,
        &mut factory.clone(),
        requirements.get("FUEL").unwrap(),
        1,
    );
    println!("fuel for 1 ore {}", ore_per_fuel);

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

        if ore_per_fuel == tril {
            println!("{}", current);
            break;
        } else if max - min <= 1 {
            println!("{}, {}", current, ore_per_fuel);
            break;
        } else if ore_per_fuel > tril {
            max = current;
        } else {
            min = current;
        }
    }
}
