use std::cmp;
use std::str;

fn main() {
    let input = include_str!("./input2.txt");
    let output = part2(input);
    dbg!(output);
}

fn find_first_spelled_out_value(line: &str) -> Option<(usize, usize)> {
    let spelled_out_values = vec![
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "zero",
    ];
    for (pos, value) in spelled_out_values.iter().enumerate() {
        if let Some(index) = line.find(value) {
            return Some((index, pos));
        }
    }
    None
}

fn find_last_spelled_out_value(line: &str) -> Option<(usize, usize)> {
    let spelled_out_values = vec![
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    for (pos, value) in spelled_out_values.iter().enumerate() {
        if let Some(index) = line.rfind(value) {
            return Some((index, pos));
        }
    }
    None
}

fn part2(input: &str) -> String {
    let mut lines = input.lines();
    let mut sum = 0;
    let numeric_values: Vec<char> = "0123456789".chars().collect();
    let mut first_digit = String::from("");
    let mut last_digit = String::from("");

    while let Some(line) = lines.next() {
        // Find first numeric value
        let first_numeric = line.find(|x| numeric_values.contains(&x)).unwrap();
        if let Some((first_spelled, val_first_spelled)) = find_first_spelled_out_value(line) {
            if first_spelled < first_numeric {
                first_digit = val_first_spelled.to_string();
            } else {
                first_digit = str::from_utf8(&vec![line.as_bytes()[first_numeric]])
                    .unwrap()
                    .to_owned();
            }
        }
        // Find last numeric value
        let last_numeric = line.rfind(|x| numeric_values.contains(&x)).unwrap();
        if let Some((last_spelled, val_last_spelled)) = find_last_spelled_out_value(line) {
            if last_spelled > last_numeric {
                last_digit = val_last_spelled.to_string();
            } else {
                last_digit = str::from_utf8(&vec![line.as_bytes()[last_numeric]])
                    .unwrap()
                    .to_owned();
            }
        }
        // Combine to string

        let calibration_value = format!("{}{}", first_digit, last_digit);
        println!("{calibration_value}");
        // Parse to int
        sum += calibration_value.parse::<i32>().unwrap();
        // Add to sum
    }

    sum.to_string()
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
        assert_eq!(result, "281");
    }
}
