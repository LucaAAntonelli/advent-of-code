use std::cmp::max;
fn main() {
    let input = include_str!("../../../../advent-of-code-input/2023/day-02.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> i32 {
    let mut lines = input.lines();
    let bag_red = 12;
    let bag_green = 13;
    let bag_blue = 14;
    let mut sum = 0;
    let mut game = 0;
    while let Some(line) = lines.next() {
        // Split string on semicolons
        game += 1;
        let mut game_red = 0;
        let mut game_green = 0;
        let mut game_blue = 0;
        for part in line.split(";").into_iter() {
            let stripped_part = part.replace(",", "");
            let vec: Vec<&str> = stripped_part.split(" ").collect();
            if let Some(red_index) = vec.iter().position(|&r| r == "red") {
                game_red = max(game_red, vec.get(red_index - 1).unwrap().parse().unwrap());
            }
            if let Some(green_index) = vec.iter().position(|&r| r == "green") {
                game_green = max(
                    game_green,
                    vec.get(green_index - 1).unwrap().parse().unwrap(),
                );
            }
            if let Some(blue_index) = vec.iter().position(|&r| r == "blue") {
                game_blue = max(game_blue, vec.get(blue_index - 1).unwrap().parse().unwrap());
            }
        }
        if game_red <= bag_red && game_blue <= bag_blue && game_green <= bag_green {
            sum += game;
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
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
            Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
            Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
            Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
            Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        );
        assert_eq!(result, 8);
    }
}
