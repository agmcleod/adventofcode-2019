fn collect_digits(num: usize) -> [usize; 6] {
    let mut digits = [0; 6];

    let mut num_copy = num;
    let mut index = 5;

    loop {
        digits[index] = num_copy % 10;
        if index == 0 {
            break;
        }
        index -= 1;

        num_copy /= 10;

        if num_copy == 0 {
            break;
        }
    }

    digits
}

fn solve(distinct_pairs: bool) -> usize {
    let mut start = 193651;
    let mut count = 0;

    loop {
        if start >= 649729 {
            break;
        }

        let digits = collect_digits(start);

        let mut current_min = digits[0];
        let mut ascends = true;
        let mut pair_size = 0;
        let mut has_pair = false;
        let mut has_even_pairs = false;
        for group in digits.windows(2) {
            if group[0] == group[1] {
                has_pair = true;
                if pair_size == 0 {
                    pair_size += 2;
                } else {
                    pair_size += 1;
                }
            } else {
                if pair_size == 2 {
                    has_even_pairs = true;
                }
                pair_size = 0;
            }

            // since we initialize as the first number, we only ever have to check the second number
            if current_min > group[1] {
                ascends = false;
            } else if group[1] > current_min {
                current_min = group[1];
            }
        }

        if pair_size == 2 {
            has_even_pairs = true;
        }

        if ascends && has_pair && ((distinct_pairs && has_even_pairs) || !distinct_pairs) {
            count += 1;
        }

        start += 1;
    }

    count
}

fn main() {
    println!("count {}", solve(false));
    println!("count {}", solve(true));
}
