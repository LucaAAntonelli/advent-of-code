fn main() {
    let input = include_str!("../../../../advent-of-code-input/2024/day-02.txt");
    let output = part2(input);
    println!("{output}");
}

fn part2(input: &str) -> u32 {
    let mut num_save_reports: u32 = 0;
    for row in input.split("\n") {
        let vector: Vec<u32> = row.split_whitespace().filter_map(|c| c.parse::<u32>().ok()).collect();
        if is_save(&vector) {
            println!("Safe");
            num_save_reports += 1;
        } else if is_save_with_removal(&vector){
            // Check if removing any single element would make the vector safe
            println!("Safe after removal");
            num_save_reports += 1;
        } else {
            println!("Unsafe");
        }
    }
    num_save_reports
}

fn is_save_with_removal(vector: &Vec<u32>) -> bool {
    for i in 0..vector.len() {
        let mut vector_removed = vector.clone();
        vector_removed.remove(i);
        if is_save(&vector_removed) {
            return true;
        }
    }
    false
}

fn is_save(vector: &Vec<u32>) -> bool {
    if vector.is_sorted_by(|a, b| a <= b) || vector.is_sorted_by(|a, b| a >= b) {
        vector.windows(2).all(|w| w[0].abs_diff(w[1]) < 4 && w[0].abs_diff(w[1]) > 0)
    } else {
        false
    }
}


#[cfg(test)]

mod test {
    use super::*;

    #[test]
    fn it_works() {
        let result = part2("7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9");
        assert_eq!(result, 4)
    }
}