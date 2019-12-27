use std::cmp;
use std::collections::HashMap;

use read_input::read_text;

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
) -> i64 {
    current_reaction
        .requirements
        .iter()
        .fold(0, |sum, requirement| {
            let mut default = 0;
            let amount = factory.get_mut(&requirement.0).unwrap_or(&mut default);

            if *amount >= requirement.1 {
                // consume it from the factory
                *amount -= requirement.1;
                sum + 0
            } else {
                // base material, like ORE
                if !reactions.contains_key(&requirement.0) {
                    *factory.get_mut(&requirement.0).unwrap() -= requirement.1;
                    sum + requirement.1
                } else {
                    let reaction = reactions.get(&requirement.0).unwrap();
                    let mut sub_total = 0;

                    loop {
                        sub_total += sum_amounts_for_chemical(reactions, factory, reaction);
                        add_to_factory(factory, &reaction.output_type, reaction.output_amount);
                        if *factory.get(&reaction.output_type).unwrap() >= requirement.1 {
                            break;
                        }
                    }

                    *factory.get_mut(&reaction.output_type).unwrap() -= requirement.1;

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

    let reaction = requirements.get("FUEL").unwrap();
    let ore_per_fuel = sum_amounts_for_chemical(&requirements, &mut factory, reaction);
    println!("{}", ore_per_fuel);
}
