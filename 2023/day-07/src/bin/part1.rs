fn main() {
    let input = include_str!("./input.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> i32 {
    todo!()
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part1(
            "32T3K 765
            T55J5 684
            KK677 28
            KTJJT 220
            QQQJA 483
            ",
        );
        assert_eq!(result, 6440);
    }
}
