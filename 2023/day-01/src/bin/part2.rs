use std::str;

fn main() {
    let input = include_str!("../../../../advent-of-code-input/2023/day-01.txt");
    let output = part2(input);
    dbg!(output);
}

fn part2(input: &str) -> i32 {
    let numbers_spelled_out_with_letters = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    input
        .lines()
        .map(|line| {
            let mut characters_digits: Vec<char> = vec![];
            let mut temporary = String::from("");
            for character in line.chars() {
                temporary += &character.to_string();

                let mut temporary_spelled_number_index = None;
                for (index, spelled_number) in numbers_spelled_out_with_letters.iter().enumerate() {
                    if temporary.contains(spelled_number) {
                        temporary_spelled_number_index = Some(index);
                        break;
                    }
                }
                if let Some(temporary_spelled_number_index) = temporary_spelled_number_index {
                    let number = temporary_spelled_number_index + 1;
                    characters_digits.push(
                        number
                            .to_string()
                            .chars()
                            .next()
                            .expect("Number should be single-character digit."),
                    );
                    temporary = character.to_string();
                }

                if character.is_ascii_digit() {
                    characters_digits.push(character);
                    temporary = String::from("");
                }
            }

            let first_digit = characters_digits.first().unwrap_or(&'0').to_owned();
            let last_digit = characters_digits.last().unwrap_or(&'0').to_owned();
            let number = format!("{}{}", first_digit, last_digit);
            let number: i32 = number.parse().expect("Should parse as a number.");
            number
        })
        .sum()
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part2(
            "two1nine
            eightwothree
            abcone2threexyz
            xtwone3four
            4nineeightseven2
            zoneight234
            7pqrstsixteen",
        );
        assert_eq!(result, 281);
    }
}
