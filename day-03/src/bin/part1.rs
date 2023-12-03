fn main() {
    let input = include_str!("./input.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> i32 {
    let mut sum = 0;
    // Transform single string into individual lines of strings
    let lines = input.split("\n");

    // Search all lines for non-period symbols

    // Check vicinity of found symbols for numbers

    // Add numbers to sum
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
