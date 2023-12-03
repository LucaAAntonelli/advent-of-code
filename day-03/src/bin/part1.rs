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
    let norm = vec!['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '.'];
    let rows: Vec<&str> = binding.split("\n").into_iter().collect();
    let re = Regex::new(r"\d+").unwrap();
    // Find numbers with regex
    for (row_it, row) in rows.iter().enumerate() {
        'matches: for mat in re.find_iter(row) {
            // Check surrounding indices for special characters
            let num: i32 = mat.as_str().parse().unwrap();
            let start = mat.start();
            let end = mat.end();
            let mut indices_to_check: Vec<(usize, usize)> = vec![];
            for asdf in [
                usize::saturating_sub(row_it, 1),
                min(rows.len() - 1, row_it),
            ] {
                for col_it in usize::saturating_sub(start, 1)..=min(end, row.len() - 1) {
                    indices_to_check.push((asdf, col_it));
                }
            }
            indices_to_check.push((row_it, usize::saturating_sub(start, 1)));
            indices_to_check.push((row_it, min(row.len(), end)));
            println!("For value {num}, check the following indices:");
            for (row_index, col_index) in indices_to_check {
                println!("Index: ({row_index}, {col_index})");
                println!(
                    "Character: {}",
                    &rows[row_index].chars().nth(col_index).unwrap()
                );
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
            2........1
            ",
        );
        assert_eq!(result, 4361);
    }
}
