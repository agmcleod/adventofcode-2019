use read_input::read_text;

fn main() {
    let text = read_text("8/input.txt").unwrap();

    let numbers: Vec<&str> = text.split("").filter(|v| v != &"").collect();

    let width = 25;
    let height = 6;

    let mut buffer = vec!["2"; width * height];

    let mut zero_count: Option<(usize, usize)> = None;
    for (index, layer) in numbers.chunks(width * height).enumerate() {
        let count = layer.iter().filter(|v| *v == &"0").count();
        if zero_count.is_some() {
            if count < zero_count.unwrap().1 {
                zero_count = Some((index, count));
            }
        } else {
            zero_count = Some((index, count));
        }

        for (i, pixel) in layer.iter().enumerate() {
            if pixel == &"1" || pixel == &"0" {
                if buffer[i] == "2" {
                    buffer[i] = pixel;
                }
            }
        }
    }

    if let Some(zero_count) = zero_count {
        let layer = numbers
            .chunks(width * height)
            .skip(zero_count.0)
            .next()
            .unwrap();
        let ones = layer.iter().filter(|v| *v == &"1").count();
        let twos = layer.iter().filter(|v| *v == &"2").count();
        println!("{}", ones * twos);
    } else {
        println!("Failed to find zeros");
    }

    for row in buffer.chunks(width) {
        println!("{}", row.join("").replace("2", " ").replace("0", " "));
    }
}
