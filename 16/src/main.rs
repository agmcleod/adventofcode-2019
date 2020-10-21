use read_input::read_text;

fn main() {
    let text = read_text("16/input.txt").unwrap();
    let mut input_list: Vec<i32> = text
        .chars()
        .map(|c| c.to_digit(10).unwrap() as i32)
        .collect();

    let pattern = vec![0, 1, 0, -1];

    for _ in 0..100 {
        let output: Vec<i32> = (1..=input_list.len())
            .map(|iteration| {
                let total: i32 = (input_list.iter().enumerate().map(|(i, n)| {
                    let multiplier = pattern[((i + 1) / iteration) % pattern.len()];

                    *n * multiplier
                }))
                .sum();

                total
                    .to_string()
                    .chars()
                    .last()
                    .unwrap()
                    .to_digit(10)
                    .unwrap() as i32
            })
            .collect();

        input_list = output;
    }

    println!(
        "{:?}",
        input_list[0..8]
            .iter()
            .map(|n| n.to_string())
            .collect::<String>()
    );
}
