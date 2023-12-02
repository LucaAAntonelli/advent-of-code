use std::str;

fn main() {
    let input = include_str!("./input2.txt");
    let output = part2(input);
    dbg!(output);
}

fn part2(input: &str) -> String {
    let mut lines = input.lines();
    let mut sum = 0;
    let numeric_values: Vec<char> = "0123456789".chars().collect();
    while let Some(line) = lines.next() {
        // Find first numeric value
        let first = line.find(|x| numeric_values.contains(&x)).unwrap();
        // Find last numeric value
        let last = line.rfind(|x| numeric_values.contains(&x)).unwrap();
        // Combine to string

        let calibration_value = format!(
            "{}{}",
            str::from_utf8(&vec![line.as_bytes()[first]]).unwrap(),
            str::from_utf8(&vec![line.as_bytes()[last]]).unwrap()
        );
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
