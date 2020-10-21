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

    input_list
}

fn main() {
    let text = read_text("16/input.txt").unwrap();
    let input_list = fft(get_initial(&text));

    println!(
        "{:?}",
        input_list[0..8]
            .iter()
            .map(|n| n.to_string())
            .collect::<String>()
    );

    let initial: Vec<i32> = get_initial(&text);

    let mut input_list: Vec<i32> = Vec::with_capacity(input_list.len() * 10000);

    for _ in 0..10000 {
        input_list.append(&mut initial.clone());
    }

    let input_list = fft(input_list);

    println!(
        "{:?}",
        input_list[0..8]
            .iter()
            .map(|n| n.to_string())
            .collect::<String>()
    );
}
