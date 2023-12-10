use std::cmp::min;

use regex::Regex;
fn main() {
    let input = include_str!("./input.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> i32 {
    let mut sum = 0;
    // Transform single string into individual lines of strings
    let binding = input.replace(" ", "");
    // Create vector of characters that DO NOT count towards special symbols
    let norm = vec!['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '.'];

    // Split input string into a vector of strings
    let rows: Vec<&str> = binding.split("\n").into_iter().collect();

    // Using regular expressions, find all positive integers in the text
    let re = Regex::new(r"\d+").unwrap();
    // Find numbers with regex
    for (row_it, row) in rows.iter().enumerate() {
        'matches: for mat in re.find_iter(row) {
            // num is the value of the positive integer
            let num: i32 = mat.as_str().parse().unwrap();
            // start is the first index of the positive integer
            let start = mat.start();
            // end is the first character that is no longer part of the integer
            let end = mat.end();
            let mut indices_to_check: Vec<(usize, usize)> = vec![];
            // Check all indices that are adjacent to any of the digits
            for asdf in [
                usize::saturating_sub(row_it, 1),
                min(rows.len() - 1, row_it + 1),
            ] {
                for col_it in usize::saturating_sub(start, 1)..=min(end, row.len() - 1) {
                    indices_to_check.push((asdf, col_it));
                }
            }
            indices_to_check.push((row_it, usize::saturating_sub(start, 1)));
            indices_to_check.push((row_it, min(end, row.len() - 1)));
            for (row_index, col_index) in indices_to_check {
                // If there is a special character in any of the checked indices, add the number to the total sum and continue
                if !norm.contains(&rows[row_index].chars().nth(col_index).unwrap()) {
                    sum += num;
                    continue 'matches;
                }
            }
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
            "467..114..
            ...*......
            ..35..633.
            ......#...
            617*......
            .....+.58.
            ..592.....
            ......755.
            ...$.*....
            .664.598..
            ",
        );
        assert_eq!(result, 4361);
    }
}
