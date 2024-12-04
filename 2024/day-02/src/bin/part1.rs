fn main() {
    let input = include_str!("../../../../advent-of-code-input/2024/day-02.txt");
    let output = part1(input);
    println!("{output}");
}

fn part1(input: &str) -> u32 {
    let mut num_save_reports: u32 = 0;
    for row in input.split("\n") {
        if is_save(row) {
            num_save_reports += 1;
        }
    }
    num_save_reports
}

fn is_save(input: &str) -> bool {
    let vector: Vec<u32> = input.split_whitespace().filter_map(|c| c.parse::<u32>().ok()).collect();
    if vector.is_sorted_by(|a, b| a <= b) || vector.is_sorted_by(|a, b| a >= b) {
        vector.windows(2).all(|w| w[0].abs_diff(w[1]) < 4 && w[0].abs_diff(w[1]) > 0)
    } else {
        false
    }
}

#[cfg(test)]

mod test {
    use super::*;

    #[test]
    fn it_works() {
        let result = part1("7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9");
        assert_eq!(result, 2)
    }
}