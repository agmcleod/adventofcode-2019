use read_input::read_text;

fn get_initial(text: &String) -> Vec<i32> {
    text.chars()
        .map(|c| c.to_digit(10).unwrap() as i32)
        .collect()
}

fn fft(mut input_list: Vec<i32>) -> Vec<i32> {
    let pattern = vec![0, 1, 0, -1];

    for _ in 0..100 {
        let output: Vec<i32> = (1..=input_list.len())
            .map(|iteration| {
                let total: i32 = (input_list.iter().enumerate().map(|(i, n)| {
                    let multiplier = pattern[((i + 1) / iteration) % pattern.len()];

                    *n * multiplier
                }))
                .sum();

                (total % 10).abs()
            })
            .collect();

        input_list = output;
    }

    input_list
}

fn main() {
    let text = read_text("16/input.txt").unwrap();
    let input_list = fft(get_initial(&text));

    println!(
        "p1: {:?}",
        input_list[0..8]
            .iter()
            .map(|n| n.to_string())
            .collect::<String>()
    );

    let initial: Vec<i32> = get_initial(&text);

    let offset: String = initial[0..7]
        .iter()
        .fold(String::new(), |result, n| format!("{}{}", result, n));

    let offset: usize = offset.parse().unwrap();

    let target_capacity = initial.len() * 10000;
    let mut long_input: Vec<i32> = Vec::with_capacity(target_capacity);

    while long_input.len() < target_capacity {
        long_input.append(&mut initial.clone());
    }

    let mut new_list = long_input.clone();
    for _ in 0..100 {
        for i in (offset..long_input.len()).rev() {
            new_list[i] = if i + 1 < long_input.len() {
                (long_input[i] + new_list[i + 1]) % 10
            } else {
                long_input[i]
            };
        }

        long_input = new_list.clone();
    }

    println!(
        "p2: {:?}",
        new_list[offset..offset + 8]
            .iter()
            .map(|n| n.to_string())
            .collect::<String>()
    );
}
