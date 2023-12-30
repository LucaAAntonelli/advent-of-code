use std::ops::{Range, RangeInclusive};

use regex::Regex;
use itertools::izip;
fn main() {
    let input = include_str!("./input.txt");
    let output = part1(input);
    dbg!(output);
}

#[derive(Debug)]
struct Race {
    time: u32,
    current_record: u32
}

impl Race {
    fn possible_push_times(&self) -> RangeInclusive<u32> {
        0..=self.time
    }

    
}

fn calculate_distance(button_press_time: u32, total_time: u32) -> u32 {
    (total_time - button_press_time) * button_press_time
}

fn part1(input: &str) -> i32 {
    let mut result = 1;
    let re = Regex::new(r"\d+").unwrap();
    let line_1 = input.split("\n").collect::<Vec<&str>>()[0];
    let line_2 = input.split("\n").collect::<Vec<&str>>()[1];
    let times = re.find_iter(line_1).map(|x| x.as_str().parse::<u32>().unwrap()).collect::<Vec<_>>();

    let records = re.find_iter(line_2).map(|x| x.as_str().parse::<u32>().unwrap()).collect::<Vec<_>>();
    let mut races: Vec<Race> = vec![];
    for (time, record) in izip!(times, records) {
        races.push(Race { time, current_record: record })
    }
    for race in races {
        let mut race_result = 0;
        for button_press_time in race.possible_push_times() {
            let distance = calculate_distance(button_press_time, race.time);
            if distance > race.current_record {
                race_result += 1;
            }
        }
        result *= race_result;
    }

    result
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
