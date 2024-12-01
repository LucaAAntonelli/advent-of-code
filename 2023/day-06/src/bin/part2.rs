use std::ops::RangeInclusive;

use regex::Regex;
use itertools::izip;
fn main() {
    let input = include_str!("../../../../advent-of-code-input/2023/day-06.txt");
    let output = part2(input);
    dbg!(output);
}

#[derive(Debug)]
struct Race {
    time: usize,
    current_record: usize
}

impl Race {
    fn possible_push_times(&self) -> RangeInclusive<usize> {
        0..=self.time
    }

    
}

fn calculate_distance(button_press_time: usize, total_time: usize) -> usize {
    (total_time - button_press_time) * button_press_time
}

fn part2(input: &str) -> usize {
    let mut result = 1;
    let re = Regex::new(r"\d+").unwrap();
    let line_1 = input.split("\n").collect::<Vec<&str>>()[0].replace(" ", "");
    let line_2 = input.split("\n").collect::<Vec<&str>>()[1].replace(" ", "");
    let times = re.find_iter(&line_1).map(|x| x.as_str().parse::<usize>().unwrap()).collect::<Vec<_>>();

    let records = re.find_iter(&line_2).map(|x| x.as_str().parse::<usize>().unwrap()).collect::<Vec<_>>();
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
        let result = part2(
            "Time:      7  15   30
            Distance:  9  40  200",
        );
        assert_eq!(result, 71503);
    }
}
