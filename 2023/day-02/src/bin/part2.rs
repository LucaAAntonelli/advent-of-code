use std::{cmp::max, str};

fn main() {
    let input = include_str!("./input.txt");
    let output = part2(input);
    dbg!(output);
}

fn part2(input: &str) -> i32 {
    let mut lines = input.lines();
    let mut sum = 0;
    while let Some(line) = lines.next() {
        let mut min_red = 0;
        let mut min_green = 0;
        let mut min_blue = 0;
        for part in line.split(";").into_iter() {
            let stripped_part = part.replace(",", "");
            let vec: Vec<&str> = stripped_part.split(" ").collect();
            if let Some(red_index) = vec.iter().position(|&r| r == "red") {
                min_red = max(min_red, vec.get(red_index - 1).unwrap().parse().unwrap());
            }
            if let Some(green_index) = vec.iter().position(|&r| r == "green") {
                min_green = max(
                    min_green,
                    vec.get(green_index - 1).unwrap().parse().unwrap(),
                );
            }
            if let Some(blue_index) = vec.iter().position(|&r| r == "blue") {
                min_blue = max(min_blue, vec.get(blue_index - 1).unwrap().parse().unwrap());
            }
        }
        sum += min_blue * min_green * min_red;
    }
    sum
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part2(
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
        ",
        );
        assert_eq!(result, 2286);
    }
}
