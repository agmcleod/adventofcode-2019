use read_input::read_text;

fn get_mass(num: isize) -> isize {
    num / 3 - 2
}

fn main() {
    let text = read_text("1/input.txt").unwrap();

    let total = text
        .lines()
        .map(|line| {
            let num: isize = line.parse().expect("Could not parse to num");
            get_mass(num)
        })
        .fold(0, |sum, item| sum + item);

    println!("{:?}", total);

    let total = text
        .lines()
        .map(|line| {
            let mut num: isize = line.parse().expect("Could not parse to num");
            let mut sub_sum = 0;
            loop {
                num = get_mass(num);
                if num > 0 {
                    sub_sum += num
                } else {
                    break;
                }
            }
            sub_sum
        })
        .fold(0, |sum, item| sum + item);

    println!("{:?}", total);
}
