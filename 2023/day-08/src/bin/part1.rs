use std::collections::HashMap;

fn main() {
    let input = include_str!("./input.txt");
    let output = part1(input);
    dbg!(output);
}


fn part1(input: &str) -> usize {
    let lines = input.lines().into_iter().collect::<Vec<&str>>();
    let instructions = lines[0].chars().collect::<Vec<char>>();
    let mut map: HashMap<&str, (String, String)> = HashMap::new();
    for line in lines[2..].iter() {
        let mut parts = line.split("=");
        let key = parts.next().unwrap().trim();
        let value = parts.next().unwrap().trim().split(",").collect::<Vec<&str>>();
        let value_left = value[0].replace("(", "");
        let value_right = value[1].trim().replace(")", "").to_string();
        map.insert(key, (value_left, value_right));
    }
    let mut steps = 0;
    let mut current = "AAA";
    while current != "ZZZ" {
        let instruction = instructions[steps % instructions.len()];
        if instruction == 'R' {
            current = map.get(current).unwrap().1.as_str();
        } else {
            current = map.get(current).unwrap().0.as_str();
        }
        steps += 1;
    }
    steps
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part1(
            "RL

            AAA = (BBB, CCC)
            BBB = (DDD, EEE)
            CCC = (ZZZ, GGG)
            DDD = (DDD, DDD)
            EEE = (EEE, EEE)
            GGG = (GGG, GGG)
            ZZZ = (ZZZ, ZZZ)",
        );
        assert_eq!(result, 2);
    }

    #[test]
    fn it_works_2() {
        let result = part1(
            "LLR

            AAA = (BBB, BBB)
            BBB = (AAA, ZZZ)
            ZZZ = (ZZZ, ZZZ)",
        );
        assert_eq!(result, 6);
    }
}
