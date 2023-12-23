use std::collections::{HashMap, HashSet};
fn main() {
    let input = include_str!("./input.txt");
    let output = part2(input);
    dbg!(output);
}

fn part2(input: &str) -> i32 {
    // Use HashMap: Key is card number, value is quantity
    let mut cards: HashMap<usize, i32> = HashMap::new();
    let mut lines = input.lines();
    let max_card_num = lines.clone().count();
    for num in 1..=max_card_num {
        cards.insert(num, 1);
    }
    let mut current_number = 1;
    while let Some(line) = lines.next() {
        let split: Vec<&str> = line.split("|").collect();
        let first_half = split[0].trim();
        let winning_numbers = first_half.split(":").collect::<Vec<&str>>()[1].trim();
        let winning_numbers: Vec<i32> = winning_numbers.split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect();

        let second_half = split[1].trim();
        let numbers: Vec<i32> = second_half.split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect();

        let set_winning_numbers: HashSet<&i32> = winning_numbers.iter().collect();
        let set_numbers: HashSet<&i32> = numbers.iter().collect();
        let num_winning_numbers = set_winning_numbers.intersection(&set_numbers).count();
        let number_of_card_i = cards.get(&current_number).unwrap().to_owned();
        for i in current_number+1..=current_number+num_winning_numbers {
            *cards.entry(i).or_insert(0) += number_of_card_i;
        }
        current_number += 1;
    }

    cards.into_values().sum()
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part2("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
        Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
        Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
        Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
        Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11");
        assert_eq!(result, 30);
    }
}