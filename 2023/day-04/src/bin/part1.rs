use std::collections::HashSet;
fn main() {
    let input = include_str!("../../../../advent-of-code-input/2023/day-04.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> i32 {
    let mut sum = 0;
    let mut lines = input.lines();

    while let Some(line) = lines.next() {
        let split: Vec<&str> = line.split("|").collect();
        let first_half = split[0].trim();
        let winning_numbers = first_half.split(":").collect::<Vec<&str>>()[1].trim();
        let mut vec_winning_numbers: Vec<&str> = winning_numbers.split(" ").collect();
        vec_winning_numbers.retain(|&x| x != "");
        let own_numbers = split[1].trim();
        let mut vec_own_numbers: Vec<&str> = own_numbers.split(" ").collect();
        vec_own_numbers.retain(|&x| x != "");
        let set_winning_numbers: HashSet<&str> = vec_winning_numbers.into_iter().collect();
        let set_own_numbers: HashSet<&str> = vec_own_numbers.into_iter().collect();
        let intersection = set_own_numbers.intersection(&set_winning_numbers);
        let count = intersection.count();
        if count == 0 {
            continue;
        } else if count == 1 {
            sum += 1;
        } else {
            let base: i32 = 2;
            sum += base.pow(count as u32 - 1);
        }
    }

    sum
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part1(
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
        Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
        Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
        Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
        Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
        );
        assert_eq!(result, 13);
    }
}
