use std::cmp::min;

use regex::Regex;
fn main() {
    let input = include_str!("./input.txt");
    let output = part2(input);
    dbg!(output);
}

fn part2(input: &str) -> i32 {
    let mut sum = 0;
    let binding = input.replace(" ", "");
    let asterisk_regex = Regex::new(r"\*").unwrap();
    let number_regex = Regex::new(r"\d+").unwrap();
    let rows: Vec<&str> = binding.split("\n").into_iter().collect();
    let mut asterisk_indices: Vec<(usize, usize)> = vec![];
    let mut number_indices: Vec<(usize, usize)> = vec![];
    for (row_index, row) in rows.iter().enumerate() {
        for mat in asterisk_regex.find_iter(row) {
            let index = mat.start();
            println!("Asterisk at ({row_index}, {index})");
            asterisk_indices.push((
                usize::saturating_sub(row_index, 1),
                usize::saturating_sub(index, 1),
            )); // Top left corner
            asterisk_indices.push((usize::saturating_sub(row_index, 1), index)); // Top
            asterisk_indices.push((
                usize::saturating_sub(row_index, 1),
                min(index + 1, row.len() - 1),
            )); // Top right corner

            asterisk_indices.push((row_index, usize::saturating_sub(index, 1))); // Left
            asterisk_indices.push((row_index, min(index + 1, row.len() - 1))); // Right

            asterisk_indices.push((
                min(row_index + 1, rows.len() - 1),
                usize::saturating_sub(index, 1),
            ));
            asterisk_indices.push((min(row_index + 1, rows.len() - 1), index));
            asterisk_indices.push((
                min(row_index + 1, rows.len() - 1),
                usize::saturating_add(index, 1),
            ));
        }
        for mat in number_regex.find_iter(row) {
            let value: i32 = mat.as_str().parse().unwrap();
            let start = mat.start();
            let end = mat.end();
            println!("Value {value} from ({row_index}, {start}) to ({row_index},{end}) ")
        }
    }
    println!("{:?}", asterisk_indices);
    sum
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part2(
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
            .........*
            ",
        );
        assert_eq!(result, 467835);
    }
}
