use read_input::read_text;
use std::collections::HashMap;

struct Reaction {
    requirements: Vec<(String, i32)>,
    output_type: String,
    output_amount: i32,
}

impl Reaction {
    fn new(requirements: Vec<(String, i32)>, output_type: String, output_amount: i32) -> Self {
        Reaction {
            requirements,
            output_type,
            output_amount,
        }
    }
}

fn get_count_with_chemical(text: &str) -> (String, i32) {
    let mut iter = text.split(" ");
    let number = iter.next().unwrap().parse().unwrap();
    let chemical = iter.next().unwrap().to_string();

    (chemical, number)
}

fn add_to_factory(factory: &mut HashMap<String, i32>, reaction: &String, amount: i32) {
    if factory.contains_key(reaction) {
        *factory.get_mut(reaction).unwrap() += amount;
    } else {
        factory.insert(reaction.clone(), amount);
    }

    println!(
        "Added {} {}, total {}",
        amount,
        reaction,
        factory.get(reaction).unwrap()
    );
}

fn sum_amounts_for_chemical(
    reactions: &HashMap<String, Reaction>,
    factory: &mut HashMap<String, i32>,
    current_reaction: &Reaction,
) -> i32 {
    current_reaction
        .requirements
        .iter()
        .fold(0, |sum, requirement| {
            let mut default = 0;
            let amount = factory.get_mut(&requirement.0).unwrap_or(&mut default);

            println!(
                "Need {}, we have {} of {}",
                requirement.1, amount, requirement.0
            );

            if *amount > requirement.1 {
                // consume it from the factory
                *amount -= requirement.1;
                sum + 0
            } else {
                // base material, like ORE`
                if !reactions.contains_key(&requirement.0) {
                    println!("returning {} + {} {}", sum, requirement.1, requirement.0);
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

                    sum + sub_total
                }
            }
        })
}

fn main() {
    let text = read_text("14/input.txt").unwrap();

    let mut requirements = HashMap::new();
    let mut factory = HashMap::new();

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
    println!(
        "{}",
        sum_amounts_for_chemical(&requirements, &mut factory, reaction)
    );
}
