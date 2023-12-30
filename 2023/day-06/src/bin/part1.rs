use regex::Regex;
fn main() {
    let input = include_str!("./input.txt");
    let output = part1(input);
    dbg!(output);
}

struct Race {
    time: u32,
    current_record: u32
}

fn part1(input: &str) -> i32 {
    let re = Regex::new(r"\d+").unwrap();
    let line_1 = input.split("\n").collect::<Vec<&str>>()[0];
    let line_2 = input.split("\n").collect::<Vec<&str>>()[1];
    let times = re.find_iter(line_1).map(|x| x.as_str().parse::<u32>().unwrap()).collect::<Vec<_>>();

    let records = re.find_iter(line_2).map(|x| x.as_str().parse::<u32>().unwrap()).collect::<Vec<_>>();

    dbg!(times);
    dbg!(records);

    todo!()
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part1(
            "Time:      7  15   30
            Distance:  9  40  200",
        );
        assert_eq!(result, 288);
    }
}
