fn main() {
    let input = include_str!("../../../../advent-of-code-input/2024/day-01.txt");     
    let output = part1(input);
    println!("{output}");
}

fn part1(input: &str) -> i32 {
    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();
    for row in input.split("\n") {
        let mut parts = row.split_whitespace();
        let left_int: i32 = parts.next().unwrap_or_default().parse().unwrap_or_default();
        let right_int: i32 = parts.next().unwrap_or_default().parse().unwrap_or_default();
        left.push(left_int);
        right.push(right_int);
    }
    left.sort();
    right.sort();
    let difference: Vec<i32> = left.iter().zip(right.iter()).map(|(a, b) | (a - b).abs()).collect();
    difference.into_iter().sum()
}


#[cfg(test)]

mod test {
    use super::*;

    #[test]
    fn it_works() {
        let result = part1("3   4
4   3
2   5
1   3
3   9
3   3");
        assert_eq!(result, 11);
    } 
}

