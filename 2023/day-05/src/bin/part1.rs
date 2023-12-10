use std::collections::HashMap;

fn main() {
    let input = include_str!("./input.txt");
    let output = part1(input);
    dbg!(output);
}

struct Mapper {
    seeds: Vec<i32>,
    seed_to_soil: HashMap<i32, i32>,
    soil_to_fertilizer: HashMap<i32, i32>,
    fertilizer_to_water: HashMap<i32, i32>,
    water_to_light: HashMap<i32, i32>,
    light_to_temperature: HashMap<i32, i32>,
    temperature_to_humidity: HashMap<i32, i32>,
    humidity_to_location: HashMap<i32, i32>,
}

impl Default for Mapper {
    fn default() -> Self {
        Self {
            seeds: vec![0],
            seed_to_soil: HashMap::from([(0, 0)]),
            soil_to_fertilizer: HashMap::from([(0, 0)]),
            fertilizer_to_water: HashMap::from([(0, 0)]),
            water_to_light: HashMap::from([(0, 0)]),
            light_to_temperature: HashMap::from([(0, 0)]),
            temperature_to_humidity: HashMap::from([(0, 0)]),
            humidity_to_location: HashMap::from([(0, 0)]),
        }
    }
}

impl Mapper {
    pub fn new(input: &str) -> Self {
        let parts: Vec<&str> = input.split("\n").collect();
        for (idx, part) in parts.iter().enumerate() {
            println!("{idx}: {part}");
        }
        Self::default()
    }
}

fn part1(input: &str) -> i32 {
    let mut result = 0;
    let mapper = Mapper::new(input);

    result
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part1(
            "seeds: 79 14 55 13

        seed-to-soil map:
        50 98 2
        52 50 48
        
        soil-to-fertilizer map:
        0 15 37
        37 52 2
        39 0 15
        
        fertilizer-to-water map:
        49 53 8
        0 11 42
        42 0 7
        57 7 4
        
        water-to-light map:
        88 18 7
        18 25 70
        
        light-to-temperature map:
        45 77 23
        81 45 19
        68 64 13
        
        temperature-to-humidity map:
        0 69 1
        1 0 69
        
        humidity-to-location map:
        60 56 37
        56 93 4
        ",
        );
        assert_eq!(result, 35);
    }
}
