use regex::Regex;

fn main() {
    let input = include_str!("../../../../advent-of-code-input/2024/day-03.txt");
    let output = part1(input);
    println!("{output}");
}

fn part1(input: &str) -> u32 {
    let mut sum = 0;
    let re = Regex::new(r"mul\(\d+,\d+\)").unwrap();
    for mat in re.find_iter(input) {
        let mul = execute_mul(mat.as_str());
        println!("Resulting multiplication: {mul}");
        sum  += execute_mul(mat.as_str());
    }

    return sum;
}

fn execute_mul(instruction: &str) -> u32 {
    let stripped_instruction = instruction.replace("mul(", "").replace(")", "");
    let mut numbers = stripped_instruction.split(",");
    let left: u32 = numbers.next().unwrap().parse().unwrap();
    let right: u32 = numbers.next().unwrap().parse().unwrap();
    println!("Found left value {left} and right value {right}");
    return left * right;
}

#[cfg(test)]

mod test{
    use super::*;

    #[test]
    fn it_works() {
        let output = part1("xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))");
        assert_eq!(output, 161);
    }
}